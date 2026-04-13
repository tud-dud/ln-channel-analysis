use std::cell::Cell;
use std::collections::HashMap;
thread_local! {
    pub static API_CALLS: Cell<u64> = const { Cell::new(0) };
}

use serde::Serialize;

#[derive(Debug, Default, Serialize, PartialEq)]
pub(crate) struct Analysis {
    pub(crate) total_num_txs: usize,
    pub(crate) min_secs: u64,
    pub(crate) max_secs: u64,
    pub(crate) mean_secs: f64,
    pub(crate) median_secs: f64,
    // stores the number of blocks with n many funding txs
    // key = num_txs, value = blocks
    pub(crate) num_txs_same_block: HashMap<usize, usize>,
}
