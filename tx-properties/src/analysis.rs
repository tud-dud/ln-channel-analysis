use std::{thread, time::Duration};

use log::{debug, error, trace};

use crate::types::{Analysis, MAX_SATOSHIS, MAX_SATOSHIS_WUMBO};
use common::Tx;

pub(crate) fn analyse_txs(txs: &[Tx]) -> Analysis {
    let mut analysis = Analysis {
        total_num_txs: txs.len(),
        ..Default::default()
    };
    for tx in txs {
        let num_outputs = tx.vout.len();
        debug!("Analysing properties of tx {}", tx.txid);
        let num_p2tr_outputs = tx
            .vout
            .iter()
            .filter(|o| is_p2tr(&o.scriptpubkey_type))
            .count();
        let p2wsh_output_addrs: Vec<_> = tx
            .vout
            .iter()
            .filter(|o| is_p2wsh(&o.scriptpubkey_type))
            .map(|o| o.scriptpubkey_address.clone())
            .collect();
        if at_least_one_p2wsh_appears_max_once(&p2wsh_output_addrs) {
            analysis.original.num_p2wsh_output_address_appeared_once += 1;
            analysis.updated.num_p2wsh_output_address_appeared_once += 1;
        }
        let num_p2wsh_outputs = p2wsh_output_addrs.len();
        let num_p2wsh_outputs_below_16m = tx
            .vout
            .iter()
            .filter(|o| is_p2wsh(&o.scriptpubkey_type) && o.value <= MAX_SATOSHIS)
            .count();
        let funded_p2sh = tx.vin.iter().any(|i| is_p2sh(&i.prevout.scriptpubkey_type));
        let funded_p2tr = tx.vin.iter().any(|i| is_p2tr(&i.prevout.scriptpubkey_type));
        let funded_p2wpkh = tx
            .vin
            .iter()
            .any(|i| is_p2wpkh(&i.prevout.scriptpubkey_type));
        if num_p2wsh_outputs > 0 {
            analysis.original.num_at_least_one_p2wsh_output += 1;
            if num_p2wsh_outputs == 1 {
                analysis.original.num_single_p2wsh_output += 1;
            }
        }
        if num_outputs <= 2 {
            analysis.original.num_max_two_outputs += 1;
            analysis
                .updated
                .num_either_at_most_two_or_one_p2tr_and_more_than_two_p2wsh_output_address += 1;
        } else if num_outputs > 2 && num_p2wsh_outputs == num_outputs - 1 && num_p2tr_outputs == 1 {
            analysis
                .updated
                .num_either_at_most_two_or_one_p2tr_and_more_than_two_p2wsh_output_address += 1;
        }
        if num_p2wsh_outputs_below_16m > 0 {
            analysis.original.num_p2wsh_output_below_16m += 1;
        }
        if funded_p2sh || funded_p2wpkh {
            analysis.original.num_funded_by_p2sh_or_p2wpkh_address += 1;
        }
        analysis
            .updated
            .num_outputs_num_txs
            .entry(num_outputs)
            .and_modify(|n| *n += 1)
            .or_insert(1);
        if (funded_p2tr || funded_p2wpkh) && !funded_p2sh {
            analysis.updated.num_funded_by_p2tr_or_p2wpkh_address += 1;
        }
        if num_p2tr_outputs > 0 {
            analysis.updated.num_at_least_one_p2tr_output += 1;
            if num_p2tr_outputs == 1 {
                analysis.updated.num_single_p2tr_output += 1;
            }
        }
        if num_p2tr_outputs + num_p2wsh_outputs == num_outputs {
            analysis.updated.num_funding_output_p2tr_or_p2wsh_address += 1;
        }
        let num_p2wsh_outputs_below_10btc = tx
            .vout
            .iter()
            .filter(|o| is_p2wsh(&o.scriptpubkey_type) && o.value <= MAX_SATOSHIS_WUMBO)
            .count();
        if num_p2wsh_outputs_below_10btc > 0 {
            analysis.updated.num_p2wsh_output_below_10btc += 1;
        }
    }

    analysis
}

fn is_p2wsh(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2wsh")
}

fn is_p2tr(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2tr")
}

fn is_p2sh(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2sh")
}

