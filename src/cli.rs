pub use clap::Command;

use crate::commands;
use crate::config::Config;
use crate::util::command_prelude::*;
use crate::util::error::CliResult;

pub(crate) fn main() -> CliResult {
    let args = cli().try_get_matches().expect("msg: no matches");

    let mut config = Config::default();
    
    let cmd = "help";
    let exec = commands::builtin_exec(cmd);
    match exec {
        Some(runner) => runner(&mut config, &args),
        None => panic!("Command `{}` not found", cmd),
    }
}

pub fn cli() -> Command {
    // The Usage example
    let usage = "package-rs [OPTION] [COMMAND]";
    // The main program client command.
    Command::new("package-rs")
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .override_usage(usage)
        .help_template(
            "\
Rust's package manager

Usage: {usage}

Options:
{options}

Some common package command are (see all command with --list):
    build        Compile the current package   
    clean        Remove the target directory   
    package      Package the target directory
    test         Run the tests
    search       Search registry for crates
    install      Insatll a Rust binary. Default location is $HOME/.cargo/bin

See 'package-rs help <command>' for more information. \n"
        )
        .arg(flag("version", "Print version info and exit").short('V'))
        .subcommands(commands::builtin())
}