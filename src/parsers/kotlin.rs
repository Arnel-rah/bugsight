use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("NullPointerException") && input.contains("kotlin") {
        return Some(ParsedError {
            error_type: "Kotlin NullPointerException".to_string(),
            message: input.to_string(),
            suggestion: "Use Kotlin's null safety — replace `String` with `String?` and use `?.` safe call operator or `?:` Elvis operator to handle nulls.".to_string(),
        });
    }

    if input.contains("error: unresolved reference") {
        return Some(ParsedError {
            error_type: "Kotlin Unresolved Reference".to_string(),
            message: input.to_string(),
            suggestion: "Symbol not found. Check imports, verify the class/function exists, and ensure dependencies are added in build.gradle.".to_string(),
        });
    }

    if input.contains("error: type mismatch") {
        return Some(ParsedError {
            error_type: "Kotlin Type Mismatch".to_string(),
            message: input.to_string(),
            suggestion: "Type mismatch. Use `.toString()`, `.toInt()`, `.toDouble()` for conversions, or check your function return types.".to_string(),
        });
    }

    if input.contains("ClassCastException") && input.contains("kotlin") {
        return Some(ParsedError {
            error_type: "Kotlin ClassCastException".to_string(),
            message: input.to_string(),
            suggestion: "Invalid cast. Use `as?` for safe casting instead of `as`. Example: `val x = obj as? MyClass` returns null if cast fails.".to_string(),
        });
    }

    if input.contains(
        "error: none of the following functions can be called with the arguments supplied",
    ) {
        return Some(ParsedError {
            error_type: "Kotlin Wrong Arguments".to_string(),
            message: input.to_string(),
            suggestion: "Wrong argument types passed to function. Check the function signature and make sure argument types match exactly.".to_string(),
        });
    }

    if input.contains("OutOfMemoryError") && input.contains("kotlin") {
        return Some(ParsedError {
            error_type: "Kotlin OutOfMemoryError".to_string(),
            message: input.to_string(),
            suggestion: "JVM ran out of memory. Increase heap size with `-Xmx512m` in your run configuration, or optimize memory usage in your code.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unresolved_reference() {
        let input = "error: unresolved reference: myFunction";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Kotlin Unresolved Reference");
        assert!(result.suggestion.contains("build.gradle"));
    }

    #[test]
    fn test_type_mismatch() {
        let input = "error: type mismatch: inferred type is String but Int was expected";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Kotlin Type Mismatch");
    }

    #[test]
    fn test_class_cast() {
        let input = "kotlin.TypeCastException: ClassCastException null cannot be cast to non-null type kotlin.String";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Kotlin ClassCastException");
        assert!(result.suggestion.contains("as?"));
    }

    #[test]
    fn test_no_match() {
        let input = "BUILD SUCCESSFUL in 3s";
        assert!(parse(input).is_none());
    }
}
