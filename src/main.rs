mod ai;
mod analyzer;
mod config;
mod history;
mod parsers;

use clap::Parser;
use colored::*;
use std::fs;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(version = "0.5.1")]
#[command(about = "Debug smarter, not harder")]
struct Cli {
    #[arg(short, long)]
    explain: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(long)]
    history: bool,

    #[arg(long)]
    clear_history: bool,

    #[arg(long)]
    stats: bool,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    init: bool,
}

fn handle_error(input: &str, cfg: &config::Config, json: bool) {
    match analyzer::analyze(input, cfg) {
        Some(result) => {
            if json {
                let output = serde_json::json!({
                    "error_type": result.error_type,
                    "message": result.message,
                    "suggestion": result.suggestion
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                println!("\n{} {}", "Analyzing:".yellow(), input);
                println!("{} {}", "Type:".bold(), result.error_type.red());
                println!("{} {}", "Message:".bold(), result.message);
                println!("{} {}", "Suggestion:".green().bold(), result.suggestion);
                println!();
            }

            if cfg.history_enabled {
                history::save(input, &result.error_type);
            }
        }
        None => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "error_type": null, "message": input, "suggestion": null })
                );
            } else {
                println!("{}", input);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let cfg = config::load();

    if cli.init {
        config::init();
    } else if cli.history {
        history::show();
    } else if cli.clear_history {
        history::clear();
    } else if cli.stats {
        history::stats();
    } else if let Some(error) = cli.explain {
        handle_error(&error, &cfg, cli.json);
    } else if let Some(path) = cli.file {
        match fs::read_to_string(&path) {
            Ok(content) => {
                for line in content.lines() {
                    handle_error(line, &cfg, cli.json);
                }
            }
            Err(e) => {
                eprintln!("{} {}: {}", "Error reading file:".red(), path, e);
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => handle_error(&l, &cfg, cli.json),
                Err(_) => break,
            }
        }
    }
}
