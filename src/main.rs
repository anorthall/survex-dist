#![doc = include_str!("../README.md")]
use std::process::exit;
use survex_dist::command::run;

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
