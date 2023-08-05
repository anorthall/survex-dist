use crate::data::Node;
use crate::output;
use crate::output::CommandOutput;
use crate::parser::parse_dump3d;
use crate::pathfinding::pathfind_route;
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
    /// The survey station to start from
    pub start: String,
    /// The survey station to end at
    pub end: String,
    /// Force inclusion of a survey station in the route. To specify multiple stations,
    /// use the flag multiple times and in the order you wish them to be included. If via points
    /// are specified, the pathfinding algorithm will run several times, once for each via point.
    /// This can result in a path which may pass through a survey station more than once as well as
    /// longer path generation times.
    #[clap(short, long)]
    pub via: Vec<String>,
    /// Exclude a survey station from the route. To specify multiple stations, use the flag
    /// multiple times.
    #[clap(short, long)]
    pub exclude: Vec<String>,
    /// The output format to use.
    #[clap(short, long, default_value = "table")]
    pub format: output::Format,
    /// Do not print the path taken.
    #[clap(short, long)]
    pub no_path: bool,
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

    // Find the excluded nodes.
    let (nodes, excluded) = Node::exclude_nodes(&nodes, &args.exclude);

    // Find the required nodes.
    let start = Node::get_by_name(&nodes, &args.start);
    let end = Node::get_by_name(&nodes, &args.end);

    let via_nodes = &args
        .via
        .iter()
        .map(|n| Node::get_by_name(&nodes, n))
        .collect::<Vec<&Node>>();
    let via_node_names = via_nodes
        .iter()
        .map(|n| n.label.clone())
        .collect::<Vec<String>>();

    let mut route = vec![start];
    for node in via_nodes {
        route.push(node);
    }
    route.push(end);

    // Iterate over the nodes and legs, and attach the legs to the nodes.
    Node::attach_legs(&nodes, &legs)?;
    info!("Successfully parsed file '{}'.", args.file.display());
    info!("Found {} nodes and {} legs.", nodes.len(), legs.len());

    // Run the pathfinding algorithm.
    let path = pathfind_route(route);

    // Output the results.
    let output = CommandOutput::new(start_time, args, path, excluded, via_node_names);
    output.print()?;

    Ok(())
}

pub fn fatal_error(msg: String) -> ! {
    eprintln!("{}", msg);
    exit(1);
}
