use std::collections::HashMap;

use serde::Serialize;

pub(crate) const MAX_SATOSHIS: u64 = 16777215;

#[derive(Debug, Default, Serialize)]
pub(crate) struct Analysis {
    pub(crate) total_num_txs: usize,
    pub(crate) original: Original,
    pub(crate) updated: Updated,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct Original {
    pub(crate) num_at_least_one_p2wsh_output: usize,
    // i)
    pub(crate) num_max_two_outputs: usize,
    // ii)
    pub(crate) num_single_p2wsh_output: usize,
    // iii)
    pub(crate) num_p2wsh_output_below_16m: usize,
    // v)
    pub(crate) num_funded_by_p2sh_or_p2wpkh_address: usize,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct Updated {
    // most either exactly 2 outputs or > n outputs where multiple consecutive indices are used as
    // channel points
    pub(crate) num_outputs_num_txs: HashMap<usize, usize>,
    // at least one Taproot output
    pub(crate) num_at_least_one_p2tr_output: usize,
    // ii) exactly one Taproot output
    pub(crate) num_single_p2tr_output: usize,
    // v) funding exclusively by P2TR or P2WPKH
    pub(crate) num_funded_by_p2tr_or_p2wpkh_address: usize,
    // all funding outputs as P2WSH or Taproot
    pub(crate) num_funding_output_p2tr_or_p2wsh_address: usize,
    // tx with either n <=2 outputs or n>2 outputs where 1 is P2TR and n-1 P2WSH
    pub(crate) num_either_at_most_two_or_one_p2tr_and_more_than_two_p2wsh_output_address: usize,
}
