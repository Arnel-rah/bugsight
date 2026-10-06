mod ai;
mod analyzer;
mod config;
mod daemon;
mod history;
mod lang;
mod parsers;
mod watcher;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::{
    fs,
    io::{self, BufRead},
};

#[derive(Parser)]
#[command(name = "bugsight")]
#[command(version)]
#[command(about = "Debug smarter, not harder")]
struct Cli {
    #[arg(short, long)]
    explain: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    daemon: bool,

    #[arg(short, long, default_value = "7878")]
    port: u16,

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

    #[arg(short, long)]
    watch: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::load();
    let msg = lang::get(&cfg);

    let Cli {
        init,
        history,
        clear_history,
        stats,
        daemon,
        port,
        watch,
        explain,
        file,
        json,
    } = cli;

    match (
        init,
        history,
        clear_history,
        stats,
        daemon,
        watch,
        explain,
        file,
    ) {
        (true, ..) => config::init_with_msg(msg),
        (_, true, ..) => history::show_with_msg(msg),
        (_, _, true, ..) => history::clear_with_msg(msg),
        (_, _, _, true, ..) => history::stats_with_msg(msg),
        (_, _, _, _, true, ..) => daemon::start(&cfg, port),
        (_, _, _, _, _, Some(path), ..) => watcher::watch(&path, &cfg),
        (_, _, _, _, _, _, Some(err), ..) => handle_error(&err, &cfg, json)?,
        (_, _, _, _, _, _, _, Some(path)) => handle_file_input(&path, &cfg, json)?,
        _ => handle_stdin_input(&cfg, json)?,
    }

    Ok(())
}

fn handle_error(input: &str, cfg: &config::Config, json: bool) -> Result<()> {
    match analyzer::analyze(input, cfg) {
        Some(result) => {
            if json {
                let output = serde_json::json!({
                    "error_type": result.error_type,
                    "message": result.message,
                    "suggestion": result.suggestion
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
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
                    serde_json::json!({
                        "error_type": null,
                        "message": input,
                        "suggestion": null
                    })
                );
            } else {
                println!("{}", input);
            }
        }
    }

    Ok(())
}

fn process_lines<I, S>(lines: I, cfg: &config::Config, json: bool) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| line.as_ref().to_string())
        .collect();

    let multiline_input = lines.join("\n");
    let is_multiline_diagnostic = lines.len() > 1
        && (multiline_input.contains("Traceback (most recent call last):")
            || multiline_input.contains("error[")
            || multiline_input.contains("panicked at")
            || multiline_input.contains("Exception:")
            || multiline_input.contains("stack traceback"));

    if is_multiline_diagnostic {
        return handle_error(&multiline_input, cfg, json);
    }

    for line in lines {
        handle_error(line.as_ref(), cfg, json)?;
    }
    Ok(())
}

fn handle_file_input(path: &str, cfg: &config::Config, json: bool) -> Result<()> {
    let content = fs::read_to_string(path).with_context(|| format!("Cannot read file '{path}'"))?;
    process_lines(content.lines(), cfg, json)
}

fn handle_stdin_input(cfg: &config::Config, json: bool) -> Result<()> {
    let stdin = io::stdin();
    let lines: Result<Vec<String>, _> = stdin.lock().lines().collect();
    let lines = lines.context("Failed to read from stdin")?;

    process_lines(lines, cfg, json)
}
