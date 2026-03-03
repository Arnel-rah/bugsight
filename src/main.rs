mod parsers;
mod analyzer;

use clap::Parser;
use colored::*;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(about = "Debug smarter, not harder")]
struct Cli {
    #[arg(short, long)]
    explain: Option<String>,
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