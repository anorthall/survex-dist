use clap::Parser;
use env_logger::Env;
use log::{error, info};
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use survex_dist::parser::parse_dump3d;

#[derive(Parser)]
#[command(name = "survex-dist")]
#[command(author = "Andrew Northall <andrew@northall.me.uk")]
#[command(version = "0.1.0")]
#[command(about = "Calculate the distance between two points in a Survex dump3d file")]
#[command(long_about = None)]
struct Args {
    file: PathBuf,
    start: String,
    end: String,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    info!(
        "Processing file '{}' from '{}' to '{}'.",
        args.file.display(),
        args.start,
        args.end
    );

    let file = match File::open(&args.file) {
        Ok(file) => file,
        Err(_) => {
            let msg = format!(
                "Unable to open file {}. Are you sure it exists and is readable?",
                args.file.display()
            );
            error!("{}", msg);
            eprintln!("{}", msg);
            exit(1);
        }
    };

    match parse_dump3d(file) {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("Unable to parse file '{}': {}", args.file.display(), e);
            error!("{}", msg);
            eprintln!("{}", msg);
            exit(1);
        }
    }
}
