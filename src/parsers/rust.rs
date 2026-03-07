use super::ParsedError;
use regex::Regex;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("panicked at") {
        let msg = extract_panic_message(input);
        return Some(ParsedError {
            error_type: "Runtime Panic".to_string(),
            message: msg.clone(),
            suggestion: suggest_for_panic(&msg),
        });
    }
    if input.contains("called `Option::unwrap()` on a `None` value")
        || input.contains("called `Result::unwrap()` on an `Err` value")
    {
        return Some(ParsedError {
            error_type: "Unwrap Error".to_string(),
            message: input.to_string(),
            suggestion: "Replace `.unwrap()` with `.unwrap_or()`, `.expect(\"msg\")`, or proper error handling with `?`.".to_string(),
        });
    }

    if input.contains("error[E") {
        return Some(ParsedError {
            error_type: "Compile Error".to_string(),
            message: input.to_string(),
            suggestion: "Run `rustc --explain Exxxx` for details.".to_string(),
        });
    }

    None
}

fn extract_panic_message(input: &str) -> String {
    let re = Regex::new(r"panicked at '([^']+)'").unwrap();
    re.captures(input)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| input.to_string())
}

fn suggest_for_panic(msg: &str) -> String {
    if msg.contains("index out of bounds") {
        return "Check your array/vector length before indexing. Use `.get(i)` instead of `[i]` to avoid panics.".to_string();
    }
    if msg.contains("unwrap()") || msg.contains("called `Option::unwrap()`") {
        return "Replace `.unwrap()` with `.unwrap_or()`, `.expect()`, or proper error handling with `?`.".to_string();
    }
    "Check the stack trace above for the exact location of the panic.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_out_of_bounds() {
        let input =
            "thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Runtime Panic");
        assert!(result.suggestion.contains(".get(i)"));
    }

    #[test]
    fn test_unwrap_on_none() {
        let input = "called `Option::unwrap()` on a `None` value";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Unwrap Error");
        assert!(result.suggestion.contains(".unwrap_or()"));
    }

    #[test]
    fn test_compile_error() {
        let input = "error[E0382]: borrow of moved value";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Compile Error");
        assert!(result.suggestion.contains("rustc --explain"));
    }

    #[test]
    fn test_no_match_returns_none() {
        let input = "Everything is fine here";
        let result = parse(input);
        assert!(result.is_none());
    }
}
