pub mod dist;
pub mod install;

pub use crate::util::*;

pub fn builtin() -> Vec<Command> {
    vec![
        dist::cli(),
        install::cli(),
    ]
}

pub type Exec = fn(&ArgMatches);

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "dist" => dist::exec,
        "install" => install::exec,
        _ => panic!("Failed to identify command"),
    };
    Some(f)
}