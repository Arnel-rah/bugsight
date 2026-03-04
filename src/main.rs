mod ai;
mod analyzer;
mod parsers;

use clap::Parser;
use colored::*;
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(version = "0.2.0")]
#[command(about = "Debug smarter, not harder")]
struct Cli {
    #[arg(short, long)]
    explain: Option<String>,
    #[arg(short, long)]
    file: Option<String>,
}

fn handle_error(input: &str) {
    match analyzer::analyze(input) {
        Some(result) => {
            println!("\n{} {}", "Analyzing:".yellow(), input);
            println!("{} {}", "Type:".bold(), result.error_type.red());
            println!("{} {}", "Message:".bold(), result.message);
            println!("{} {}", "Suggestion:".green().bold(), result.suggestion);
            println!();
        }
        None => {
            println!("{}", input);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(error) = cli.explain {
        handle_error(&error);
    } else if let Some(path) = cli.file {
        match fs::read_to_string(&path) {
            Ok(content) => {
                for line in content.lines() {
                    handle_error(line);
                }
            }
            Err(e) => {
                eprintln!("{} {}: {}", "Error reading file".red(), path, e);
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => handle_error(&l),
                Err(_) => break,
            }
        }
    }
}
