use std::{
    error::Error,
    fs::File,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use common::{Channel, Tx};
use log::{LevelFilter, error, info, trace};
use types::{API_CALLS, Analysis};

mod analysis;
mod types;

#[derive(Debug, Parser)]
struct Opt {
    /// Path to the CSV with the channel points
    #[arg(long = "input", short = 'i')]
    chanpoints: PathBuf,
    #[arg(long = "log", short = 'l', default_value = "info")]
    log_level: LevelFilter,
    #[arg(long = "out", short = 'o', default_value = "./results")]
    output_path: String,
}

fn main() {
    let opt = Opt::parse();
    let log_level = opt.log_level;
    env_logger::builder().filter_level(log_level).init();
    let start = Instant::now();
    let channels = Channel::read_from_file(&opt.chanpoints);
    let client = reqwest::blocking::Client::new();
    std::fs::create_dir_all(opt.output_path.clone())
        .expect("Failure creating {} directory for results.");
    let intervals_file = PathBuf::from(opt.output_path.clone()).join("channel-intervals.json");

    info!("Querying API for {} channels", channels.len());
    let mut txs = vec![];
    for channel in channels.iter() {
        trace!("Querying API for {}", channel.funding_txid);
        match client
            .get(format!(
                "https://blockstream.info/api/tx/{}",
                channel.funding_txid
            ))
            .send()
        {
            Ok(get) => match get.text() {
                Ok(text) => {
                    if let Some(tx) = Tx::from_json_str(&text) {
                        txs.push(tx);
                    }
                }
                Err(e) => error!("Error getting text from response: {e}"),
            },
            Err(e) => error!("API get failed: {e}"),
        }
        // avoid rate limit
        API_CALLS.with(|c| c.set(c.get() + 1));
        let api_calls = API_CALLS.with(|c| c.get());
        // max 700/hour
        let sleep = if api_calls >= 650 {
            info!("Sleeping for one hour after {api_calls} calls because of rate limit");
            API_CALLS.with(|c| c.set(0));
            3600000
        } else {
            250
        };
        thread::sleep(Duration::from_millis(sleep));
    }
    info!("Analysing channel opening frequency of {} TXs", txs.len());
    let analysis = analysis::analyse_frequency(&txs);
    let _ = write_to_file(analysis, &intervals_file);
    let elapsed = start.elapsed().as_secs();
    info!(
        "Finished frequency analysis of {} TXs after {}s",
        channels.len(),
        elapsed
    )
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
