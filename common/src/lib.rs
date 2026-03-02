use csv::Reader;
use log::info;
use serde::Deserialize;
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Channel {
    pub funding_txid: String,
    pub output_index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Tx {
    pub txid: String,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}

#[derive(Debug, Deserialize)]
pub struct Vin {
    #[serde(rename = "prevout")]
    pub prevout: Prevout,
}

#[derive(Debug, Deserialize)]
pub struct Prevout {
    pub scriptpubkey_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Vout {
    pub scriptpubkey_type: String,
    pub value: u64,
}

impl Channel {
    pub fn read_from_file(path: &PathBuf) -> HashSet<Self> {
        let mut channels = HashSet::new();
        if let Ok(mut rdr) = Reader::from_path(path) {
            for result in rdr.deserialize().flatten() {
                channels.insert(result);
            }
        }
        info!("Read {} channel points from CSV", channels.len());
        channels
    }
}

impl Tx {
    pub fn from_json_str(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    pub fn tx_json() -> String {
        r#"
            {
          "txid": "c9208b6b4059f42bc5d073bc79cb192b3b53411127a796a8659b204089167e3f",
          "version": 2,
          "locktime": 0,
          "vin": [
            {
              "txid": "1ec28e0a355480920251105c9c7a4cb06878ec1fc9b4007f9781e503d50bfb94",
              "vout": 1,
              "prevout": {
                "scriptpubkey": "5120819cd9ff3a3af4f667d098f90e43cf8718d4f66bcc10d8994124a8d923ff6e71",
                "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 819cd9ff3a3af4f667d098f90e43cf8718d4f66bcc10d8994124a8d923ff6e71",
                "scriptpubkey_type": "v1_p2tr",
                "scriptpubkey_address": "bc1psxwdnle68t60ve7snrusus70suvdfantesgd3x2pyj5djglldecspa78jd",
                "value": 30015374
              },
              "scriptsig": "",
              "scriptsig_asm": "",
              "witness": [
                "db6192a803dbdbed799365428cbc00faad8d90b9a1726fed26d2f647e85e589b67e93efbd1ad9cdcf8171f428c7cb30d89952dfe9ec6f40f426c7d24c0dce8bd"
              ],
              "is_coinbase": false,
              "sequence": 0
            }
          ],
          "vout": [
            {
              "scriptpubkey": "0020a1f6e6da227869a93c08f270061ec20ecac050880396897e9d82fac8f1b5fb1e",
              "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_32 a1f6e6da227869a93c08f270061ec20ecac050880396897e9d82fac8f1b5fb1e",
              "scriptpubkey_type": "v0_p2wsh",
              "scriptpubkey_address": "bc1q58mwdk3z0p56j0qg7fcqv8kzpm9vq5ygqwtgjl5astav3ud4lv0qaya83u",
              "value": 9820024
            },
            {
              "scriptpubkey": "51200367c7dd86b1e08c6b6d4ae13530fde54fa3a38c7590fd40d531957900680a47",
              "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 0367c7dd86b1e08c6b6d4ae13530fde54fa3a38c7590fd40d531957900680a47",
              "scriptpubkey_type": "v1_p2tr",
              "scriptpubkey_address": "bc1pqdnu0hvxk8sgc6mdftsn2v8au4868guvwkg06sx4xx2hjqrgpfrszdhqtn",
              "value": 20195195
            }
          ],
          "size": 205,
          "weight": 616,
          "fee": 155,
          "status": {
            "confirmed": true,
            "block_height": 937047,
            "block_hash": "000000000000000000011b8f1e0b4b13cb363758ad005958da68488be949a39c",
            "block_time": 1771315113
          }
        }"#.to_owned()
    }

    #[test]
    fn from_csv_to_channels() {
        let path = PathBuf::from("../test_data/chanpoints.csv");
        let actual = Channel::read_from_file(&path);
        assert_eq!(actual.len(), 8);
    }

    #[test]
    fn from_json_to_tx() {
        let tx = tx_json();
        let actual = Tx::from_json_str(&tx).unwrap();
        assert_eq!(actual.vin.len(), 1);
        assert_eq!(actual.vout.len(), 2);
        assert_eq!(actual.vout[0].scriptpubkey_type, "v0_p2wsh");
        assert_eq!(actual.vout[1].scriptpubkey_type, "v1_p2tr");
    }
}
