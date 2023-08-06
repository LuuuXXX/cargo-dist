use clap::ArgMatches;

use crate::util::command_prelude::*;
use crate::util::error::*;
use crate::config::*;

pub fn cli() -> Command {
    subcommand("help")
        .about("Displays help for the program")
        .arg(Arg::new("COMMAND").action(ArgAction::Set))
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    Ok(())
}