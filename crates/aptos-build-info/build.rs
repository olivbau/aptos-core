// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

fn main() -> shadow_rs::SdResult<()> {
    // CARGO_CFG env vars don't make it to the program at runtime, so we
    // propagate it here by adding a new env var via this cargo directive.
    println!(
        "cargo:rustc-env=USING_TOKIO_UNSTABLE={}",
        std::env::var("CARGO_CFG_TOKIO_UNSTABLE").is_ok()
    );
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    shadow_rs::new()
}
