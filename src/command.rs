use crate::output;
use crate::output::CommandOutput;
use crate::pathfinding::pathfind_route;
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use survex_rs::read::load_from_path;

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

// TODO: Allow for via nodes and excluded nodes.
pub fn run() -> Result<(), Box<dyn Error>> {
    let start_time = std::time::Instant::now();

    // Initialise the program and parse the command line arguments.
    let args = Args::parse();
    let data = load_from_path(args.file.clone()).unwrap_or_else(|_| {
        let msg = format!("Unable to open file '{}'.", args.file.display());
        fatal_error(msg);
    });

    // Find the start and end nodes.
    let start = data.get_by_label_part(&args.start).unwrap_or_else(|| {
        let msg = format!(
            "Unable to find start station '{}'. You may need to be more specific.",
            args.end
        );
        fatal_error(msg);
    });
    let start = start.borrow();
    let start_id = start.index;

    let end = data.get_by_label_part(&args.end).unwrap_or_else(|| {
        let msg = format!(
            "Unable to find end station '{}'. You may need to be more specific.",
            args.end
        );
        fatal_error(msg);
    });
    let end = end.borrow();
    let end_id = end.index;

    // Run the pathfinding algorithm.
    let route = vec![start_id, end_id];
    let path = pathfind_route(&data, route);

    // Convert the vector of NodeIndexes to a vector of stations.
    let mut route = Vec::new();
    for index in path {
        let station = data.get_by_index(index).unwrap();
        route.push(station);
    }

    // Output the results.
    let excluded = Vec::new();
    let via_node_names = Vec::new();
    let output = CommandOutput::new(start_time, args, route, excluded, via_node_names);
    output.print()?;

    Ok(())
}

pub fn fatal_error(msg: String) -> ! {
    eprintln!("{}", msg);
    exit(1);
}
