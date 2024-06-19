use clap::{arg, command, Command};
use git2::Repository;

use crate::{
    config::PROJECT_DIRS,
    git::{self, RepositoryData},
    log,
};

pub fn create_command() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("install")
                .about("Install a template")
                .arg(arg!(<URL>).value_parser(RepositoryData::from_url)),
        )
        .subcommand(
            command!("update")
                .about("Update templates")
                .arg(arg!([NAME])),
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

    match Repository::clone(&repo.url, data_dir.join(repo.repository.to_string())) {
        Ok(_) => log::success("install", "cloned repository"),
        Err(e) => log::error("install", &format!("failed to clone repository: {}", e)),
    }
}

pub fn update(name: Option<&String>) {
    let data_dir = PROJECT_DIRS.data_dir();
    let failed = |e| log::error("update", &format!("failed to update repository: {}", e));

    if let Some(name) = name {
        let path = data_dir.join(name);
        git::update_repository(name, &path).unwrap_or_else(failed);
    } else {
        std::fs::read_dir(data_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .for_each(|path| {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    git::update_repository(name, &path).unwrap_or_else(failed)
                }
            });
    }
}
