use crate::analyzer;
use crate::config::Config;
use crate::lang;
use colored::*;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch(path: &str, cfg: &Config) {
    let msg = lang::get(cfg);
    let path = Path::new(path);

    if !path.exists() {
        eprintln!("{} {}", "File not found:".red(), path.display());
        return;
    }

    println!(
        "\n{} {}\n{}\n",
        "Watching:".green().bold(),
        path.display(),
        "Press Ctrl+C to stop.".dimmed()
    );

    // Position initiale — fin du fichier
    let mut offset = {
        let f = File::open(path).unwrap();
        f.metadata().unwrap().len()
    };

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_millis(500)),
    )
    .unwrap();

    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let mut f = File::open(path).unwrap();
                    f.seek(SeekFrom::Start(offset)).unwrap();

                    let reader = BufReader::new(&f);
                    let mut new_offset = offset;

                    for line in reader.lines() {
                        match line {
                            Ok(l) => {
                                new_offset += l.len() as u64 + 1;
                                if l.trim().is_empty() {
                                    continue;
                                }

                                match analyzer::analyze(&l, cfg) {
                                    Some(result) => {
                                        let timestamp = chrono::Local::now().format("%H:%M:%S");
                                        println!(
                                            "\n{} {}",
                                            format!("[{}]", timestamp).dimmed(),
                                            "Error detected!".red().bold()
                                        );
                                        println!(
                                            "{} {}",
                                            msg.error_type.bold(),
                                            result.error_type.red()
                                        );
                                        println!(
                                            "{} {}",
                                            msg.suggestion.green().bold(),
                                            result.suggestion
                                        );
                                        println!();

                                        if cfg.history_enabled {
                                            crate::history::save(&l, &result.error_type);
                                        }
                                    }
                                    None => {
                                        let timestamp = chrono::Local::now().format("%H:%M:%S");
                                        println!("{} {}", format!("[{}]", timestamp).dimmed(), l);
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }

                    offset = new_offset;
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
            _ => {}
        }
    }
}
