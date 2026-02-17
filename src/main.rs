use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use clap::Parser;
use log::{LevelFilter, debug, error, info, trace};
use types::{Channel, Properties, Tx};

mod analysis;
mod types;

#[derive(Debug, Parser)]
struct Opt {
    /// Path to the CSV with the channel points
    #[arg(long = "input", short = 'i')]
    chanpoints: PathBuf,
    #[arg(long = "log", short = 'l', default_value = "info")]
    log_level: LevelFilter,
    /// Path to directory in which the results will be stored
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
    let properties_file = PathBuf::from(opt.output_path.clone()).join("tx-properties.csv");
    info!("Analysis will be written to {}", properties_file.display());
    let mut wtr = File::create_new(properties_file).expect("File already exists?");
    writeln!(wtr, "num_outputs,num_p2wsh_outputs,num_p2tr_outputs,output_types,output_values,funding_types,funding_amounts").expect("failed to write header");
    info!("Querying API for {} channels", channels.len());
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
                        debug!("Analysing properties of tx {}", tx.txid);
                        let property = analysis::analyse_tx(tx);
                        write_to_file(&property, &mut wtr);
                    }
                }
                Err(e) => error!("Error getting text from response: {e}"),
            },
            Err(e) => error!("API get failed: {e}"),
        }
        // avoid rate limit
        thread::sleep(Duration::from_millis(250));
    }
    let elapsed = start.elapsed().as_secs();
    info!(
        "Finished analysis of {} TXs after {}s",
        channels.len(),
        elapsed
    )
}

fn write_to_file(property: &Properties, wtr: &mut File) {
    let output_types = property
        .type_output_value
        .iter()
        .map(|(t, _)| t.clone())
        .collect::<Vec<_>>()
        .join(",");
    let output_values = property
        .type_output_value
        .iter()
        .map(|(_, v)| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let funding_types = property
        .type_amt_funding_addresses
        .iter()
        .map(|(t, _)| t.clone())
        .collect::<Vec<_>>()
        .join(",");
    let funding_amounts = property
        .type_amt_funding_addresses
        .iter()
        .map(|(_, v)| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    writeln!(
        wtr,
        "{},{},{},{},{},{},{}",
        property.num_outputs,
        property.num_p2wsh_outputs,
        property.num_p2tr_outputs,
        output_types,
        output_values,
        funding_types,
        funding_amounts
    )
    .expect("Failure writing property to file");
}
