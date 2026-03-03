pub mod rust;
pub mod general;

pub use rust::ParsedError;

pub fn parse_error(input: &str) -> Option<ParsedError> {
    rust::parse(input)
        .or_else(|| general::parse(input))
}