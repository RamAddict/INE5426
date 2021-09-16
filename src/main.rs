// Import External Modules
extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::{env, fs, path::PathBuf, time::Instant};

use comfy_table::{Cell, Color, Table, presets::{UTF8_FULL, UTF8_HORIZONTAL_BORDERS_ONLY}, Attribute, CellAlignment};
use pest::Parser;

use clap::{AppSettings, Clap};
use symbols_table::SymbolsTable;
use token_list::TokenList;
// Import Internal Modules
#[cfg(test)]
mod test;
mod token_type;
mod token_list;
mod symbols_table;
mod consumer;
mod ast;
mod ir;
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
    #[clap(subcommand)]
    sub_command: CLISubcommands
}

#[derive(Clap)]
enum CLISubcommands {
    #[clap(version = "v0.0.1")]
    #[clap(name = "lex")]
    #[clap(about = "Do a lexical analysis using the CC20211 lang")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Lexer(CLICommandLexer),
    #[clap(version = "v0.0.1")]
    #[clap(name = "syntax")]
    #[clap(about = "Do a syntax analysis using the CC20211 lang")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Syntax(CLICommandSyntax)
}

#[derive(Clap)]
struct CLICommandLexer {
    // File to Read
    #[clap(about = "The file to be read and parsed. Example: ./example_file.lcc")]
    input: String,
    // Options
    #[clap(short = 's', long = "symbols", about = "Display the symbols table")]
    show_symbols: bool,
    #[clap(short = 't', long = "tokens", about = "Display the tokens list")]
    show_tokens: bool,
    #[clap(short = 'i', long = "info", about = "Prints parsing information", parse(try_from_str), default_value = "true")]
    show_info: bool
}

#[derive(Clap)]
struct CLICommandSyntax {
    // File to Read
    #[clap(about = "The file to be read and parsed. Example: ./example_file.lcc")]
    input: String,
    // Options
    #[clap(short = 'i', long = "info", about = "Prints syntax information", parse(try_from_str), default_value = "true")]
    show_info: bool
}

#[derive(Clap)]
struct CLICommandSemantic {
    // File to Read
    #[clap(about = "The file to be read and parsed. Example: ./example_file.lcc")]
    input: String,
    // Options
    #[clap(short = 'i', long = "info", about = "Prints semantic information", parse(try_from_str), default_value = "true")]
    show_info: bool
}
// Define Helpers
fn read_file_content(file_path_str: &str) -> Result<String, std::io::Error> {
    let file_path = PathBuf::from(file_path_str);
    let file_path = match file_path.is_absolute() {
        true => file_path,
        false => env::current_dir()
            .unwrap_or(PathBuf::from("/"))
            .join(file_path),
    };
    // Return content
    fs::read_to_string(file_path)
}

fn section_header(section_name: &str) -> Table {
    let mut table = Table::new();
    let section_label = Cell::new(section_name)
        .set_alignment(comfy_table::CellAlignment::Center)
        .fg(Color::Green)
        .add_attribute(Attribute::Bold);
    table
        .apply_modifier(UTF8_HORIZONTAL_BORDERS_ONLY)
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .set_header(vec![section_label]);

    return table;
}
// Define Entrypoint
fn main() {
    // Parse Arguments
    let cli_options = CLIOptions::parse();
    // Match Command
    match cli_options.sub_command {
        CLISubcommands::Lexer(lexer_opts) => {
            // Read File
            let file_content = read_file_content(lexer_opts.input.as_str())
                .expect("Ops. Something occured while reading your file");
            // Try Parse Program
            let parsing_timing = Instant::now();
            match ParserCC20211::parse(Rule::tokens, &file_content) {
                Ok(pairs) => {
                    // Success Parsed
                    let elapsed_time = parsing_timing.elapsed();
                    // Generate Tokens List
                    let tokens_list = TokenList::new(&mut pairs.clone());
                    let symbols_table = SymbolsTable::new(&mut pairs.clone());
                    // Print Info Based on Options
                    if lexer_opts.show_tokens {
                        let section_header = section_header("Tokens List");
                        println!("{}", section_header);
                        println!("{}\n\n", tokens_list.to_table());
                    }
                    if lexer_opts.show_symbols {
                        let section_header = section_header("Symbols Table");
                        println!("{}", section_header);
                        println!("{}\n\n", symbols_table.to_table());
                    }
                    if lexer_opts.show_info {
                        let mut info_table = Table::new();
                        info_table
                            .apply_modifier(UTF8_FULL)
                            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
                            .set_header(vec![Cell::new("Statistics").fg(Color::Yellow).add_attribute(Attribute::Bold), Cell::new("")])
                            .add_row(vec![Cell::new("Status").add_attribute(Attribute::Bold), Cell::new("Success").fg(Color::Green).add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center)])
                            .add_row(vec![Cell::new("Tokens Identified:"), Cell::new(tokens_list.len()).add_attribute(Attribute::Bold).set_alignment(CellAlignment::Right)])
                            .add_row(vec![Cell::new("Symbols Identified:"), Cell::new(symbols_table.len()).add_attribute(Attribute::Bold).set_alignment(CellAlignment::Right)])
                            .add_row(vec![Cell::new("Elapsed Time:"), Cell::new(format!("{:.3}s", elapsed_time.as_secs_f64())).set_alignment(CellAlignment::Right)]);
                        println!("{}", info_table);
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        CLISubcommands::Syntax(syntax_options) => {
            // Read File
            let file_content = read_file_content(syntax_options.input.as_str())
                .expect("Ops. Something occured while reading your file");
            // Try Parse Program
            let parsing_timing = Instant::now();
            match ParserCC20211::parse(Rule::program, &file_content) {
                Ok(_pairs) => {
                    // Success Parsed
                    let elapsed_time = parsing_timing.elapsed();
                    // Print Info Based on Options
                    if syntax_options.show_info {
                        let mut info_table = Table::new();
                        info_table
                            .apply_modifier(UTF8_FULL)
                            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
                            .set_header(vec![Cell::new("Statistics").fg(Color::Yellow).add_attribute(Attribute::Bold), Cell::new("")])
                            .add_row(vec![Cell::new("Status").add_attribute(Attribute::Bold), Cell::new("Success").fg(Color::Green).add_attribute(Attribute::Bold).set_alignment(CellAlignment::Center)])
                            .add_row(vec![Cell::new("Elapsed Time:"), Cell::new(format!("{:.3}s", elapsed_time.as_secs_f64())).set_alignment(CellAlignment::Right)]);
                        println!("{}", info_table);
                    }
                }
                Err(error) => {
                    // println!("{:?}", error);
                    println!("{}", error);
                }
            }
        }
    }
}
