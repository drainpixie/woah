use std::path::PathBuf;

use git2::{build::CheckoutBuilder, Repository};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::log;

#[derive(Debug, Clone, PartialEq)]
pub struct RepositoryData {
    pub url: Box<str>,
    pub host: Box<str>,
    pub username: Box<str>,
    pub repository: Box<str>,
}

impl RepositoryData {
    pub fn new(url: &str, host: &str, username: &str, repository: &str) -> Self {
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

pub fn extract_url(url: &str) -> Result<RepositoryData, String> {
    GIT_REGEX
        .captures(url)
        .map(|captures| RepositoryData::new(url, &captures[1], &captures[2], &captures[3]))
        .ok_or_else(|| "Invalid URL".to_string())
}

pub fn update_repository(name: &str, path: &PathBuf) -> Result<(), String> {
    let repo = Repository::open(path).map_err(|e| format!("failed to open repository: {}", e))?;

    log::info(name, &format!("updating repository {}", name));

    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| format!("couldn't find remote 'origin': {}", e))?;

    remote
        .fetch(&["main"], None, None)
        .map_err(|e| format!("failed to fetch remote 'origin': {}", e))?;

    let fetch_head = repo
        .find_reference("FETCH_HEAD")
        .map_err(|e| format!("couldn't find FETCH_HEAD: {}", e))?;

    let fetch_commit = repo
        .reference_to_annotated_commit(&fetch_head)
        .map_err(|e| format!("couldn't find commit: {}", e))?;

    let analysis = repo
        .merge_analysis(&[&fetch_commit])
        .map_err(|e| format!("couldn't find merge analysis: {}", e))?;

    if analysis.0.is_up_to_date() {
        log::info(name, "repository is already up to date");
    } else if analysis.0.is_fast_forward() {
        log::info(name, "repository is fast-forwardable");

        let refname = "refs/heads/main";
        let mut reference = repo
            .find_reference(refname)
            .map_err(|e| format!("couldn't find reference: {}", e))?;

        reference
            .set_target(fetch_commit.id(), "fast-forward")
            .map_err(|e| format!("failed to fast-forward repository: {}", e))?;

        repo.set_head(refname)
            .map_err(|e| format!("failed to set HEAD to main: {}", e))?;
        repo.checkout_head(Some(CheckoutBuilder::default().force()))
            .map_err(|e| format!("failed to checkout: {}", e))?;

        log::success(name, "repository updated successfully");
    }

    Ok(())
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

        #[rustfmt::skip]
        let invalid_cases = [
            "https://github.com",
            "ssh://github.com",
            "git@github.com",
        ];

        for (url, expected) in cases {
            assert_eq!(extract_url(url), Ok(expected), "Failed on URL: {}", url);
        }

        for url in invalid_cases {
            assert!(
                extract_url(url).is_err(),
                "Expected an error for URL: {}",
                url
            );
        }
    }
}
