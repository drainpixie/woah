use std::path::PathBuf;

use git2::{build::CheckoutBuilder, Repository};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::log;

pub static GIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?:\/\/|(?:ssh:\/\/)?git@|git:\/\/)?(?:www\.)?([\w.-]+)[/:]([^/]+)\/([^/.]+)(?:\.git)?").unwrap()
});

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

    pub fn from_url(url: &str) -> Result<Self, String> {
        GIT_REGEX
            .captures(url)
            .map(|captures| Self::new(url, &captures[1], &captures[2], &captures[3]))
            .ok_or_else(|| "Invalid URL".to_string())
    }
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
        let cases = [
            RepositoryData::from_url("https://github.com/username/repository.git"),
            RepositoryData::from_url("http://github.com/username/repository"),
            RepositoryData::from_url("git@github.com:username/repository.git"),
            RepositoryData::from_url("ssh://git@github.com/username/repository"),
            RepositoryData::from_url("git://github.com/username/repository"),
            RepositoryData::from_url("https://bitbucket.org/username/repository.git"),
            RepositoryData::from_url("git@gitlab.com:username/repository.git"),
            RepositoryData::from_url("https://example.com/username/repository"),
            RepositoryData::from_url("git@example.com:username/repository.git"),
            RepositoryData::from_url("ssh://git@example.com/username/repository"),
            RepositoryData::from_url("https://github.com/username/repository.git/extra"),
        ];

        for case in cases {
            assert!(case.is_ok(), "Expected a valid URL: {:?}", case);
        }

        assert!(RepositoryData::from_url("https://github.com").is_err(), "Expected an invalid URL");
        assert!(RepositoryData::from_url("ssh://github.com").is_err(), "Expected an invalid URL");
        assert!(RepositoryData::from_url("git@github.com").is_err(), "Expected an invalid URL");
    }
}
