use crate::Config;
pub use crate::util::*;

pub fn cli() -> Command {
    subcommand("dist")
        .about("Distribution package manager")
        .arg(flag("output-dir", "The directory to output the distribution").short('o'))
}

pub fn exec(config: &mut Config, args: &ArgMatches) {
    println!("config: {:?}", config);
    println!("args: {:?}", args);
}