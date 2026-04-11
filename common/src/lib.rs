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
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct Vin {
    #[serde(rename = "prevout")]
    pub prevout: Prevout,
}

#[derive(Debug, Deserialize)]
pub struct Prevout {
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
}

#[derive(Debug, Deserialize)]
pub struct Vout {
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    // unix timestamp
    pub block_time: u64,
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
    pub fn from_json_str_to_vec(json: &str) -> Option<Vec<Self>> {
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

    pub fn tx_json_vec() -> String {
        r#"
        [
  {
    "txid": "e7359bce157934ef767037be45feebd6103201e76d6edd5bb2c0f0daa98e118a",
    "version": 2,
    "locktime": 938166,
    "vin": [
      {
        "txid": "fada00382935a8703e6d86ce00a1bc2612f6b730474f1868b2c98dceaaba11f5",
        "vout": 0,
        "prevout": {
          "scriptpubkey": "0014293df4f52a09f809fcfcd4603b63956d7b71e643",
          "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 293df4f52a09f809fcfcd4603b63956d7b71e643",
          "scriptpubkey_type": "v0_p2wpkh",
          "scriptpubkey_address": "bc1q9y7lfaf2p8uqnl8u63srkcu4d4ahrejr77atvt",
          "value": 29969
        },
        "scriptsig": "",
        "scriptsig_asm": "",
        "witness": [
          "304402205626bbe2d7271e9faced8f748abc40ccd3c3a9ba5d1f24d8ac4ba64e17cfe8b4022015576dabfe472fea931d40d44edf01215af9fafa16f558b069f1ed6b2ee72cb601",
          "038f2083b1ce8934764ab5233532a6d0f47eaa539e44ff673a2184daefd987ca98"
        ],
        "is_coinbase": false,
        "sequence": 4294967293
      },
      {
        "txid": "1a5a36a16562e7afe1f3306aa12b02e43d582eae9323f864ec3bbb2853fc1d53",
        "vout": 0,
        "prevout": {
          "scriptpubkey": "00140c2a7c91a46d0347a370692e21d1615e7d215515",
          "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 0c2a7c91a46d0347a370692e21d1615e7d215515",
          "scriptpubkey_type": "v0_p2wpkh",
          "scriptpubkey_address": "bc1qps48eydyd5p50gmsdyhzr5tpte7jz4g4zljjgj",
          "value": 98895
        },
        "scriptsig": "",
        "scriptsig_asm": "",
        "witness": [
          "304402201d3a53becd534b5d823445133c80f255d3b3b5218687982e8b3f711416e1a0eb02207d990a4881f18552d6475ef3660a0800fa29be9bd4fcee3e997348512c0a172d01",
          "03c64a2b3aff815b7f2291a1c75c05826089312c5ab469af34540e62aa47c1e73b"
        ],
        "is_coinbase": false,
        "sequence": 4294967293
      },
      {
        "txid": "3bbef682fb46e1f148a70a11d6e25e22e50a295436ffa1db891a3c87df29b6b7",
        "vout": 2,
        "prevout": {
          "scriptpubkey": "0014dba65fc537690fe5a5c86621fec0a81531a9a1cc",
          "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 dba65fc537690fe5a5c86621fec0a81531a9a1cc",
          "scriptpubkey_type": "v0_p2wpkh",
          "scriptpubkey_address": "bc1qmwn9l3fhdy87tfwgvcslas9gz5c6ngwvckek7p",
          "value": 1397162
        },
        "scriptsig": "",
        "scriptsig_asm": "",
        "witness": [
          "3044022072aca32cdb00154c568522b796888096e081930958e40ae1cbc12fa5edec62eb02206646f72922c8778c6d6d07ba0f6f1d9fae9eec805e0cb91fe3e7e9ed38827cea01",
          "023d9a789cc133efed75709a099d6e1429787847f3778407eb582a9a9e23f5c89f"
        ],
        "is_coinbase": false,
        "sequence": 4294967293
      }
    ],
    "vout": [
      {
        "scriptpubkey": "0014fc687a59e8a4884527cf64a77899e959a70ec95b",
        "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 fc687a59e8a4884527cf64a77899e959a70ec95b",
        "scriptpubkey_type": "v0_p2wpkh",
        "scriptpubkey_address": "bc1ql3585k0g5jyy2f70vjnh3x0ftxnsaj2mmtrua9",
        "value": 29968
      },
      {
        "scriptpubkey": "0020b291166038377d47bcc67ee849334fbe5c00db2024fd9a5b7847e6ae3bd05af4",
        "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_32 b291166038377d47bcc67ee849334fbe5c00db2024fd9a5b7847e6ae3bd05af4",
        "scriptpubkey_type": "v0_p2wsh",
        "scriptpubkey_address": "bc1qk2g3vcpcxa7500xx0m5yjv60hewqpkeqyn7e5kmcgln2uw7stt6qwpmtwv",
        "value": 1495767
      }
    ],
    "size": 530,
    "weight": 1151,
    "fee": 291,
    "status": {
      "confirmed": true,
      "block_height": 938167,
      "block_hash": "000000000000000000009d689b51deff726923888433b44ce7425f817218ac53",
      "block_time": 1771961950
    }
  }
]
        "#.to_owned()
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
        assert_eq!(actual.status.block_time, 1771315113);
    }
    #[test]
    fn from_json_to_tx_vec() {
        let tx = tx_json_vec();
        let actual = Tx::from_json_str_to_vec(&tx).unwrap();
        assert_eq!(actual.len(), 1);
        let tx = &actual[0];
        assert_eq!(tx.vin.len(), 3);
        assert_eq!(tx.vout.len(), 2);
        assert_eq!(
            tx.vout[1].scriptpubkey_address,
            "bc1qk2g3vcpcxa7500xx0m5yjv60hewqpkeqyn7e5kmcgln2uw7stt6qwpmtwv"
        );
        assert_eq!(
            tx.vin[2].prevout.scriptpubkey_address,
            "bc1qmwn9l3fhdy87tfwgvcslas9gz5c6ngwvckek7p",
        );
    }
}
