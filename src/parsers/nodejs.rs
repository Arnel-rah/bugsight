use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("Cannot find module") {
        return Some(ParsedError {
            error_type: "Node.js Missing Module".to_string(),
            message: input.to_string(),
            suggestion:
                "Run `npm install` or `npm install <module>` to install missing dependencies."
                    .to_string(),
        });
    }

    if input.contains("TypeError: Cannot read properties of undefined")
        || input.contains("TypeError: Cannot read property")
    {
        return Some(ParsedError {
            error_type: "Node.js Undefined Property".to_string(),
            message: input.to_string(),
            suggestion: "The object is undefined. Use optional chaining `obj?.property` or check with `if (obj)` before accessing.".to_string(),
        });
    }

    if input.contains("SyntaxError: Unexpected token") {
        return Some(ParsedError {
            error_type: "Node.js Syntax Error".to_string(),
            message: input.to_string(),
            suggestion: "Check for missing brackets, commas, or quotes. Run `node --check file.js` to validate syntax.".to_string(),
        });
    }

    if input.contains("EADDRINUSE") {
        return Some(ParsedError {
            error_type: "Node.js Port In Use".to_string(),
            message: input.to_string(),
            suggestion: "The port is already in use. Run `lsof -i :<port>` to find the process and `kill -9 <PID>` to stop it.".to_string(),
        });
    }

    if input.contains("UnhandledPromiseRejection") {
        return Some(ParsedError {
            error_type: "Node.js Unhandled Promise".to_string(),
            message: input.to_string(),
            suggestion:
                "Add `.catch()` to your promise or use `try/catch` inside an `async` function."
                    .to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cannot_find_module() {
        let input = "Error: Cannot find module 'express'";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Node.js Missing Module");
        assert!(result.suggestion.contains("npm install"));
    }

    #[test]
    fn test_undefined_property() {
        let input = "TypeError: Cannot read properties of undefined (reading 'name')";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Node.js Undefined Property");
        assert!(result.suggestion.contains("?."));
    }

    #[test]
    fn test_port_in_use() {
        let input = "Error: EADDRINUSE: address already in use :::3000";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Node.js Port In Use");
    }

    #[test]
    fn test_no_match() {
        let input = "Server started successfully";
        assert!(parse(input).is_none());
    }
}
