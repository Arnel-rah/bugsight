use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("Fatal error: Uncaught Error: Call to undefined function") {
        return Some(ParsedError {
            error_type: "PHP Undefined Function".to_string(),
            message: input.to_string(),
            suggestion: "The function doesn't exist. Check for typos or make sure the required file is included with `require_once`.".to_string(),
        });
    }

    if input.contains("Fatal error: Uncaught Error: Call to a member function") {
        return Some(ParsedError {
            error_type: "PHP Null Object Call".to_string(),
            message: input.to_string(),
            suggestion: "You're calling a method on null. Check if the object is initialized before calling methods on it.".to_string(),
        });
    }

    if input.contains("Parse error: syntax error") {
        return Some(ParsedError {
            error_type: "PHP Syntax Error".to_string(),
            message: input.to_string(),
            suggestion: "Syntax error in your PHP code. Check for missing semicolons, brackets or quotes on the line indicated.".to_string(),
        });
    }

    if input.contains("Warning: Undefined variable") {
        return Some(ParsedError {
            error_type: "PHP Undefined Variable".to_string(),
            message: input.to_string(),
            suggestion: "Variable is not defined. Initialize it before use or check for typos in the variable name.".to_string(),
        });
    }

    if input.contains("Fatal error: Allowed memory size") {
        return Some(ParsedError {
            error_type: "PHP Memory Limit".to_string(),
            message: input.to_string(),
            suggestion: "PHP memory limit exceeded. Increase it in php.ini: `memory_limit = 256M` or optimize your code to use less memory.".to_string(),
        });
    }

    if input.contains("Warning: Undefined array key") {
        return Some(ParsedError {
            error_type: "PHP Undefined Array Key".to_string(),
            message: input.to_string(),
            suggestion: "Array key doesn't exist. Use `isset($array['key'])` or `$array['key'] ?? 'default'` to avoid this warning.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_function() {
        let input = "Fatal error: Uncaught Error: Call to undefined function myFunc()";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "PHP Undefined Function");
        assert!(result.suggestion.contains("require_once"));
    }

    #[test]
    fn test_syntax_error() {
        let input = "Parse error: syntax error, unexpected token in /var/www/index.php on line 42";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "PHP Syntax Error");
    }

    #[test]
    fn test_undefined_variable() {
        let input = "Warning: Undefined variable $username in /app/index.php on line 10";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "PHP Undefined Variable");
    }

    #[test]
    fn test_memory_limit() {
        let input = "Fatal error: Allowed memory size of 134217728 bytes exhausted";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "PHP Memory Limit");
        assert!(result.suggestion.contains("memory_limit"));
    }

    #[test]
    fn test_no_match() {
        let input = "PHP application started successfully";
        assert!(parse(input).is_none());
    }
}
