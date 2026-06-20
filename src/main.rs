use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::load();
    let msg = lang::get(&cfg);

    match cli {
        Cli { init: true, .. }          => config::init_with_msg(msg),
        Cli { history: true, .. }       => history::show_with_msg(msg),
        Cli { clear_history: true, .. } => history::clear_with_msg(msg),
        Cli { stats: true, .. }         => history::stats_with_msg(msg),
        Cli { daemon: true, .. }        => daemon::start(&cfg, cli.port),
        Cli { watch: Some(path), .. }   => watcher::watch(&path, &cfg),
        Cli { explain: Some(err), .. }  => handle_error(&err, &cfg, cli.json)?,
        Cli { file: Some(path), .. }    => handle_file_input(&path, &cfg, cli.json)?,
        _                               => handle_stdin_input(&cfg, cli.json)?,
    }

    Ok(())
}

fn process_lines<'a>(
    lines: impl Iterator<Item = &'a str>,
    cfg: &config::Config,
    json: bool,
) -> Result<()> {
    for line in lines {
        handle_error(line, cfg, json)?;
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
    let lines = stdin.lock().lines().map_while(Result::ok);
    for line in lines {
        handle_error(&line, cfg, json)?;
    }
    Ok(())
}
