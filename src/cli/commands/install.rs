use crate::Config;
pub use crate::util::*;

pub fn cli() -> Command {
    subcommand("install")
        .about("Insatll a Rust binary.")
}

pub fn exec(config: &mut Config, args: &ArgMatches) {
    println!("config: {:?}", config);
    println!("args: {:?}", args);
}