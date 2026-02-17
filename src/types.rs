use csv::Reader;
use log::info;
use std::{collections::HashSet, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct Channel {
    pub(crate) funding_txid: String,
    pub(crate) output_index: usize,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_csv_to_channels() {
        let path = PathBuf::from("test_data/toy_chanpoints.csv");
        let actual = Channel::read_from_file(&path);
        assert_eq!(actual.len(), 4);
    }
}
