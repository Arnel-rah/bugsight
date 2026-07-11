use anyhow::{Context, Result};
use std::{fs, io::{self, BufRead}};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::load();
    let msg = lang::get(&cfg);

    let Cli {
        init, history, clear_history, stats,
        daemon, port, watch, explain, file, json
    } = cli;

    match (init, history, clear_history, stats, daemon, watch, explain, file) {
        (true, ..)            => config::init_with_msg(msg),
        (_, true, ..)         => history::show_with_msg(msg),
        (_, _, true, ..)      => history::clear_with_msg(msg),
        (_, _, _, true, ..)   => history::stats_with_msg(msg),
        (_, _, _, _, true, ..)=> daemon::start(&cfg, port),
        (_, _, _, _, _, Some(path), ..)  => watcher::watch(&path, &cfg),
        (_, _, _, _, _, _, Some(err), ..) => handle_error(&err, &cfg, json)?,
        (_, _, _, _, _, _, _, Some(path)) => handle_file_input(&path, &cfg, json)?,
        _                     => handle_stdin_input(&cfg, json)?,
    }

    Ok(())
}

fn process_lines<I, S>(lines: I, cfg: &config::Config, json: bool) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for line in lines {
        handle_error(line.as_ref(), cfg, json)?;
    }
    Ok(())
}

fn handle_file_input(path: &str, cfg: &config::Config, json: bool) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read file '{path}'"))?;
    process_lines(content.lines(), cfg, json)
}

fn handle_stdin_input(cfg: &config::Config, json: bool) -> Result<()> {
    let stdin = io::stdin();
    let lines: Result<Vec<String>, _> = stdin.lock().lines().collect();
    let lines = lines.context("Failed to read from stdin")?;

    process_lines(lines, cfg, json)
}
