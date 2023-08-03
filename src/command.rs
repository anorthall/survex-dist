use crate::data::Node;
use crate::output::{print_metadata_as_table, print_path_as_table};
use crate::parser::parse_dump3d;
use crate::pathfinding::pathfind;
use clap::Parser;
use log::info;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(name = "survex-dist")]
#[command(author, version, about)]
#[command(
    long_about = "For information on usage, please see https://github.com/anorthall/survex-dist"
)]
pub struct Args {
    /// The file to process.
    pub file: PathBuf,
    /// The survey station to start from. Partial matches are allowed.
    pub start: String,
    /// The survey station to end at. Partial matches are allowed.
    pub end: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let start_time = std::time::Instant::now();

    // Initialise the program and parse the command line arguments.
    let args = Args::parse();
    let file = File::open(&args.file).unwrap_or_else(|_| {
        let msg = format!("Unable to open file '{}'.", args.file.display());
        fatal_error(msg);
    });

    // Parse the dump3d file.
    let (_headers, nodes, legs) = parse_dump3d(file)?;

    // Find the start and end nodes.
    let start = Node::get_by_name(&nodes, &args.start);
    let end = Node::get_by_name(&nodes, &args.end);

    // Iterate over the nodes and legs, and attach the legs to the nodes.
    Node::attach_legs(&nodes, &legs)?;
    info!("Successfully parsed file '{}'.", args.file.display());
    info!("Found {} nodes and {} legs.", nodes.len(), legs.len());

    // Run the pathfinding algorithm.
    let (path, cost) = pathfind(start, end);

    // TODO: Implement different output formats.
    // Print the output as a table.
    print_path_as_table(&path);
    println!();
    print_metadata_as_table(&path, &cost, start_time);

    Ok(())
}

pub fn fatal_error(msg: String) -> ! {
    eprintln!("{}", msg);
    exit(1);
}
