use crate::Config;
use super::commands;

pub use crate::util::*;

pub fn main() {
    let args = cli().get_matches();

    let mut config = Config::default();
    
    let (cmd, subcommand_args) = match args.subcommand() {
        Some((cmd, args)) => (cmd, args),
        None => todo!(),
    };
    let exec = commands::builtin_exec(cmd);
    match exec {
        Some(runner) => runner(&mut config, subcommand_args),
        None => todo!(),
    }
}

pub fn cli() -> Command { 
    // The Usage example
    let usage = "cargo-dist [OPTION] [COMMAND]";
    // The main program client command.
    Command::new("cargo-dist")
        // Subcommands all count their args' display order independently (from 0),
        // which makes their args interspersed with global args. This puts global args last.
        .next_display_order(1000)
        .allow_external_subcommands(true)
        // Provide a custom help subcommand for calling into man pages
        .disable_help_subcommand(true)
        .override_usage(usage)
        .help_template(
            "\
Rust's package manager

Usage: {usage}

Options:
{options}

Some common package command are (see all command with --list):
    dist         Distribution package manager
    install      Insatll a Rust binary. Default location is $HOME/.cargo/bin

See 'cargo-dist help <command>' for more information. \n"
        )
        .arg(flag("version", "Print version info and exit").short('V'))
        .subcommands(commands::builtin())
}