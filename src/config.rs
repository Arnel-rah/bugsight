use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::lang::Messages;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Config {
    #[serde(default = "default_true")]
    pub ai_enabled: bool,

    #[serde(default = "default_true")]
    pub history_enabled: bool,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "default_max_history")]
    pub max_history: usize,
}

fn default_true() -> bool {
    true
}
fn default_language() -> String {
    "en".to_string()
}
fn default_max_history() -> usize {
    100
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai_enabled: true,
            history_enabled: true,
            language: "en".to_string(),
            max_history: 100,
        }
    }
}

fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".bugsight.toml"))
}

pub fn load() -> Config {
    if let Some(path) = config_path() {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
    }
    Config::default()
}

pub fn init_with_msg(msg: &Messages) {
    if let Some(path) = config_path() {
        if path.exists() {
            println!("{} {}", msg.config_exists, path.display());
            return;
        }

        let default_config = r#"# bugsight configuration file

# Enable AI fallback for unknown errors (requires GROQ_API_KEY)
ai_enabled = true

# Save analyzed errors to history
history_enabled = true

# Language for suggestions: "en" or "fr"
language = "en"

# Maximum number of errors to keep in history
max_history = 100
"#;

        match std::fs::write(&path, default_config) {
            Ok(_) => println!("{} {}", msg.config_created, path.display()),
            Err(e) => eprintln!("Failed to create config: {}", e),
        }
    }
}

// pub fn init() {
//     init_with_msg(&crate::lang::EN);
// }