fn is_p2wpkh(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2wpkh")
}

// true if at least one of the addresses appeared at most once as both an input and output in the
// blockchain
fn at_least_one_p2wsh_appears_max_once(addrs: &[String]) -> bool {
    let client = reqwest::blocking::Client::new();
    for addr in addrs.iter() {
        trace!("Querying API for {}", addr);
        match client
            .get(format!("https://blockstream.info/api/address/{}/txs", addr))
            .send()
        {
            Ok(get) => match get.text() {
                Ok(text) => {
                    if let Some(txs) = Tx::from_json_str_to_vec(&text) {
                        // check length
                        if txs.len() <= 2 {
                            // once as input
                            // once as output
                            let mut seen_as_input = 0;
                            let mut seen_as_output = 0;
                            for tx in txs {
                                for vout in tx.vout {
                                    if vout.scriptpubkey_address == *addr {
                                        seen_as_output += 1;
                                    }
                                }
                                for vin in tx.vin {
                                    if vin.prevout.scriptpubkey_address == *addr {
                                        seen_as_input += 1;
                                    }
                                }
                            }
                            if seen_as_input + seen_as_output <= 2 {
                                return true;
                            }
                        }
                    }
                }
                Err(e) => error!("Error getting text from response: {e}"),
            },
            Err(e) => error!("API get failed: {e}"),
        }
        // avoid rate limit
        thread::sleep(Duration::from_millis(250));
    }
    false
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    fn tx_json() -> String {
        r#"
            {
          "txid": "0417db532eae86973a3aba5293c8682ceee6f06c85535e122decce8a5fba663a",
          "version": 2,
          "locktime": 0,
          "vin": [
            {
              "txid": "2eb7d6c17620e99f5eb77761df86a49416fd7c2b128d4a73e78d421bc52f9264",
              "vout": 0,
              "prevout": {
                "scriptpubkey": "512033fa9b7529afcd2f0f4a03c8f94c98a176efc33a373ae1ef7515babf5390c99d",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 33fa9b7529afcd2f0f4a03c8f94c98a176efc33a373ae1ef7515babf5390c99d",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1px0afkaff4lxj7r62q0y0jnyc59mwlse6xuawrmm4zkat75usexwsy4f7uh",
                "value": 282342
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "ba042e1cb5da948a19f1996c7d3ce1d802d1650846e5f9a1a22a9e8916495264b2639aa184113f5c978c7809bc18fa547a0689dd5913ffa505bc95d51b09484f"
              ],
              "is_coinbase": false,
              "sequence": 0
            },
            {
              "txid": "6fee27013a0537aadccf10b2456fba2d805eb3ec09ffa796ab414e8d6d5f0b3b",
              "vout": 0,
              "prevout": {
                "scriptpubkey": "512002831fe69db3afb41dc50cf267652f0cbf4dd5ae1b8542055da6f6dc2528a175",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 02831fe69db3afb41dc50cf267652f0cbf4dd5ae1b8542055da6f6dc2528a175",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1pq2p3le5akwhmg8w9pnexwef0pjl5m4dwrwz5yp2a5mmdcffg596sakc2zf",
                "value": 236503
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "71765c78098b43c9909b9d512b389d0d5c4aa5f0b0acaa96cdf487fea9deb35c612f33e1b4395cea7cceb4b12c2fc079e35ea03a1b9452bdbae2a9ce92822598"
              ],
              "is_coinbase": false,
              "sequence": 0
            },
            {
              "txid": "765d063b2c8b5b0bcc5ffa037f8788bc90e38f1ce97ccb5fd4a576e032b68be7",
              "vout": 0,
              "prevout": {
                "scriptpubkey": "51203f22602918bd5ce7da8c53c6fafb59a80642bbbee50baa397429ec746632755c",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 3f22602918bd5ce7da8c53c6fafb59a80642bbbee50baa397429ec746632755c",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1p8u3xq2gch4ww0k5v20r0476e4qry9wa7u5965wt598k8ge3jw4wq5kpx4m",
                "value": 363972
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "d60d92f6cb23776ea7af88cbe38328f011d9d51239ebbbaaefb6fdc02e20168e0ee3a49662ffa17e9ec61a65e056427dcabe466d0d0bef0dd6f0c107fa9fa94e"
              ],
              "is_coinbase": false,
              "sequence": 0
            },
            {
              "txid": "9b66bbfcefa30e711055cc4c37c381a80d9e1bb817108267d320905c66e81e0b",
              "vout": 1,
              "prevout": {
                "scriptpubkey": "51209e46955174160562865508347019b57a041cce38df99b355eabf9bfe07394a42",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 9e46955174160562865508347019b57a041cce38df99b355eabf9bfe07394a42",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1pnerf25t5zczk9pj4pq68qxd40gzpen3cm7vmx402h7dlupeeffpqty7d37",
                "value": 313220
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "e6b4d0104ee3b7606355d23e657f2b2fd6279995744f30b0308f1d12d177e457b6846efb8d4fe5e2d6b937e5e2b8a69d388aa8eb7084b5d68f218026bf23bd91"
              ],
              "is_coinbase": false,
              "sequence": 0
            },
            {
              "txid": "da56535b80cb0611b95c4a48c1e76218c68b1b101027a9e5a223fea9d3adacfc",
              "vout": 0,
              "prevout": {
                "scriptpubkey": "512095d1224e8029f116055a5bc5281bb9f09329a4a52163455bfcbb68578e61e62e",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 95d1224e8029f116055a5bc5281bb9f09329a4a52163455bfcbb68578e61e62e",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1pjhgjyn5q98c3vp26t0zjsxae7zfjnf99y9352kluhd590rnpuchq60qq4u",
                "value": 234176
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "21343856287852fc89403e15990a94e8f7b56a5d129d726eaa3233fe1de444b00713621763ebd608d08cb228808e87162058ae9d6740e20fe786c428af591f95"
              ],
              "is_coinbase": false,
              "sequence": 0
            }
          ],
          "vout": [
            {
              "scriptpubkey": "5120259508f959496e32414ef447731dd7e510ce98b9cf8aeeb875ae647af0be893c",
              "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 259508f959496e32414ef447731dd7e510ce98b9cf8aeeb875ae647af0be893c",
              "scriptpubkey_type": "v1_p2tr",
              "scriptpubkey_address": "bc1pyk2s372ef9hrys2w73rhx8whu5gvax9ee79wawr44ej84u973y7q8g4auv",
              "value": 207347
            },
            {
              "scriptpubkey": "0020ae5a82618348cb5e3f3a3fdd854e9281277ab571af4da4322b9e1ce3efadf626",
              "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_32 ae5a82618348cb5e3f3a3fdd854e9281277ab571af4da4322b9e1ce3efadf626",
              "scriptpubkey_type": "v0_p2wsh",
              "scriptpubkey_address": "bc1q4edgycvrfr94u0e68lwc2n5jsynh4dt34ax6gv3tncww8mad7cnqrlecxn",
              "value": 1222098
            }
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

    #[test]
    fn analyse_tx() {
        let mut txs = vec![];
        txs.push(Tx::from_json_str(&tx_json()).unwrap());
        txs.push(Tx::from_json_str(&tests::tx_json()).unwrap());
        let anaysis = analyse_txs(&txs);
        assert_eq!(anaysis.total_num_txs, 2);

        assert_eq!(anaysis.original.num_at_least_one_p2wsh_output, 2);
        assert_eq!(anaysis.original.num_max_two_outputs, 2);
        assert_eq!(anaysis.original.num_single_p2wsh_output, 2);
        assert_eq!(anaysis.original.num_p2wsh_output_below_16m, 2);
        assert_eq!(anaysis.original.num_funded_by_p2sh_or_p2wpkh_address, 0);

        assert_eq!(anaysis.updated.num_outputs_num_txs, HashMap::from([(2, 2)]));
        assert_eq!(anaysis.updated.num_at_least_one_p2tr_output, 2);
        assert_eq!(anaysis.updated.num_single_p2tr_output, 2);
        assert_eq!(anaysis.updated.num_funded_by_p2tr_or_p2wpkh_address, 2);
        assert_eq!(
            anaysis
                .updated
                .num_either_at_most_two_or_one_p2tr_and_more_than_two_p2wsh_output_address,
            2
        );
    }
}
