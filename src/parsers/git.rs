use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("CONFLICT") {
        return Some(ParsedError {
            error_type: "Git Merge Conflict".to_string(),
            message: input.to_string(),
            suggestion: "Open the conflicting files, resolve the `<<<<<<`, `=======`, `>>>>>>>` markers, then run `git add .` and `git commit`.".to_string(),
        });
    }

    if input.contains("rejected") && input.contains("non-fast-forward") {
        return Some(ParsedError {
            error_type: "Git Push Rejected".to_string(),
            message: input.to_string(),
            suggestion: "Run `git pull --rebase origin main` to sync with remote, then push again."
                .to_string(),
        });
    }

    if input.contains("does not exist") && input.contains("pathspec") {
        return Some(ParsedError {
            error_type: "Git Branch Not Found".to_string(),
            message: input.to_string(),
            suggestion: "Check branch name with `git branch -a`. Use `git checkout -b <branch>` to create a new one.".to_string(),
        });
    }

    if input.contains("Your local changes to the following files would be overwritten") {
        return Some(ParsedError {
            error_type: "Git Uncommitted Changes".to_string(),
            message: input.to_string(),
            suggestion: "Save your changes with `git stash`, then pull: `git pull`. Restore with `git stash pop`.".to_string(),
        });
    }

    if input.contains("Permission denied (publickey)") {
        return Some(ParsedError {
            error_type: "Git SSH Permission Denied".to_string(),
            message: input.to_string(),
            suggestion: "Your SSH key is not set up. Run `ssh-keygen -t ed25519` and add the public key to GitHub Settings → SSH Keys.".to_string(),
        });
    }

    if input.contains("nothing to commit") {
        return Some(ParsedError {
            error_type: "Git Nothing To Commit".to_string(),
            message: input.to_string(),
            suggestion: "No changes detected. Run `git status` to see the state of your repo. Use `git add .` to stage changes first.".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_conflict() {
        let input = "CONFLICT (content): Merge conflict in src/main.rs";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Git Merge Conflict");
        assert!(result.suggestion.contains("git add"));
    }

    #[test]
    fn test_push_rejected() {
        let input = "! [rejected] main -> main (non-fast-forward)";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Git Push Rejected");
        assert!(result.suggestion.contains("git pull --rebase"));
    }

    #[test]
    fn test_ssh_permission_denied() {
        let input = "Permission denied (publickey).";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Git SSH Permission Denied");
        assert!(result.suggestion.contains("ssh-keygen"));
    }

    #[test]
    fn test_uncommitted_changes() {
        let input = "Your local changes to the following files would be overwritten by merge";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Git Uncommitted Changes");
        assert!(result.suggestion.contains("git stash"));
    }

    #[test]
    fn test_no_match() {
        let input = "Already up to date.";
        assert!(parse(input).is_none());
    }
}
