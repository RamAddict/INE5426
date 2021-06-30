// Import External Modules
extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::{env, fs, path::PathBuf};

use pest::Parser;

use clap::{AppSettings, Clap};
// Import Internal Modules
#[cfg(test)]
mod test;
mod symbols_table;
// Configure Parser

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ParserCC20211;
// Configure CLI Args
#[derive(Clap)]
#[clap(version = "v0.0.1")]
#[clap(name = "S.A.P.O")]
#[clap(about = "SAPO's A Parser Option")]
#[clap(setting = AppSettings::ColoredHelp)]
struct CLIOptions {
    // File to Read
    #[clap(about = "The file to be read and parsed. Example: ~/example_file.lcc")]
    input: PathBuf
}
// Define Entrypoint
fn main() {
    // Parse Arguments
    let cli_options = CLIOptions::parse();
    // Read File
    let file_path = PathBuf::from(cli_options.input);
    let file_path = match file_path.is_absolute() {
        true => file_path,
        false => env::current_dir().unwrap_or(PathBuf::from("/")).join(file_path),
    };
    let file_content = fs::read_to_string(file_path).expect("Ops. Something occured while reading your file");
    // Try Parse Program
    match ParserCC20211::parse(Rule::program, &file_content) {
        Ok(_pairs) => {
            // Success Parsed
            println!("All Right!");
        },
        Err(error) => {
            println!("{}", error);
        },
    }
}