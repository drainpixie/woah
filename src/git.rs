use once_cell::sync::Lazy;
use regex::Regex;
use std::{path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub struct Repository {
    pub url: Box<str>,
    pub name: Box<str>,
    pub username: Box<str>,
}

pub static GIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?:\/\/|(?:ssh:\/\/)?git@|git:\/\/)?(?:www\.)?([\w.-]+)[/:]([^/]+)\/([^/.]+)(?:\.git)?").unwrap()
});

pub fn clone(url: &str, path: PathBuf) -> Result<(), String> {
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .output()
        .map_err(|e| format!("failed to execute git: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn pull(path: PathBuf) -> Result<(), String> {
    let output = Command::new("git")
        .current_dir(path)
        .arg("pull")
        .output()
        .map_err(|e| format!("failed to execute git: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn from_url(url: &str) -> Result<Repository, String> {
    GIT_REGEX
        .captures(url)
        .map(|captures| {
            Ok(Repository {
                url: url.into(),
                username: captures[2].into(),
                name: captures[3].into(),
            })
        })
        .unwrap_or_else(|| Err("Invalid URL".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_git() {
        let cases = [
            from_url("https://github.com/username/repository.git"),
            from_url("http://github.com/username/repository"),
            from_url("git@github.com:username/repository.git"),
            from_url("ssh://git@github.com/username/repository"),
            from_url("git://github.com/username/repository"),
            from_url("https://bitbucket.org/username/repository.git"),
            from_url("git@gitlab.com:username/repository.git"),
            from_url("https://example.com/username/repository"),
            from_url("git@example.com:username/repository.git"),
            from_url("ssh://git@example.com/username/repository"),
            from_url("https://github.com/username/repository.git/extra"),
        ];

        for case in cases {
            assert!(case.is_ok(), "Expected a valid URL: {:?}", case);
        }

        assert!(
            from_url("https://github.com").is_err(),
            "Expected an invalid URL"
        );
        assert!(
            from_url("ssh://github.com").is_err(),
            "Expected an invalid URL"
        );
        assert!(
            from_url("git@github.com").is_err(),
            "Expected an invalid URL"
        );
    }
}
