use crate::ai;
use crate::config::Config;
use crate::parsers::ParsedError;

pub fn analyze(input: &str, config: &Config) -> Option<ParsedError> {
    if let Some(result) = crate::parsers::parse_error(input) {
        return Some(result);
    }

    if let Some(ai_response) = ai::ask_ai(input, config.ai_enabled) {
        return Some(ParsedError {
            error_type: "AI Analysis".to_string(),
            message: input.to_string(),
            suggestion: ai_response,
        });
    }

    None
}
