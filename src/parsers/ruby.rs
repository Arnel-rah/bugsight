use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("NoMethodError") {
        return Some(ParsedError {
            error_type: "Ruby NoMethodError".to_string(),
            message: input.to_string(),
            suggestion: "Method doesn't exist on this object. Check for typos or verify the object type with `.class` before calling the method.".to_string(),
        });
    }

    if input.contains("NameError: uninitialized constant") {
        return Some(ParsedError {
            error_type: "Ruby Uninitialized Constant".to_string(),
            message: input.to_string(),
            suggestion: "Class or module not found. Check your `require` statements or verify the constant name and its file location.".to_string(),
        });
    }

    if input.contains("LoadError: cannot load such file") {
        return Some(ParsedError {
            error_type: "Ruby Load Error".to_string(),
            message: input.to_string(),
            suggestion: "File or gem not found. Run `bundle install` to install missing gems or check the file path.".to_string(),
        });
    }

    if input.contains("SyntaxError") {
        return Some(ParsedError {
            error_type: "Ruby Syntax Error".to_string(),
            message: input.to_string(),
            suggestion: "Syntax error in your Ruby code. Check for missing `end`, `do`, or mismatched brackets on the indicated line.".to_string(),
        });
    }

    if input.contains("TypeError") && input.contains("rb") {
        return Some(ParsedError {
            error_type: "Ruby Type Error".to_string(),
            message: input.to_string(),
            suggestion:
                "Wrong type passed. Use `.to_s`, `.to_i`, or `.to_f` to convert types explicitly."
                    .to_string(),
        });
    }

    if input.contains("ActiveRecord") && input.contains("RecordNotFound") {
        return Some(ParsedError {
            error_type: "Rails Record Not Found".to_string(),
            message: input.to_string(),
            suggestion: "Database record not found. Use `find_by` instead of `find` to return nil instead of raising an error.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_method_error() {
        let input = "NoMethodError: undefined method 'name' for nil:NilClass";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Ruby NoMethodError");
    }

    #[test]
    fn test_load_error() {
        let input = "LoadError: cannot load such file -- devise";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Ruby Load Error");
        assert!(result.suggestion.contains("bundle install"));
    }

    #[test]
    fn test_uninitialized_constant() {
        let input = "NameError: uninitialized constant UserController";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Ruby Uninitialized Constant");
    }

    #[test]
    fn test_rails_record_not_found() {
        let input = "ActiveRecord::RecordNotFound: Couldn't find User with id=99";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Rails Record Not Found");
        assert!(result.suggestion.contains("find_by"));
    }

    #[test]
    fn test_no_match() {
        let input = "Server started on port 3000";
        assert!(parse(input).is_none());
    }
}
