// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::account::{
    create::{CreateAccount, DEFAULT_FUNDED_COINS},
    fund::FundAccount,
    list::{ListAccount, ListQuery},
    transfer::{TransferCoins, TransferSummary},
};
use crate::common::init::InitTool;
use crate::common::types::{
    account_address_from_public_key, AccountAddressWrapper, CliError, CliTypedResult,
    EncodingOptions, FaucetOptions, GasOptions, MoveManifestAccountWrapper, MovePackageDir,
    PrivateKeyInputOptions, PromptOptions, RestOptions, RngArgs, TransactionOptions,
    TransactionSummary,
};
use crate::common::utils::write_to_file;
use crate::move_tool::{
    ArgWithType, CompilePackage, InitPackage, MemberId, PublishPackage, RunFunction, TestPackage,
};
use crate::node::{
    AddStake, AnalyzeMode, AnalyzeValidatorPerformance, IncreaseLockup, JoinValidatorSet,
    LeaveValidatorSet, OperatorArgs, RegisterValidatorCandidate, ShowValidatorConfig,
    ShowValidatorSet, ShowValidatorStake, UnlockStake, UpdateValidatorNetworkAddresses,
    ValidatorConfigArgs, WithdrawStake,
};
use crate::CliCommand;
use aptos_crypto::{bls12381, ed25519::Ed25519PrivateKey, x25519, PrivateKey};
use aptos_genesis::config::HostAndPort;
use aptos_keygen::KeyGen;
use aptos_logger::warn;
use aptos_rest_client::{aptos_api_types::MoveType, Transaction};
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_temppath::TempPath;
use aptos_types::{
    on_chain_config::ConsensusScheme, validator_config::ValidatorConfig,
    validator_info::ValidatorInfo,
};
use framework::natives::code::UpgradePolicy;
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf, str::FromStr, time::Duration};
use thiserror::private::PathAsDisplay;
use tokio::time::{sleep, Instant};

pub const INVALID_ACCOUNT: &str = "0xDEADBEEFCAFEBABE";

pub const FIRST_MOVE_FILE: &str = "
module NamedAddress0::store {
    use std::string;
    use aptos_framework::coin::{Self};

    struct CoolCoin has key {}

