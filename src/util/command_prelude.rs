pub use clap::Command;
pub use clap::{Arg, ArgAction, ArgMatches};

pub trait CommandExt: Sized {
    fn _arg(self, arg: Arg) -> Self;

    fn arg_target_triple(self, target: &'static str) -> Self {
        self._arg(
            multi_opt("target", "TRIPLE", target)
        )
    }

    fn arg_target_dir(self) -> Self {
        self._arg(
            opt("target-dir", "Directory for all generated artifacts").value_name("Directory"),
        )
    }

    fn arg_release(self, release: &'static str) -> Self {
        self._arg(flag("release", release).short('r'))
    }

    fn arg_manifest_path(self) -> Self {
        self._arg(opt("manifest-path", "Path to Cargo.toml").value_name("PATH"))
    }
}

impl CommandExt for Command {
    fn _arg(self, arg: Arg) -> Self {
        self.arg(arg)
    }
}

pub fn flag(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub fn opt(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::Set)
}

pub fn multi_opt(name: &'static str, value_name: &'static str, help: &'static str) -> Arg {
    opt(name, help)
        .value_name(value_name)
        .action(ArgAction::Append)
}

pub fn subcommand(name: &'static str) -> Command {
    Command::new(name)
}