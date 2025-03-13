mod lexer;
mod parser;

use std::{fs::File, io::{BufReader, Read}, process::exit};

use clap::Parser;
use log::{info, Level, LevelFilter, SetLoggerError};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    info!("{}", PKG_NAME);
    info!("Version: {}", PKG_VERSION);

    let mut file = match File::open(args.input_file) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {}", err);
            exit(1)
        }
    };

    let mut file_content = Vec::new();
    let _bytes_read = match file.read_to_end(&mut file_content) {
        Ok(size) => size,
        Err(err) => {
            println!("Failed to read file: {}", err);
            exit(1)
        }
    };

    file_content.push(b'\0');

    let lexemes = match lexer::process(&file_content) {
        Ok(lexemes) => lexemes,
        Err(err) => {
            println!("Failed to parse file.\n{}", err);
            exit(1)
        }
    };

    for lexeme in &lexemes {
        println!("{:?}", lexeme)
    }
}
