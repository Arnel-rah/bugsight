use crate::config::Config;

#[allow(dead_code)]
pub struct Messages {
    pub analyzing: &'static str,
    pub error_type: &'static str,
    pub message: &'static str,
    pub suggestion: &'static str,
    pub no_history: &'static str,
    pub history_cleared: &'static str,
    pub config_created: &'static str,
    pub config_exists: &'static str,
    pub total_errors: &'static str,
    pub error_stats: &'static str,
}

pub const EN: Messages = Messages {
    analyzing: "Analyzing:",
    error_type: "Type:",
    message: "Message:",
    suggestion: "Suggestion:",
    no_history: "No history yet. Run bugsight to analyze some errors first.",
    history_cleared: "History cleared.",
    config_created: "Config created at",
    config_exists: "Config already exists at",
    total_errors: "Total errors analyzed:",
    error_stats: "Error Statistics",
};

pub const FR: Messages = Messages {
    analyzing: "Analyse en cours :",
    error_type: "Type :",
    message: "Message :",
    suggestion: "Suggestion :",
    no_history: "Aucun historique. Analysez d'abord des erreurs avec bugsight.",
    history_cleared: "Historique effacé.",
    config_created: "Configuration créée à",
    config_exists: "La configuration existe déjà à",
    total_errors: "Total des erreurs analysées :",
    error_stats: "Statistiques des erreurs",
};

pub fn get(config: &Config) -> &'static Messages {
    match config.language.as_str() {
        "fr" => &FR,
        _ => &EN,
    }
}
