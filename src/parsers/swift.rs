use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("fatal error: unexpectedly found nil") {
        return Some(ParsedError {
            error_type: "Swift Nil Unwrap".to_string(),
            message: input.to_string(),
            suggestion: "You're force-unwrapping a nil optional. Use `if let`, `guard let`, or `??` to safely unwrap optionals instead of `!`.".to_string(),
        });
    }

    if input.contains("EXC_BAD_ACCESS") {
        return Some(ParsedError {
            error_type: "Swift EXC_BAD_ACCESS".to_string(),
            message: input.to_string(),
            suggestion: "Accessing deallocated memory. Check for dangling pointers, retain cycles, or use weak/unowned references in closures.".to_string(),
        });
    }

    if input.contains("error: use of unresolved identifier") {
        return Some(ParsedError {
            error_type: "Swift Unresolved Identifier".to_string(),
            message: input.to_string(),
            suggestion: "Variable or function not found. Check for typos, missing imports, or that the symbol is in scope.".to_string(),
        });
    }

    if input.contains("error: cannot convert value of type") {
        return Some(ParsedError {
            error_type: "Swift Type Mismatch".to_string(),
            message: input.to_string(),
            suggestion: "Type mismatch. Use explicit casting like `Int(value)`, `String(value)`, or `as?` for safe downcasting.".to_string(),
        });
    }

    if input.contains("error: value of type") && input.contains("has no member") {
        return Some(ParsedError {
            error_type: "Swift No Member".to_string(),
            message: input.to_string(),
            suggestion: "Property or method doesn't exist on this type. Check the Swift documentation or verify the type with `type(of: value)`.".to_string(),
        });
    }

    if input.contains("Thread 1: Fatal error") {
        return Some(ParsedError {
            error_type: "Swift Fatal Error".to_string(),
            message: input.to_string(),
            suggestion: "Fatal error at runtime. Check the message for details — common causes are index out of range, force unwrap on nil, or precondition failure.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil_unwrap() {
        let input = "fatal error: unexpectedly found nil while unwrapping an Optional value";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Swift Nil Unwrap");
        assert!(result.suggestion.contains("if let"));
    }

    #[test]
    fn test_exc_bad_access() {
        let input = "EXC_BAD_ACCESS (SIGSEGV)";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Swift EXC_BAD_ACCESS");
    }

    #[test]
    fn test_unresolved_identifier() {
        let input = "error: use of unresolved identifier 'myVariable'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Swift Unresolved Identifier");
    }

    #[test]
    fn test_type_mismatch() {
        let input = "error: cannot convert value of type 'String' to expected argument type 'Int'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Swift Type Mismatch");
    }

    #[test]
    fn test_no_match() {
        let input = "Build succeeded";
        assert!(parse(input).is_none());
    }
}
