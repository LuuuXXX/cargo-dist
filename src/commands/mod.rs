use crate::util::command_prelude::*;
use crate::util::error::*;

use crate::config::*;

pub mod help;

pub fn builtin() -> Vec<Command> {
    vec![
        help::cli(),
    ]
}

pub type Exec = fn(&mut Config, &ArgMatches) -> CliResult;

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "help" => help::exec,
        _ => return None,
    };

    Some(f)
}