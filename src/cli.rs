use std::fs::create_dir;

use clap::{arg, command, Command};
use git2::Repository;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{config::PROJECT_DIRS, log};

#[derive(Debug, Clone, PartialEq)]
pub struct RepositoryData {
    url: Box<str>,

    host: Box<str>,
    username: Box<str>,
    repository: Box<str>,
}

impl RepositoryData {
    fn new(url: &str, host: &str, username: &str, repository: &str) -> Self {
        Self {
            url: url.into(),
            host: host.into(),
            username: username.into(),
            repository: repository.into(),
        }
    }
}

pub static GIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?:\/\/|(?:ssh:\/\/)?git@|git:\/\/)?(?:www\.)?([\w.-]+)[/:]([^/]+)\/([^/.]+)(?:\.git)?").unwrap()
});

pub fn create_command() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("install")
                .about("Install a template")
                .arg(arg!([URL]).value_parser(extract_git)),
        )
        .subcommand(
            command!("update")
                .about("Update templates")
                .arg(arg!(<NAME>)),
        )
}

pub fn install(repo: &RepositoryData) {
    let data_dir = PROJECT_DIRS.data_dir();

    log::info(
        "install",
        &format!(
            "installing template {} by {}",
            repo.repository, repo.username
        ),
    );

    log::info("install", &format!("directory is {}", data_dir.display()));

    match Repository::clone(&repo.url, &data_dir.join(repo.repository.to_string())) {
        Ok(_) => log::success("install", "cloned repository"),
        Err(e) => log::error("install", &format!("failed to clone repository: {}", e)),
    }
}

fn extract_git(url: &str) -> Result<RepositoryData, String> {
    match GIT_REGEX.captures(url) {
        Some(captures) => Ok(RepositoryData::new(
            url,
            &captures[1],
            &captures[2],
            &captures[3],
        )),
        None => Err("Invalid URL".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_git() {
        #[rustfmt::skip]
                let cases = [
                    ("https://github.com/username/repository.git", RepositoryData::new("https://github.com/username/repository.git", "github.com", "username", "repository")),
                    ("http://github.com/username/repository", RepositoryData::new("http://github.com/username/repository", "github.com", "username", "repository")),
                    ("git@github.com:username/repository.git", RepositoryData::new("git@github.com:username/repository.git", "github.com", "username", "repository")),
                    ("ssh://git@github.com/username/repository", RepositoryData::new("ssh://git@github.com/username/repository", "github.com", "username", "repository")),
                    ("git://github.com/username/repository", RepositoryData::new("git://github.com/username/repository", "github.com", "username", "repository")),
                    ("https://bitbucket.org/username/repository.git", RepositoryData::new("https://bitbucket.org/username/repository.git", "bitbucket.org", "username", "repository")),
                    ("git@gitlab.com:username/repository.git", RepositoryData::new("git@gitlab.com:username/repository.git", "gitlab.com", "username", "repository")),
                    ("https://example.com/username/repository", RepositoryData::new("https://example.com/username/repository", "example.com", "username", "repository")),
                    ("git@example.com:username/repository.git", RepositoryData::new("git@example.com:username/repository.git", "example.com", "username", "repository")),
                    ("ssh://git@example.com/username/repository", RepositoryData::new("ssh://git@example.com/username/repository", "example.com", "username", "repository")),
                    ("https://github.com/username/repository.git/extra", RepositoryData::new("https://github.com/username/repository.git/extra", "github.com", "username", "repository")),
                ];

        let icases = ["https://github.com", "ssh://github.com", "git@github.com"];

        for (url, expected) in cases {
            assert_eq!(extract_git(url), Ok(expected), "Failed on URL: {}", url);
        }

        for url in icases {
            assert!(
                extract_git(url).is_err(),
                "Expected an error for URL: {}",
                url
            );
        }
    }
}
