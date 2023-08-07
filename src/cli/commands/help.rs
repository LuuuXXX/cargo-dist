use crate::Config;
pub use crate::util::*;

pub fn cli() -> Command {
    subcommand("help")
        .about("Displays help for the program")
}

pub fn exec(config: &mut Config, args: &ArgMatches) {
    println!("config: {:?}", config);
    println!("args: {:?}", args);
}