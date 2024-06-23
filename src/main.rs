use cli::create_command;
use config::create_data_directory;
use git::Repository;

mod cli;
mod config;
mod git;
mod log;

fn main() {
    create_data_directory();

    match create_command().get_matches().subcommand() {
        Some(("install", sub)) => cli::install(sub.get_one::<&Repository>("URL").unwrap()),
        Some(("update", sub)) => cli::update(sub.get_one::<String>("NAME")),
        _ => unreachable!(),
    }
}
