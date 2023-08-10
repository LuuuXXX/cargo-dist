use crate::ArgMatchesExt; 
use crate::Mode;

use crate::cli::builder::Builder;
use crate::cli::builder::cargo;

pub use crate::util::*;

pub fn cli() -> Command {
    subcommand("dist")
        .about("Distribution package manager")
        .arg_release("Build artifacts in release mode, with optimizations")
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .arg_manifest_path()
        .after_help("Run `cargo-dist dist --help` for more detailed information. \n")
}

pub fn exec(args: &ArgMatches) {
    // Generate the compile options
    let compile_opts = args.compile_options(Mode::Dist);
    
    // Invokes the `cargo build` command to build the artifacts
    let cargo = cargo(&compile_opts);


    // Process to builad package and generate the tarball
    let builder = Builder::new(cargo, &compile_opts);
    builder.run();
}