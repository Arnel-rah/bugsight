use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("permission denied") {
        return Some(ParsedError {
            error_type: "Permission Error".to_string(),
            message: input.to_string(),
            suggestion: "Try running with `sudo` or check file permissions with `ls -la`.".to_string(),
        });
    }

    if input.contains("No such file or directory") {
        return Some(ParsedError {
            error_type: "File Not Found".to_string(),
            message: input.to_string(),
            suggestion: "Check the path exists with `ls` or `pwd`.".to_string(),
        });
    }

    None
}