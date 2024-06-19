use cli::{create_command, RepositoryData};
use config::create_data_directory;

mod cli;
mod config;
mod log;

fn main() {
    create_data_directory();

    match create_command().get_matches().subcommand() {
        Some(("install", sub)) => cli::install(sub.get_one::<RepositoryData>("URL").unwrap()),
        Some(("update", sub)) => cli::update(sub.get_one::<String>("NAME")),
        _ => unreachable!(),
    }
}
