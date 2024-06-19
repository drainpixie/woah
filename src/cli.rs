use clap::{arg, command, Command};
use once_cell::sync::Lazy;
use regex::Regex;

type Git = (Box<str>, Box<str>, Box<str>);

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

pub fn install(url: Git) {
    
}

// TODO: Find a more proper way to make clap handle string slices
fn extract_git(url: &str) -> Result<Git, String> {
    if let Some(captures) = GIT_REGEX.captures(url) {
        let [host, username, repository] = captures
            .iter()
            .skip(1)
            .map(|x| Box::from(x.unwrap().as_str()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok((host, username, repository))
    } else {
        Err("Invalid URL".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_git() {
        #[rustfmt::skip]
        let cases = vec![
            ("https://github.com/username/repository.git", ("github.com", "username", "repository")),
            ("http://github.com/username/repository", ("github.com", "username", "repository")),
            ("git@github.com:username/repository.git", ("github.com", "username", "repository")),
            ("ssh://git@github.com/username/repository", ("github.com", "username", "repository")),
            ("git://github.com/username/repository", ("github.com", "username", "repository")),
            ("https://bitbucket.org/username/repository.git", ("bitbucket.org", "username", "repository")),
            ("git@gitlab.com:username/repository.git", ("gitlab.com", "username", "repository")),
            ("https://example.com/username/repository", ("example.com", "username", "repository")),
            ("git@example.com:username/repository.git", ("example.com", "username", "repository")),
            ("ssh://git@example.com/username/repository", ("example.com", "username", "repository")),
            ("https://github.com/username/repository.git/extra", ("github.com", "username", "repository")),
        ];

        #[rustfmt::skip]
        let icases = [
            "https://github.com",
            "ssh://github.com",
            "git@github.com"
        ];

        for (url, (host, username, repository)) in cases {
            let expected: Git = (host.into(), username.into(), repository.into());
            assert_eq!(extract_git(url), Ok(expected), "Failed on URL: {}", url);
        }

        for url in icases {
            assert_eq!(
                extract_git(url),
                Err("Invalid URL".to_string()),
                "Failed on URL: {}",
                url
            );
        }
    }
}
