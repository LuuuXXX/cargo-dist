pub use crate::util::*;

pub fn cli() -> Command {
    subcommand("install")
        .about("Insatll a Rust binary.")
        .arg(
            Arg::new("crate")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .num_args(0..),
        )
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .after_help("Run `cargo-dist dist --help` for more detailed information. \n")
}

pub fn exec(_args: &ArgMatches) {
    // let krates = args
    //     .get_many::<String>("crate")
    //     .unwrap_or_default()
    //     .collect::<Vec<_>>();

    // for krate in krates {
    //     // Generate the compile options
    //     let compile_opts = args._compile_options(krate);
    //     let cargo = cargo(&compile_opts);
    //     // Process to generate the tarball
    //     targball(cargo,&compile_opts);
    // }
    todo!()
}