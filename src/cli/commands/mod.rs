pub mod dist;

pub use crate::util::*;

pub fn builtin() -> Vec<Command> {
    vec![
        dist::cli(),
    ]
}

pub type Exec = fn(&ArgMatches);

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "dist" => dist::exec,
        _ => panic!("Failed to identify command"),
    };
    Some(f)
}