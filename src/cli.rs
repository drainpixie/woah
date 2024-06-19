use clap::{arg, command, value_parser, Command};
use once_cell::sync::Lazy;
use regex::Regex;

pub static GIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    let pattern = r"(?:https:\/\/|http:\/\/|git@|ssh:\/\/git@|git:\/\/)?(?:www\.)?([a-zA-Z0-9.-]+)[/:]([^/]+)\/([^/.]+)(?:\.git)?";
    Regex::new(pattern).unwrap()
});

fn try_parse_git_url(url: &str) -> Result<(String, String, String), String> {
    if let Some(captures) = GIT_REGEX.captures(url) {
        let host = captures.get(1).unwrap().as_str().to_string();
        let username = captures.get(2).unwrap().as_str().to_string();
        let repository = captures.get(3).unwrap().as_str().to_string();

        Ok((host, username, repository))
    } else {
        Err("Invalid URL".to_string())
    }
}

pub fn create_command() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("install")
                .about("Install a template")
                .arg(arg!([URL]).value_parser(try_parse_git_url)),
        )
        .subcommand(
            command!("update")
                .about("Update templates")
                .arg(arg!(<NAME>)),
        )
}

pub fn install(url: String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_url() {
        #[rustfmt::skip]
        let valid_cases = vec![
            ("https://github.com/username/repository.git", Ok(("github.com", "username", "repository"))),
            ("http://github.com/username/repository", Ok(("github.com", "username", "repository"))),
            ("git@github.com:username/repository.git", Ok(("github.com", "username", "repository"))),
            ("ssh://git@github.com/username/repository", Ok(("github.com", "username", "repository"))),
            ("git://github.com/username/repository", Ok(("github.com", "username", "repository"))),
            ("https://bitbucket.org/username/repository.git", Ok(("bitbucket.org", "username", "repository"))),
            ("git@gitlab.com:username/repository.git", Ok(("gitlab.com", "username", "repository"))),
            ("https://example.com/username/repository", Ok(("example.com", "username", "repository"))),
            ("git@example.com:username/repository.git", Ok(("example.com", "username", "repository"))),
            ("ssh://git@example.com/username/repository", Ok(("example.com", "username", "repository"))),
            ("https://github.com/username/repository.git/extra", Ok(("github.com", "username", "repository"))),
        ];

        for (url, expected) in valid_cases {
            let expected = expected.map(|(host, username, repository)| {
                (
                    host.to_string(),
                    username.to_string(),
                    repository.to_string(),
                )
            });

            assert_eq!(try_parse_git_url(url), expected, "Failed on URL: {}", url);
        }

        let invalid_cases = vec!["https://github.com", "ssh://github.com", "git@github.com"];

        for url in invalid_cases {
            assert_eq!(
                try_parse_git_url(url),
                Err("Invalid URL".to_string()),
                "Failed on URL: {}",
                url
            );
        }
    }
}
