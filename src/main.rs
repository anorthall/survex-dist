use std::path::PathBuf;
use clap::Parser;

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
    let args = Args::parse();
    println!("Processing file '{}' from '{}' to '{}'.", args.file.display(), args.start, args.end);
}
