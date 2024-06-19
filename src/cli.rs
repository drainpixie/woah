use clap::{arg, command, Command};

pub fn create_command() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            command!("install")
                .about("Install a template")
                .arg(arg!([URL])),
        )
        .subcommand(
            command!("update")
                .about("Update templates")
                .arg(arg!(<NAME>)),
        )
}
