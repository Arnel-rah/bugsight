use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("NullPointerException") {
        return Some(ParsedError {
            error_type: "Java NullPointerException".to_string(),
            message: input.to_string(),
            suggestion: "An object is null. Add a null check with `if (obj != null)` or use `Optional<T>` to handle null values safely.".to_string(),
        });
    }

    if input.contains("ArrayIndexOutOfBoundsException") {
        return Some(ParsedError {
            error_type: "Java Array Out Of Bounds".to_string(),
            message: input.to_string(),
            suggestion: "Index exceeds array length. Check array size with `.length` before accessing elements.".to_string(),
        });
    }

    if input.contains("ClassNotFoundException") {
        return Some(ParsedError {
            error_type: "Java Class Not Found".to_string(),
            message: input.to_string(),
            suggestion: "The class is missing from classpath. Check your dependencies in pom.xml or build.gradle and run `mvn install` or `gradle build`.".to_string(),
        });
    }

    if input.contains("OutOfMemoryError") {
        return Some(ParsedError {
            error_type: "Java Out Of Memory".to_string(),
            message: input.to_string(),
            suggestion: "JVM ran out of memory. Increase heap size with `-Xmx512m` or `-Xmx1g` flag when running your app.".to_string(),
        });
    }

    if input.contains("StackOverflowError") {
        return Some(ParsedError {
            error_type: "Java Stack Overflow".to_string(),
            message: input.to_string(),
            suggestion:
                "Infinite recursion detected. Check your recursive methods for a proper base case."
                    .to_string(),
        });
    }

    if input.contains("ClassCastException") {
        return Some(ParsedError {
            error_type: "Java Class Cast Error".to_string(),
            message: input.to_string(),
            suggestion: "Invalid type cast. Use `instanceof` to check the type before casting."
                .to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pointer() {
        let input = "Exception in thread 'main' java.lang.NullPointerException";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Java NullPointerException");
        assert!(result.suggestion.contains("null check"));
    }

    #[test]
    fn test_array_out_of_bounds() {
        let input = "java.lang.ArrayIndexOutOfBoundsException: Index 5 out of bounds for length 3";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Java Array Out Of Bounds");
    }

    #[test]
    fn test_out_of_memory() {
        let input = "java.lang.OutOfMemoryError: Java heap space";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Java Out Of Memory");
        assert!(result.suggestion.contains("-Xmx"));
    }

    #[test]
    fn test_stack_overflow() {
        let input = "java.lang.StackOverflowError";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Java Stack Overflow");
    }

    #[test]
    fn test_no_match() {
        let input = "Build successful";
        assert!(parse(input).is_none());
    }
}
