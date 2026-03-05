use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

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