    public entry fun init(
        account: &signer,
        decimals: u64,
        monitor_supply: bool
    ) {
        let (_, _) = coin::initialize<CoolCoin>(account, string::utf8(b\"CoolCoin\"), string::utf8(b\"COOL\"), decimals, monitor_supply);
        coin::register<CoolCoin>(account);
    }
}";

/// A framework for testing the CLI
pub struct CliTestFramework {
    account_keys: Vec<Ed25519PrivateKey>,
    endpoint: Url,
    faucet_endpoint: Url,
    move_dir: Option<PathBuf>,
}

impl CliTestFramework {
    pub fn local_new(num_accounts: usize) -> CliTestFramework {
        let dummy_url = Url::parse("http://localhost").unwrap();
        let mut framework = CliTestFramework {
            account_keys: Vec::new(),
            endpoint: dummy_url.clone(),
            faucet_endpoint: dummy_url,
            move_dir: None,
        };
        let mut keygen = KeyGen::from_seed([0; 32]);
        for _ in 0..num_accounts {
            framework
                .account_keys
                .push(keygen.generate_ed25519_private_key());
        }
        framework
    }

    pub async fn new(endpoint: Url, faucet_endpoint: Url, num_accounts: usize) -> CliTestFramework {
        let mut framework = CliTestFramework {
            account_keys: Vec::new(),
            endpoint,
            faucet_endpoint,
            move_dir: None,
        };
        let mut keygen = KeyGen::from_seed([0; 32]);

        for _ in 0..num_accounts {
            framework
                .add_cli_account(keygen.generate_ed25519_private_key())
                .await
                .unwrap();
        }

        framework
    }

    pub async fn add_cli_account(
        &mut self,
        private_key: Ed25519PrivateKey,
    ) -> CliTypedResult<usize> {
        let index = self.add_private_key(private_key);

        // Create account if it doesn't exist (and there's a faucet)
        let client = aptos_rest_client::Client::new(self.endpoint.clone());
        let address = self.account_id(index);
        if client.get_account(address).await.is_err() {
            self.fund_account(index, None).await?;
            warn!("Funded account {:?}", address);
        } else {
            warn!("Account {:?} already exists", address);
        }

        Ok(index)
    }

    pub fn add_private_key(&mut self, private_key: Ed25519PrivateKey) -> usize {
        self.account_keys.push(private_key);
        self.account_keys.len() - 1
    }

    pub async fn create_account(
        &self,
        index: usize,
        mint_key: &Ed25519PrivateKey,
    ) -> CliTypedResult<String> {
        CreateAccount {
            txn_options: TransactionOptions {
                private_key_options: PrivateKeyInputOptions::from_private_key(mint_key)?,
                encoding_options: Default::default(),
                profile_options: Default::default(),
                rest_options: self.rest_options(),
                gas_options: Default::default(),
            },
            account: self.account_id(index),
            use_faucet: false,
            faucet_options: Default::default(),
            initial_coins: DEFAULT_FUNDED_COINS,
        }
        .execute()
        .await
    }

    pub async fn create_account_with_faucet(&self, index: usize) -> CliTypedResult<String> {
        CreateAccount {
            txn_options: Default::default(),
            account: self.account_id(index),
            use_faucet: true,
            faucet_options: self.faucet_options(),
            initial_coins: 0,
        }
        .execute()
        .await
    }

    pub async fn fund_account(&self, index: usize, amount: Option<u64>) -> CliTypedResult<String> {
        FundAccount {
            profile_options: Default::default(),
            account: self.account_id(index),
            faucet_options: self.faucet_options(),
            num_coins: amount.unwrap_or(DEFAULT_FUNDED_COINS),
            rest_options: self.rest_options(),
        }
        .execute()
        .await
    }

    pub async fn list_account(&self, index: usize, query: ListQuery) -> CliTypedResult<Vec<Value>> {
        ListAccount {
            rest_options: self.rest_options(),
            profile_options: Default::default(),
            account: Some(self.account_id(index)),
            query,
        }
        .execute()
        .await
    }

    pub async fn transfer_coins(
        &self,
        sender_index: usize,
        receiver_index: usize,
        amount: u64,
        gas_options: Option<GasOptions>,
    ) -> CliTypedResult<TransferSummary> {
        TransferCoins {
            txn_options: self.transaction_options(sender_index, gas_options),
            account: self.account_id(receiver_index),
            amount,
        }
        .execute()
        .await
    }

    pub async fn transfer_invalid_addr(
        &self,
        sender_index: usize,
        amount: u64,
        gas_options: Option<GasOptions>,
    ) -> CliTypedResult<TransferSummary> {
        TransferCoins {
            txn_options: self.transaction_options(sender_index, gas_options),
            account: AccountAddress::from_hex_literal(INVALID_ACCOUNT).unwrap(),
            amount,
        }
        .execute()
        .await
    }

    pub async fn show_validator_config(&self, index: usize) -> CliTypedResult<ValidatorConfig> {
        ShowValidatorConfig {
            rest_options: self.rest_options(),
            profile_options: Default::default(),
            operator_args: self.operator_args(index),
        }
        .execute()
        .await
        .map(|v| to_validator_config(&v))
    }

    pub async fn show_validator_set(&self) -> CliTypedResult<ValidatorSet> {
        ShowValidatorSet {
            rest_options: self.rest_options(),
            profile_options: Default::default(),
        }
        .execute()
        .await
        .map(|v| to_validator_set(&v))
    }

    pub async fn show_validator_stake(&self, index: usize) -> CliTypedResult<Value> {
        ShowValidatorStake {
            rest_options: self.rest_options(),
            profile_options: Default::default(),
            operator_args: self.operator_args(index),
        }
        .execute()
        .await
    }

    pub async fn register_validator_candidate(
        &self,
        index: usize,
        consensus_public_key: bls12381::PublicKey,
        proof_of_possession: bls12381::ProofOfPossession,
        validator_host: HostAndPort,
        validator_network_public_key: x25519::PublicKey,
    ) -> CliTypedResult<Transaction> {
        RegisterValidatorCandidate {
            txn_options: self.transaction_options(index, None),
            validator_config_args: ValidatorConfigArgs {
                validator_config_file: None,
                consensus_public_key: Some(consensus_public_key),
                proof_of_possession: Some(proof_of_possession),
                validator_host: Some(validator_host),
                validator_network_public_key: Some(validator_network_public_key),
                full_node_host: None,
                full_node_network_public_key: None,
            },
        }
        .execute()
        .await
    }

    pub async fn add_stake(&self, index: usize, amount: u64) -> CliTypedResult<Transaction> {
        AddStake {
            txn_options: self.transaction_options(index, None),
            amount,
        }
        .execute()
        .await
    }

    pub async fn unlock_stake(&self, index: usize, amount: u64) -> CliTypedResult<Transaction> {
        UnlockStake {
            txn_options: self.transaction_options(index, None),
            amount,
        }
        .execute()
        .await
    }

    pub async fn withdraw_stake(&self, index: usize, amount: u64) -> CliTypedResult<Transaction> {
        WithdrawStake {
            node_op_options: self.transaction_options(index, None),
            amount,
        }
        .execute()
        .await
    }

    pub async fn increase_lockup(&self, index: usize) -> CliTypedResult<Transaction> {
        IncreaseLockup {
            txn_options: self.transaction_options(index, None),
        }
        .execute()
        .await
    }

    pub async fn join_validator_set(&self, index: usize) -> CliTypedResult<Transaction> {
        JoinValidatorSet {
            txn_options: self.transaction_options(index, None),
            operator_args: self.operator_args(index),
        }
        .execute()
        .await
    }

    pub async fn leave_validator_set(&self, index: usize) -> CliTypedResult<Transaction> {
        LeaveValidatorSet {
            txn_options: self.transaction_options(index, None),
            operator_args: self.operator_args(index),
        }
        .execute()
        .await
    }

    pub async fn update_validator_network_addresses(
        &self,
        index: usize,
        validator_host: HostAndPort,
        validator_network_public_key: x25519::PublicKey,
    ) -> CliTypedResult<Transaction> {
        UpdateValidatorNetworkAddresses {
            txn_options: self.transaction_options(index, None),
            operator_args: self.operator_args(index),
            validator_config_args: ValidatorConfigArgs {
                validator_config_file: None,
                consensus_public_key: None,
                proof_of_possession: None,
                validator_host: Some(validator_host),
                validator_network_public_key: Some(validator_network_public_key),
                full_node_host: None,
                full_node_network_public_key: None,
            },
        }
        .execute()
        .await
    }

    pub async fn analyze_validator_performance(
        &self,
        start_epoch: Option<u64>,
        end_epoch: Option<u64>,
    ) -> CliTypedResult<()> {
        AnalyzeValidatorPerformance {
            start_epoch,
            end_epoch,
            rest_options: self.rest_options(),
            profile_options: Default::default(),
            analyze_mode: AnalyzeMode::All,
        }
        .execute()
        .await
    }

    pub async fn init(&self, private_key: &Ed25519PrivateKey) -> CliTypedResult<()> {
        InitTool {
            rest_url: Some(self.endpoint.clone()),
            faucet_url: Some(self.faucet_endpoint.clone()),
            rng_args: RngArgs::from_seed([0; 32]),
            private_key_options: PrivateKeyInputOptions::from_private_key(private_key)?,
            profile_options: Default::default(),
            prompt_options: PromptOptions::yes(),
            encoding_options: EncodingOptions::default(),
            skip_faucet: false,
        }
        .execute()
        .await
    }

    /// Wait for an account to exist
    pub async fn wait_for_account(&self, index: usize) -> CliTypedResult<Vec<Value>> {
        let mut result = self.list_account(index, ListQuery::Balance).await;
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(10) {
            match result {
                Ok(_) => return result,
                _ => {
                    sleep(Duration::from_millis(500)).await;
                    result = self.list_account(index, ListQuery::Balance).await;
                }
            };
        }

        result
    }

    pub async fn account_balance_now(&self, index: usize) -> CliTypedResult<u64> {
        let result = self.list_account(index, ListQuery::Balance).await?;
        Ok(json_account_to_balance(result.get(0).unwrap()))
    }

    pub async fn assert_account_balance_now(&self, index: usize, expected: u64) {
        let result = self.list_account(index, ListQuery::Balance).await;
        assert!(
            result.is_ok(),
            "Account {} not yet created, {}, last 10 transactions: {}",
            self.account_id(index),
            result.unwrap_err(),
            self.last_n_transactions_details(10).await
        );
        let accounts = result.unwrap();
        let account = accounts.get(0).unwrap();
        let coin = json_account_to_balance(account);
        assert_eq!(
            coin,
            expected,
            "Account {} with state: {:?}, last 10 transactions: {}",
            self.account_id(index),
            account,
            self.last_n_transactions_details(10).await
        );
    }

    async fn last_n_transactions_details(&self, count: u16) -> String {
        let result = aptos_rest_client::Client::new(self.endpoint.clone())
            .get_transactions(None, Some(count))
            .await;
        if let Err(e) = result {
            return format!("Err({:?})", e);
        }
        let lines = result
            .unwrap()
            .inner()
            .iter()
            .map(|t| {
                if let Transaction::UserTransaction(u) = t {
                    format!(
                        " * [{}] {}: sender={}, payload={:?}",
                        t.version().unwrap_or(0),
                        t.vm_status(),
                        u.request.sender,
                        u.request.payload
                    )
                } else {
                    format!(
                        " * [{}] {}: {}",
                        t.version().unwrap_or(0),
                        t.vm_status(),
                        t.type_str()
                    )
                }
            })
            .collect::<Vec<_>>();
        format!("\n{}\n", lines.join("\n"))
    }

    pub fn init_move_dir(&mut self) {
        let move_dir = TempPath::new();
        move_dir
            .create_as_dir()
            .expect("Expected to be able to create move temp dir");
        self.move_dir = Some(move_dir.path().to_path_buf());
    }

    pub fn add_move_files(&self) {
        let move_dir = self.move_dir();
        let sources_dir = move_dir.join("sources");

        let hello_blockchain_contents = include_str!(
            "../../../../aptos-move/move-examples/hello_blockchain/sources/HelloBlockchain.move"
        );
        let source_path = sources_dir.join("HelloBlockchain.move");
        write_to_file(
            source_path.as_path(),
            &source_path.as_display().to_string(),
            hello_blockchain_contents.as_bytes(),
        )
        .unwrap();

        let hello_blockchain_test_contents = include_str!("../../../../aptos-move/move-examples/hello_blockchain/sources/HelloBlockchainTest.move");
        let test_path = sources_dir.join("HelloBlockchainTest.move");
        write_to_file(
            test_path.as_path(),
            &test_path.as_display().to_string(),
            hello_blockchain_test_contents.as_bytes(),
        )
        .unwrap();
    }

    pub fn move_dir(&self) -> PathBuf {
        assert!(self.move_dir.is_some(), "Must have initialized the temp move directory with `CliTestFramework::init_move_dir()` first");
        self.move_dir.as_ref().cloned().unwrap()
    }

    pub async fn init_package(
        &self,
        name: String,
        account_strs: BTreeMap<&str, &str>,
    ) -> CliTypedResult<()> {
        InitPackage {
            name,
            package_dir: Some(self.move_dir()),
            named_addresses: Self::move_manifest_named_addresses(account_strs),
            prompt_options: PromptOptions {
                assume_yes: false,
                assume_no: true,
            },
        }
        .execute()
        .await
    }

    pub async fn compile_package(
        &self,
        account_strs: BTreeMap<&str, &str>,
    ) -> CliTypedResult<Vec<String>> {
        CompilePackage {
            move_options: self.move_options(account_strs),
        }
        .execute()
        .await
    }

    pub async fn test_package(
        &self,
        account_strs: BTreeMap<&str, &str>,
        filter: Option<&str>,
    ) -> CliTypedResult<&'static str> {
        TestPackage {
            move_options: self.move_options(account_strs),
            filter: filter.map(|str| str.to_string()),
        }
        .execute()
        .await
    }

    pub async fn publish_package(
        &self,
        index: usize,
        gas_options: Option<GasOptions>,
        account_strs: BTreeMap<&str, &str>,
        legacy_flow: bool,
        upgrade_policy: Option<UpgradePolicy>,
    ) -> CliTypedResult<TransactionSummary> {
        PublishPackage {
            move_options: self.move_options(account_strs),
            txn_options: self.transaction_options(index, gas_options),
            legacy_flow,
            upgrade_policy,
        }
        .execute()
        .await
    }

    pub async fn run_function(
        &self,
        index: usize,
        gas_options: Option<GasOptions>,
        function_id: MemberId,
        args: Vec<&str>,
        type_args: Vec<&str>,
    ) -> CliTypedResult<TransactionSummary> {
        let mut parsed_args = Vec::new();
        for arg in args {
            parsed_args.push(
                ArgWithType::from_str(arg)
                    .map_err(|err| CliError::UnexpectedError(err.to_string()))?,
            )
        }

        let mut parsed_type_args = Vec::new();
        for arg in type_args {
            parsed_type_args.push(
                MoveType::from_str(arg)
                    .map_err(|err| CliError::UnexpectedError(err.to_string()))?,
            )
        }

        RunFunction {
            txn_options: self.transaction_options(index, gas_options),
            function_id,
            args: parsed_args,
            type_args: parsed_type_args,
        }
        .execute()
        .await
    }

    pub fn move_options(&self, account_strs: BTreeMap<&str, &str>) -> MovePackageDir {
        MovePackageDir {
            package_dir: Some(self.move_dir()),
            output_dir: None,
            named_addresses: Self::named_addresses(account_strs),
        }
    }

    pub fn move_manifest_named_addresses(
        account_strs: BTreeMap<&str, &str>,
    ) -> BTreeMap<String, MoveManifestAccountWrapper> {
        account_strs
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    MoveManifestAccountWrapper::from_str(value).unwrap(),
                )
            })
            .collect()
    }

    pub fn named_addresses(
        account_strs: BTreeMap<&str, &str>,
    ) -> BTreeMap<String, AccountAddressWrapper> {
        account_strs
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    AccountAddressWrapper::from_str(value).unwrap(),
                )
            })
            .collect()
    }

    pub fn rest_options(&self) -> RestOptions {
        RestOptions::new(Some(self.endpoint.clone()))
    }

    pub fn faucet_options(&self) -> FaucetOptions {
        FaucetOptions::new(Some(self.faucet_endpoint.clone()))
    }

    fn transaction_options(
        &self,
        index: usize,
        gas_options: Option<GasOptions>,
    ) -> TransactionOptions {
        TransactionOptions {
            private_key_options: PrivateKeyInputOptions::from_private_key(self.private_key(index))
                .unwrap(),
            rest_options: self.rest_options(),
            gas_options: gas_options.unwrap_or_default(),
            ..Default::default()
        }
    }

    fn operator_args(&self, index: usize) -> OperatorArgs {
        OperatorArgs {
            pool_address: Some(self.account_id(index)),
        }
    }

    pub fn private_key(&self, index: usize) -> &Ed25519PrivateKey {
        self.account_keys.get(index).unwrap()
    }

    pub fn account_id(&self, index: usize) -> AccountAddress {
        let private_key = self.private_key(index);
        account_address_from_public_key(&private_key.public_key())
    }
}

