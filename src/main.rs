fn main() {
    let cli = Cli::parse();
    let cfg = config::load();
    let msg = lang::get(&cfg);
    match cli {
        Cli { init: true, .. }           => config::init_with_msg(msg),
        Cli { history: true, .. }        => history::show_with_msg(msg),
        Cli { clear_history: true, .. }  => history::clear_with_msg(msg),
        Cli { stats: true, .. }          => history::stats_with_msg(msg),
        Cli { daemon: true, .. }         => daemon::start(&cfg, cli.port),

        Cli { watch: Some(path), .. }    => watcher::watch(&path, &cfg),
        Cli { explain: Some(err), .. }   => handle_error(&err, &cfg, cli.json),
        Cli { file: Some(path), .. }     => handle_file_input(&path, &cfg, cli.json),

        _ => handle_stdin_input(&cfg, cli.json),
    }
}

fn handle_file_input(path: &str, cfg: &config::Config, json: bool) {
    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.lines() {
                handle_error(line, cfg, json);
            }
        }
        Err(e) => eprintln!("Error reading file {}: {}", path, e),
    }
}

fn handle_stdin_input(cfg: &config::Config, json: bool) {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map_while(Result::ok) {
        handle_error(&line, cfg, json);
    }
}
