# 🤝 Contributing to bugsight

First off, thanks for taking the time to contribute! 🎉

---

## 🚀 How to contribute

### 1. Fork & clone
```bash
git clone https://github.com/Arnel-rah/bugsight
cd bugsight
```

### 2. Create a branch
```bash
git checkout -b feat/your-feature
```

### 3. Make your changes

### 4. Run tests
```bash
cargo test
```

### 5. Commit & push
```bash
git commit -m "feat: add your feature"
git push origin feat/your-feature
```

### 6. Open a Pull Request on GitHub

---

## 🧩 How to add a new parser

The easiest way to contribute is to add error patterns for a new language.

1. Create `src/parsers/YOUR_LANG.rs`
2. Implement the `parse(input: &str) -> Option<ParsedError>` function
3. Register it in `src/parsers/mod.rs`
4. Add tests in the same file

Example for a new language:
```rust
use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("YOUR_ERROR_PATTERN") {
        return Some(ParsedError {
            error_type: "Your Error Type".to_string(),
            message: input.to_string(),
            suggestion: "Your fix suggestion.".to_string(),
        });
    }
    None
}
```

---

## 📋 Commit convention

| Prefix | Usage |
|--------|-------|
| `feat` | new feature |
| `fix` | bug fix |
| `docs` | documentation |
| `test` | adding tests |
| `chore` | config, dependencies |
| `refactor` | code refactoring |

---

## 💡 Ideas for contributions

- Add parsers for Go, Python, Node.js, Docker...
- Improve existing suggestions
- Add more test cases
- Improve CLI output
