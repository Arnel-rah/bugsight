use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("ModuleNotFoundError") || input.contains("No module named") {
        return Some(ParsedError {
            error_type: "Python Missing Module".to_string(),
            message: input.to_string(),
            suggestion: "Run `pip install <module>` to install the missing package. If in a venv, activate it first.".to_string(),
        });
    }

    if input.contains("IndentationError") {
        return Some(ParsedError {
            error_type: "Python Indentation Error".to_string(),
            message: input.to_string(),
            suggestion: "Check your indentation — use spaces OR tabs consistently, never mix both."
                .to_string(),
        });
    }

    if input.contains("TypeError") {
        return Some(ParsedError {
            error_type: "Python Type Error".to_string(),
            message: input.to_string(),
            suggestion: "You're using the wrong type. Check the expected types and use `type()` or `isinstance()` to debug.".to_string(),
        });
    }

    if input.contains("KeyError") {
        return Some(ParsedError {
            error_type: "Python Key Error".to_string(),
            message: input.to_string(),
            suggestion: "The key doesn't exist in the dict. Use `dict.get(key)` instead of `dict[key]` to avoid crashes.".to_string(),
        });
    }

    if input.contains("AttributeError") {
        return Some(ParsedError {
            error_type: "Python Attribute Error".to_string(),
            message: input.to_string(),
            suggestion: "The object doesn't have this attribute. Check for typos or use `hasattr(obj, 'attr')` before accessing.".to_string(),
        });
    }

    if input.contains("SyntaxError") {
        return Some(ParsedError {
            error_type: "Python Syntax Error".to_string(),
            message: input.to_string(),
            suggestion: "Check for missing colons, parentheses, or quotes. Look at the line number indicated.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_not_found() {
        let input = "ModuleNotFoundError: No module named 'requests'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Python Missing Module");
        assert!(result.suggestion.contains("pip install"));
    }

    #[test]
    fn test_indentation_error() {
        let input = "IndentationError: unexpected indent";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Python Indentation Error");
    }

    #[test]
    fn test_key_error() {
        let input = "KeyError: 'username'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Python Key Error");
        assert!(result.suggestion.contains("dict.get(key)"));
    }

    #[test]
    fn test_no_match() {
        let input = "Everything is fine";
        assert!(parse(input).is_none());
    }
}
