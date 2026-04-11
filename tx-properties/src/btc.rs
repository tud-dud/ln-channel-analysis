use rand::prelude::*;
use rand::rngs::StdRng;
use reqwest::blocking::Client;
use std::{
    error::Error,
    fs::File,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use common::{Block, Channel, Tx};
use log::{LevelFilter, debug, error, info, warn};
use types::Analysis;

mod analysis;
mod types;

#[derive(Debug, Parser)]
struct Opt {
    /// Path to the CSV with the channel points
    #[arg(long = "input", short = 'i')]
    ln_chanpoints: PathBuf,
    #[arg(long = "log", short = 'l', default_value = "info")]
    log_level: LevelFilter,
    /// Path to directory in which the results will be stored
    #[arg(long = "out", short = 'o', default_value = "./results")]
    output_path: String,
    /// Block height to start at
    #[arg(long = "start", short = 's')]
    start: usize,
    /// Block height to end at (inclusive)
    #[arg(long = "end", short = 'e')]
    end: usize,
    /// Seed for the rng
    #[arg(long = "rng", short = 'r', default_value_t = 4711)]
    seed: u64,
    /// Number of blocks to analyse. Pass '0' to analyse all
    #[arg(long = "num", short = 'n', default_value_t = 100)]
    num_blocks: usize,
}
fn main() {
    let opt = Opt::parse();
    let log_level = opt.log_level;
    env_logger::builder().filter_level(log_level).init();
    let channels = Channel::read_from_file(&opt.ln_chanpoints)
        .iter()
        .map(|c| c.funding_txid.clone())
        .collect::<Vec<String>>();
    let mut rng = StdRng::seed_from_u64(opt.seed);
    let range = (opt.start..=opt.end).collect::<Vec<usize>>();
    let heights_to_analyse = if opt.num_blocks > 0 {
        let num_blocks = if opt.num_blocks > range.len() {
            warn!("number of blocks is greater than provided range. defaulting to max");
            range.len()
        } else {
            opt.num_blocks
        };
        range
            .sample(&mut rng, num_blocks)
            .copied()
            .collect::<Vec<usize>>()
    } else {
        range
    };
    info!("Sampled {} blocks for analysis", heights_to_analyse.len());
    let client = Client::new();
    std::fs::create_dir_all(opt.output_path.clone())
        .expect("Failure creating {} directory for results.");
    let properties_file = PathBuf::from(opt.output_path.clone()).join("btc-tx-properties.json");
    info!("Starting API queries");
    let mut txs = vec![];
    let start = Instant::now();
    for height in heights_to_analyse {
        info!("Getting block at height {}", height);
        match client
            .get(format!(
                "https://blockstream.info/api/block-height/{}",
                height
            ))
            .send()
        {
            Ok(get) => match get.text() {
                Ok(hash) => {
                    // we can get this blocks txs
                    info!("get block {}", hash);
                    match client
                        .get(format!("https://blockstream.info/api/block/{}", hash))
                        .send()
                    {
                        Ok(get) => match get.text() {
                            Ok(text) => {
                                if let Some(block) = Block::from_json_str(&text) {
                                    let block_txs = get_txs_for_block(&block, &client);
                                    info!("received {} txs for {}", block_txs.len(), block.id);
                                    txs.extend(block_txs);
                                }
                            }
                            Err(e) => error!("Error getting text from response: {e}"),
                        },
                        Err(e) => error!("API get failed: {e}"),
                    }
                }
                Err(e) => error!("Error getting text from response: {e}"),
            },
            Err(e) => error!("API get failed: {e}"),
        }
        // avoid rate limit
        thread::sleep(Duration::from_millis(200));
    }

    // exclude LN
    let prev_num_txs = txs.len();
    txs.retain(|t| channels.contains(&t.txid));
    let removed = prev_num_txs - txs.len();
    info!(
        "Analysing property heuristics of {} TXs after removing {} funding TXs",
        txs.len(),
        removed
    );
    let analysis = analysis::analyse_txs(&txs);
    let _ = write_to_file(analysis, &properties_file);
    let elapsed = start.elapsed().as_secs();
    info!("Finished analysis of {} TXs after {}s", txs.len(), elapsed)
}

fn get_txs_for_block(block: &Block, client: &Client) -> Vec<Tx> {
    let mut txs = vec![];
    let mut curr_idx = 0;
    debug!("Will fetch {} txs from block {}", block.tx_count, block.id);
    while curr_idx < block.tx_count {
        info!("current index = {}", curr_idx);
        match client
            .get(format!(
                "https://blockstream.info/api/block/{}/txs/{}",
                block.id, curr_idx
            ))
            .send()
        {
            Ok(get) => match get.text() {
                Ok(text) => {
                    if let Some(block_txs) = Tx::from_json_str_to_vec(&text) {
                        info!("extending with {} txs", block_txs.len());
                        // has to be multiple according to api documentation
                        curr_idx = (curr_idx + block_txs.len()).next_multiple_of(25);
                        txs.extend(block_txs);
                    } else {
                        // avoid endless loop
                        curr_idx += 25;
                    }
                }
                Err(e) => {
                    error!("Error getting text from response: {e}");
                    // avoid endless loop
                    curr_idx += 25;
                }
            },
            Err(e) => {
                error!("API get failed: {e}");
                // avoid endless loop
                curr_idx += 25;
            }
        }
        thread::sleep(Duration::from_millis(200));
        if curr_idx > 30 {
            break;
        }
    }
    info!("got {} TXs from {}", txs.len(), block.id);
    txs
}

fn write_to_file(analysis: Analysis, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if let Ok(f) = File::create(path) {
        match serde_json::to_writer_pretty(f, &analysis) {
            Ok(_) => {
                info!(
                    "Analysis results written to {} successfully",
                    path.display()
                )
            }
            Err(e) => error!("Error {e} writing to {} as JSON", path.display()),
        }
    }

    Ok(())
}
