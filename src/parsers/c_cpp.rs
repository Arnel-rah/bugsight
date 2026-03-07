use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("segmentation fault") || input.contains("Segmentation fault") {
        return Some(ParsedError {
            error_type: "Segmentation Fault".to_string(),
            message: input.to_string(),
            suggestion: "You're accessing invalid memory. Check for null pointers, array out of bounds, or use-after-free. Run with `valgrind` or `gdb` to find the exact location.".to_string(),
        });
    }

    if input.contains("undefined reference to") {
        return Some(ParsedError {
            error_type: "Linker Error".to_string(),
            message: input.to_string(),
            suggestion: "A function or symbol is declared but not defined. Check that all source files are compiled and linked. Add missing `-l` flags for libraries.".to_string(),
        });
    }

    if input.contains("double free") || input.contains("free(): invalid pointer") {
        return Some(ParsedError {
            error_type: "Double Free Error".to_string(),
            message: input.to_string(),
            suggestion: "You're freeing the same memory twice. Set pointers to NULL after freeing: `free(ptr); ptr = NULL;`. Use valgrind to detect memory issues.".to_string(),
        });
    }

    if input.contains("heap buffer overflow") || input.contains("stack buffer overflow") {
        return Some(ParsedError {
            error_type: "Buffer Overflow".to_string(),
            message: input.to_string(),
            suggestion: "Writing beyond allocated buffer bounds. Check array sizes and use safer functions like `strncpy` instead of `strcpy`, `snprintf` instead of `sprintf`.".to_string(),
        });
    }

    if input.contains("memory leak") || input.contains("definitely lost") {
        return Some(ParsedError {
            error_type: "Memory Leak".to_string(),
            message: input.to_string(),
            suggestion: "Allocated memory was never freed. Make sure every `malloc`/`new` has a corresponding `free`/`delete`. Use valgrind: `valgrind --leak-check=full ./program`.".to_string(),
        });
    }

    if input.contains("undefined behavior")
        || input.contains("runtime error: signed integer overflow")
    {
        return Some(ParsedError {
            error_type: "Undefined Behavior".to_string(),
            message: input.to_string(),
            suggestion: "Undefined behavior detected. Compile with `-fsanitize=undefined` to catch these at runtime. Common causes: integer overflow, null dereference, uninitialized variables.".to_string(),
        });
    }

    if input.contains("core dumped") {
        return Some(ParsedError {
            error_type: "Core Dump".to_string(),
            message: input.to_string(),
            suggestion: "Program crashed and dumped core. Debug with: `gdb ./program core`. Compile with `-g` flag for better debug info.".to_string(),
        });
    }

    if input.contains("implicit declaration of function") {
        return Some(ParsedError {
            error_type: "C Implicit Declaration".to_string(),
            message: input.to_string(),
            suggestion: "Function used before declaration. Add the correct `#include` header or declare the function prototype before using it.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segfault() {
        let input = "Segmentation fault (core dumped)";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Segmentation Fault");
        assert!(result.suggestion.contains("valgrind"));
    }

    #[test]
    fn test_undefined_reference() {
        let input = "undefined reference to `main'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Linker Error");
    }

    #[test]
    fn test_double_free() {
        let input = "double free or corruption (out)";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Double Free Error");
        assert!(result.suggestion.contains("NULL"));
    }

    #[test]
    fn test_memory_leak() {
        let input = "definitely lost: 40 bytes in 1 blocks";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Memory Leak");
        assert!(result.suggestion.contains("valgrind"));
    }

    #[test]
    fn test_core_dump() {
        let input = "Aborted (core dumped)";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Core Dump");
        assert!(result.suggestion.contains("gdb"));
    }

    #[test]
    fn test_implicit_declaration() {
        let input = "warning: implicit declaration of function 'printf'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "C Implicit Declaration");
    }

    #[test]
    fn test_no_match() {
        let input = "Build successful";
        assert!(parse(input).is_none());
    }
}
