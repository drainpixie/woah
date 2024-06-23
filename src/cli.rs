use std::{io, path::Path};

use clap::{arg, command, Command};

use crate::{
    config::PROJECT_DIRS,
    git::{self, Repository},
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
                .arg(arg!(<URL>).value_parser(git::from_url)),
        )
        .subcommand(
            command!("update")
                .about("Update templates")
                .arg(arg!([NAME])),
        )
}

pub fn install(repo: &Repository) {
    log::info(
        "install",
        &format!("installing template {} by {}", repo.name, repo.username),
    );

    git::clone(&repo.url, PROJECT_DIRS.data_dir().join(repo.name.as_ref()))
        .map(|_| log::success(&repo.name, "installed"))
        .map_err(|e| log::error(&repo.name, &e))
        .ok();
}

pub fn update(name: Option<&String>) {
    let dir = PROJECT_DIRS.data_dir();
    let update_repo = |name: &str| {
        git::pull(dir.join(name))
            .map(|_| log::success(name, "updated"))
            .map_err(|e| log::error(name, &e))
    };

    match name {
        Some(name) => {
            update_repo(name).ok();
        }
        None => {
            let directories = descend(PROJECT_DIRS.data_dir(), 0)
                .map_err(|e| log::error("update", &e.to_string()))
                .unwrap();

            for directory in directories {
                // NOTE: We could just pass &directory but then ugly logs
                let sep: &str = std::path::MAIN_SEPARATOR_STR;
                let name = directory
                    .split('/')
                    .rev()
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join(sep);

                update_repo(&name).ok();
            }
        }
    }
}

fn descend(dir: &Path, depth: usize) -> Result<Vec<String>, io::Error> {
    fn inner(dir: &Path, depth: usize, subdirs: &mut Vec<String>) -> Result<(), io::Error> {
        if depth >= 2 {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    if depth == 0 {
                        inner(&path, depth + 1, subdirs)?;
                        continue;
                    }

                    subdirs.push(path_str.to_string());
                    inner(&path, depth + 1, subdirs)?;
                }
            }
        }

        Ok(())
    }

    let mut subdirs = Vec::new();
    inner(dir, depth, &mut subdirs)?;

    Ok(subdirs)
}
