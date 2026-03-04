use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("panic: runtime error: index out of range") {
        return Some(ParsedError {
            error_type: "Go Runtime Panic".to_string(),
            message: input.to_string(),
            suggestion: "Check slice/array length before indexing. Use `if i < len(slice)` before accessing `slice[i]`.".to_string(),
        });
    }

    if input.contains("panic: nil pointer dereference") {
        return Some(ParsedError {
            error_type: "Go Nil Pointer".to_string(),
            message: input.to_string(),
            suggestion: "Check for nil before dereferencing. Use `if ptr != nil` before accessing pointer fields.".to_string(),
        });
    }

    if input.contains("undefined:") {
        return Some(ParsedError {
            error_type: "Go Undefined Symbol".to_string(),
            message: input.to_string(),
            suggestion: "Check imports and package names. Run `go mod tidy` to fix missing dependencies.".to_string(),
        });
    }

    if input.contains("cannot use") && input.contains("as type") {
        return Some(ParsedError {
            error_type: "Go Type Mismatch".to_string(),
            message: input.to_string(),
            suggestion: "You're passing the wrong type. Check the expected type and cast if necessary.".to_string(),
        });
    }

    if input.contains("go: no module provides") {
        return Some(ParsedError {
            error_type: "Go Missing Module".to_string(),
            message: input.to_string(),
            suggestion: "Run `go get <module>` to install the missing dependency, then `go mod tidy`.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_out_of_range() {
        let input = "panic: runtime error: index out of range [5] with length 3";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Go Runtime Panic");
    }

    #[test]
    fn test_nil_pointer() {
        let input = "panic: nil pointer dereference";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Go Nil Pointer");
    }

    #[test]
    fn test_missing_module() {
        let input = "go: no module provides github.com/some/pkg";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Go Missing Module");
    }

    #[test]
    fn test_no_match() {
        let input = "Everything is fine";
        assert!(parse(input).is_none());
    }
}