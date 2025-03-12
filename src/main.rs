mod lexer;
mod parser;
// mod server;
mod logger;

use std::process::exit;

use clap::Parser;
use log::{info, Level, LevelFilter, SetLoggerError};
use logger::Logger;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static LOGGER: Logger = Logger;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let log_level = "info";
    let log_level = match log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Error,
    };

    match log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level)) {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to set log level: {}", err);
        }
    }

    info!("{}", PKG_NAME);
    info!("Version: {}", PKG_VERSION);
}