// ValidatorConfig/ValidatorSet doesn't match Move ValidatorSet struct,
// and json is serialized with different types from both, so hardcoding deserialization.

fn str_to_vec(value: &serde_json::Value) -> Vec<u8> {
    let str = value.as_str().unwrap();
    (&*hex::decode(&str[2..str.len()]).unwrap()).to_vec()
}

fn to_validator_config(value: &serde_json::Value) -> ValidatorConfig {
    ValidatorConfig {
        consensus_public_key: serde_json::from_value(
            value.get("consensus_pubkey").unwrap().clone(),
        )
        .unwrap(),
        validator_network_addresses: str_to_vec(value.get("network_addresses").unwrap()),
        fullnode_network_addresses: str_to_vec(value.get("fullnode_addresses").unwrap()),
        validator_index: u64::from_str(value.get("validator_index").unwrap().as_str().unwrap())
            .unwrap(),
    }
}

fn to_validator_info_vec(value: &serde_json::Value) -> Vec<ValidatorInfo> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|value| {
            let account_addr =
                AccountAddress::from_hex_literal(value.get("addr").unwrap().as_str().unwrap())
                    .unwrap();
            ValidatorInfo::new(
                account_addr,
                u64::from_str(value.get("voting_power").unwrap().as_str().unwrap()).unwrap(),
                to_validator_config(value.get("config").unwrap()),
            )
        })
        .collect()
}

// Original ValidatorSet has private fields, to make sure invariants are kept,
// so creating a new one for testing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatorSet {
    pub consensus_scheme: ConsensusScheme,
    pub active_validators: Vec<ValidatorInfo>,
    pub pending_inactive: Vec<ValidatorInfo>,
    pub pending_active: Vec<ValidatorInfo>,
}

fn to_validator_set(value: &serde_json::Value) -> ValidatorSet {
    ValidatorSet {
        consensus_scheme: match value.get("consensus_scheme").unwrap().as_u64().unwrap() {
            0u64 => ConsensusScheme::BLS12381,
            _ => panic!(),
        },
        active_validators: to_validator_info_vec(value.get("active_validators").unwrap()),
        pending_inactive: to_validator_info_vec(value.get("pending_inactive").unwrap()),
        pending_active: to_validator_info_vec(value.get("pending_active").unwrap()),
    }
}

fn json_account_to_balance(value: &Value) -> u64 {
    u64::from_str(
        value
            .as_object()
            .unwrap()
            .get("coin")
            .unwrap()
            .as_object()
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap()
}