use cli::create_command;
use config::create_data_directory;

mod cli;
mod config;

fn main() {
    create_data_directory();

    match create_command().get_matches().subcommand() {
        Some(("install", sub)) => println!(
            "install {:?}",
            sub.get_one::<(String, String, String)>("URL").unwrap()
        ),
        Some(("update", sub)) => println!("update {}", sub.get_one::<String>("NAME").unwrap()),
        _ => unreachable!(),
    }
}
