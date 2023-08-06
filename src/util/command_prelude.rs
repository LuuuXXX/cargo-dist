pub use clap::Command;
pub use clap::{Arg, ArgAction, ArgMatches};

pub fn flag(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub fn subcommand(name: &'static str) -> Command {
    Command::new(name)
}