use std::collections::HashMap;

use log::debug;

use crate::types::Analysis;
use common::Tx;

pub(crate) fn analyse_frequency(txs: &[Tx]) -> Analysis {
    let mut analysis = Analysis {
        total_num_txs: txs.len(),
        ..Default::default()
    };
    if !txs.is_empty() {
        debug!("Calculating statistics for {} txs", analysis.total_num_txs);
        let mut times: Vec<u64> = txs.iter().map(|t| t.status.block_time).collect();
        times.sort_unstable();
        let pairs: Vec<u64> = times.windows(2).map(|w| w[1] - w[0]).collect();
        analysis.min_secs = *pairs.iter().min().unwrap_or(&u64::MIN);
        analysis.max_secs = *pairs.iter().max().unwrap_or(&u64::MAX);
        let sum: u64 = pairs.iter().sum();
        analysis.mean_secs = sum as f64 / pairs.len() as f64;
        let mut sorted_pairs = pairs.clone();
        sorted_pairs.sort_unstable();
        analysis.median_secs = if !sorted_pairs.len().is_multiple_of(2) {
            sorted_pairs[sorted_pairs.len() / 2] as f64
        } else {
            let mid = sorted_pairs.len() / 2;
            (sorted_pairs[mid - 1] as f64 + sorted_pairs[mid] as f64) / 2.0
        };
        analysis.num_txs_same_block = get_txs_per_blockheight(txs);
    }

    analysis
}

fn get_txs_per_blockheight(txs: &[Tx]) -> HashMap<usize, usize> {
    let mut num_n_txs_same_block: HashMap<usize, usize> = HashMap::new();
    let mut txs_per_blockheight: HashMap<u64, usize> = HashMap::new();
    for tx in txs {
        txs_per_blockheight
            .entry(tx.status.block_height)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    for num_txs in txs_per_blockheight.values() {
        num_n_txs_same_block
            .entry(*num_txs)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    num_n_txs_same_block
}

#[cfg(test)]
mod tests {

    use super::*;

    fn tx_json() -> String {
        r#"
            {
          "txid": "0417db532eae86973a3aba5293c8682ceee6f06c85535e122decce8a5fba663a",
          "version": 2,
          "locktime": 0,
          "vin": [
          ],
          "vout": [
          ],
          "size": 633,
          "weight": 1536,
          "fee": 768,
          "status": {
            "confirmed": true,
            "block_height": 937004,
            "block_hash": "000000000000000000012365801d776174e143288fad3839aa80e7aef0699e62",
            "block_time": 1771296822
          }
        }
        "#
        .to_owned()
    }
    fn tx_json2() -> String {
        r#"
            {
          "txid": "0417db532eae86973a3aba5293c8682ceee6f06c85535e122decce8a5fba663a",
          "version": 2,
          "locktime": 0,
          "vin": [
          ],
          "vout": [
          ],
          "size": 633,
          "weight": 1536,
          "fee": 768,
          "status": {
            "confirmed": true,
            "block_height": 937014,
            "block_hash": "000000000000000000012365801d776174e143288fad3839aa80e7aef0699e62",
            "block_time": 1771315113
          }
        }
        "#
        .to_owned()
    }
    fn tx_json3() -> String {
        r#"
            {
          "txid": "0417db532eae86973a3aba5293c8682ceee6f06c85535e122decce8a5fba663a",
          "version": 2,
          "locktime": 0,
          "vin": [
          ],
          "vout": [
          ],
          "size": 633,
          "weight": 1536,
          "fee": 768,
          "status": {
            "confirmed": true,
            "block_height": 937004,
            "block_hash": "000000000000000000012365801d776174e143288fad3839aa80e7aef0699e62",
            "block_time": 1771315213
          }
        }
        "#
        .to_owned()
    }

    #[test]
    fn analyse_tx_freq() {
        let mut txs = vec![];
        txs.push(Tx::from_json_str(&tx_json()).unwrap());
        txs.push(Tx::from_json_str(&tx_json2()).unwrap());
        txs.push(Tx::from_json_str(&tx_json3()).unwrap());
        let actual = analyse_frequency(&txs);
        let expected = Analysis {
            total_num_txs: 3,
            min_secs: 100,
            max_secs: 18291,
            median_secs: 9195.5,
            mean_secs: 9195.5,
            num_txs_same_block: HashMap::from([(2, 2), (2, 1)]),
        };
        assert_eq!(actual.total_num_txs, expected.total_num_txs);
        assert_eq!(actual.min_secs, expected.min_secs);
        assert_eq!(actual.max_secs, expected.max_secs);
        assert_eq!(actual.median_secs, expected.median_secs);
        assert_eq!(actual.mean_secs, expected.mean_secs);
        for (k, v) in expected.num_txs_same_block {
            assert_eq!(actual.num_txs_same_block[&k], v);
        }
    }
}
