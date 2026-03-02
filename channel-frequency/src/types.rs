use serde::Serialize;

#[derive(Debug, Default, Serialize, PartialEq)]
pub(crate) struct Analysis {
    pub(crate) total_num_txs: usize,
    pub(crate) min_secs: u64,
    pub(crate) max_secs: u64,
    pub(crate) mean_secs: f64,
    pub(crate) median_secs: f64,
}
