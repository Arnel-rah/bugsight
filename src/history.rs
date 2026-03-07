use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use colored::Colorize;

use crate::lang::Messages;

fn history_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".bugsight_history"))
}

pub fn save(error: &str, error_type: &str) {
    if let Some(path) = history_path() {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let entry = format!("[{}] ({}) {}\n", timestamp, error_type, error);
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = file.write_all(entry.as_bytes());
        }
    }
}
pub fn show_with_msg(msg: &Messages) {
    if let Some(path) = history_path() {
        match fs::read_to_string(&path) {
            Ok(content) if !content.is_empty() => {
                println!("{}", content);
            }
            _ => {
                println!("{}", msg.no_history);
            }
        }
    }
}

pub fn clear_with_msg(msg: &Messages) {
    if let Some(path) = history_path() {
        let _ = fs::remove_file(path);
        println!("{}", msg.history_cleared);
    }
}

pub fn stats_with_msg(msg: &Messages) {
    if let Some(path) = history_path() {
        match fs::read_to_string(&path) {
            Ok(content) if !content.is_empty() => {
                let mut counts: HashMap<String, usize> = HashMap::new();
                let mut total = 0;

                for line in content.lines() {
                    if let Some(start) = line.find('(') {
                        if let Some(end) = line.find(')') {
                            let error_type = &line[start + 1..end];
                            *counts.entry(error_type.to_string()).or_insert(0) += 1;
                            total += 1;
                        }
                    }
                }

                let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(&a.1));

                println!("\n{}", msg.error_stats.bold());
                println!("{}", "─".repeat(40));
                for (error_type, count) in &sorted {
                    let bar = "█".repeat(*count);
                    let percentage = (*count as f64 / total as f64 * 100.0) as usize;
                    println!(
                        "{:<30} {} {}% {}",
                        error_type.red(),
                        count.to_string().bold(),
                        percentage,
                        bar.green()
                    );
                }
                println!("{}", "─".repeat(40));
                println!("{} {}", msg.total_errors.bold(), total);
                println!();
            }
            _ => {
                println!("{}", msg.no_history);
            }
        }
    }
}

pub fn show() {
    if let Some(path) = history_path() {
        match fs::read_to_string(&path) {
            Ok(content) if !content.is_empty() => {
                println!("{}", content);
            }
            _ => {
                println!("No history yet. Run bugsight to analyze some errors first.");
            }
        }
    }
}

pub fn clear() {
    if let Some(path) = history_path() {
        let _ = fs::remove_file(path);
        println!("History cleared.");
    }
}
