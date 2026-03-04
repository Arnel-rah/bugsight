pub mod rust;
pub mod general;
pub mod golang;
pub mod python;
pub mod nodejs;

pub use rust::ParsedError;

pub fn parse_error(input: &str) -> Option<ParsedError> {
    rust::parse(input)
        .or_else(|| general::parse(input))
        .or_else(|| golang::parse(input))
        .or_else(|| python::parse(input))
        .or_else(|| nodejs::parse(input))
}