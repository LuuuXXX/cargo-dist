pub mod dist;
pub mod help;
pub mod install;

pub use crate::util::*;

use crate::Config;

pub fn builtin() -> Vec<Command> {
    vec![
        dist::cli(),
        help::cli(),
        install::cli(),
    ]
}

pub type Exec = fn(&mut Config, &ArgMatches);

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "dist" => dist::exec,
        "help" => help::exec,
        "install" => install::exec,
        _ => panic!("Failed to identify command"),
    };
    Some(f)
}