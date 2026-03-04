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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_denied() {
        let input = "permission denied: cannot open file config.toml";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Permission Error");
        assert!(result.suggestion.contains("sudo"));
    }

    #[test]
    fn test_file_not_found() {
        let input = "No such file or directory: /home/user/.config";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "File Not Found");
        assert!(result.suggestion.contains("ls"));
    }

    #[test]
    fn test_unknown_error_returns_none() {
        let input = "some random log message";
        let result = parse(input);
        assert!(result.is_none());
    }
}