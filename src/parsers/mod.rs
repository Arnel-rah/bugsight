use serde::Serialize;
pub mod docker;
pub mod general;
pub mod git;
pub mod golang;
pub mod java;
pub mod nodejs;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;

#[derive(Serialize)]
pub struct ParsedError {
    pub error_type: String,
    pub message: String,
    pub suggestion: String,
}

pub fn parse_error(input: &str) -> Option<ParsedError> {
    rust::parse(input)
        .or_else(|| general::parse(input))
        .or_else(|| golang::parse(input))
        .or_else(|| python::parse(input))
        .or_else(|| nodejs::parse(input))
        .or_else(|| docker::parse(input))
        .or_else(|| git::parse(input))
        .or_else(|| java::parse(input))
        .or_else(|| php::parse(input))
        .or_else(|| ruby::parse(input))
}
