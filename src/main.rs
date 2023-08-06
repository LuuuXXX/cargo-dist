pub mod commands;
pub mod util;

mod cli;
mod config;

fn main() {
    let res = cli::main();
    match res {
        Err(e) => panic!("{:?}", e.error),
        Ok(()) => {}
    }
}