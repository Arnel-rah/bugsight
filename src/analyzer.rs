
use crate::parsers::ParsedError;

pub fn analyze(input: &str) -> Option<ParsedError> {
    crate::parsers::parse_error(input)
}