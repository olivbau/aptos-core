// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::HashValue;
use aptos_infallible::Mutex;
use aptos_logger::{prelude::*, Level, Schema};
use aptos_state_view::StateViewId;
use aptos_types::transaction::Version;
use arc_swap::ArcSwap;
use crossbeam::utils::CachePadded;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::Serialize;
use std::sync::Arc;

pub mod counters;

#[derive(Default)]
struct BufferedLog {
    entries: Vec<(Level, AdapterLogSchema, String)>, // Log level & the message string
    critical_err_cnt: usize,
    // To avoid de-allocations on the critical path, store cleared number.
    skip_cleared: usize,
}

static BUFFERED_LOGS: Lazy<ArcSwap<Vec<CachePadded<Mutex<BufferedLog>>>>> =
    Lazy::new(|| ArcSwap::new(Arc::new(Vec::new())));

#[derive(Schema, Clone)]
pub struct AdapterLogSchema {
    name: LogEntry,

    // only one of the next 3 `Option`s will be set. Unless it is in testing mode
    // in which case nothing will be set.
    // Those values are coming from `StateView::id()` and the info carried by
    // `StateViewId`

    // StateViewId::BlockExecution - typical transaction execution
    block_id: Option<HashValue>,
    // StateViewId::ChunkExecution - state sync
    first_version: Option<Version>,
    // StateViewId::TransactionValidation - validation
    base_version: Option<Version>,

    // transaction position in the list of transactions in the block,
    // 0 if the transaction is not part of a block (i.e. validation).
    txn_idx: usize,
}

impl AdapterLogSchema {
    pub fn new(view_id: StateViewId, txn_idx: usize) -> Self {
        match view_id {
            StateViewId::BlockExecution { block_id } => Self {
                name: LogEntry::Execution,
                block_id: Some(block_id),
                first_version: None,
                base_version: None,
                txn_idx,
            },
            StateViewId::ChunkExecution { first_version } => Self {
                name: LogEntry::Execution,
                block_id: None,
                first_version: Some(first_version),
                base_version: None,
                txn_idx,
            },
            StateViewId::TransactionValidation { base_version } => Self {
                name: LogEntry::Validation,
                block_id: None,
                first_version: None,
                base_version: Some(base_version),
                txn_idx,
            },
            StateViewId::Miscellaneous => Self {
                name: LogEntry::Miscellaneous,
                block_id: None,
                first_version: None,
                base_version: None,
                txn_idx,
            },
        }
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogEntry {
    Execution,
    Validation,
    Miscellaneous, // usually testing
}

#[macro_export]
macro_rules! buffer_log {
    ($level:expr, $($args:tt)+) => {
        if enabled!($level) {
            buffer_log($level, $($args)+);
        }
    };
}

// Logs directly (no buffering).
fn log_message(level: Level, context: &AdapterLogSchema, message: &String) {
    match level {
        Level::Error => error!(*context, "{}", message),
        Level::Warn => warn!(*context, "{}", message),
        Level::Info => info!(*context, "{}", message),
        Level::Debug => debug!(*context, "{}", message),
        Level::Trace => trace!(*context, "{}", message),
    }
}

/// Adds a message at a specified logging level and given context (that includes txn index)
/// to the ongoing buffer.
/// Note: flush_logs should be called earlier with the appropriate size to make sure
/// underlying BUFFERED_LOGS storage is indexed sufficiently.
pub fn buffer_log(level: Level, context: AdapterLogSchema, message: String, critical: bool) {
    let txn_idx = context.txn_idx;
    let logs = BUFFERED_LOGS.load();
    let mut log = logs[txn_idx].lock();

    // Be defensive if e.g. buffer_log is called before initialization. Can pass the log
    // through but also log a critical error.
    if txn_idx >= logs.len() {
        // Directly log the message.
        log_message(level, &context, &message);
        error!(
            "buffer_log at idx = {}, but Buffer len = {}",
            txn_idx,
            logs.len()
        );
        counters::CRITICAL_ERRORS.inc();
    }

    log.entries.push((level, context, message));
    if critical {
        log.critical_err_cnt += 1;
    }
}

/// Clears the buffered log for a transaction without logging anything. Useful e.g. when a
/// speculative transaction execution is aborted by parallel execution (failed validation).
pub fn clear_log(txn_idx: usize) {
    let logs = BUFFERED_LOGS.load();
    let mut log = logs[txn_idx].lock();

    // Logically clear the buffered logs.
    log.skip_cleared = log.entries.len();
    log.critical_err_cnt = 0;
}

/// Useful for e.g. module r/w fallback to sequential execution, as in this case we would
/// like to discard all logs coming from the attempted parallel execution.
pub fn clear_logs() {
    // TODO: Could parallelize if needed. Currently does logical clear only.
    for i in 0..BUFFERED_LOGS.load().len() {
        clear_log(i);
    }
}

/// Resizes the log buffer, as needed (if not large enough) to the new provided size.
pub fn resize_log_buffer(num_txns: usize) {
    if num_txns > BUFFERED_LOGS.load().len() {
        swap_new_buffer(num_txns);
    }
}

fn swap_new_buffer(num_txns: usize) -> Arc<Vec<CachePadded<Mutex<BufferedLog>>>> {
    let swap_to = (0..num_txns)
        .map(|_| CachePadded::new(Mutex::new(BufferedLog::default())))
        .collect();
    BUFFERED_LOGS.swap(Arc::new(swap_to))
}

/// Records all the buffered logs and clears the buffer. The clearing happens
/// synchronously (because the next block may need to start executing and buffering
/// logs), however the flushing happens asynchronously in the global rayon pool.
pub fn flush_logs() {
    // TODO: if this ends up slow, we can re-use the BufferedLog entries.
    let swapped = swap_new_buffer(BUFFERED_LOGS.load().len());

    rayon::spawn(move || {
        (**swapped)
            .par_iter()
            .with_min_len(25)
            .for_each(|log_mutex| {
                let buffered_log = log_mutex.lock();

                for _ in 0..buffered_log.critical_err_cnt {
                    counters::CRITICAL_ERRORS.inc();
                }

                for (level, context, message) in
                    buffered_log.entries.iter().skip(buffered_log.skip_cleared)
                {
                    log_message(*level, context, message);
                }
            });
    });
}
