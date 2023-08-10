use crate::output;
use crate::output::CommandOutput;
use crate::pathfinding::pathfind_route;
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use survex_rs::data::{RefStation, SurveyData};
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
    let start = get_station_by_label(&data, &args.start);
    let start = start.borrow();
    let start_id = start.index;

    let end = get_station_by_label(&data, &args.end);
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

fn get_station_by_label(data: &SurveyData, query: &str) -> RefStation {
    let mut matches = Vec::new();
    for station in &data.stations {
        let stn = station.borrow();
        if stn.label == query {
            return Rc::clone(station);
        } else if station.borrow().label.contains(query) {
            matches.push(station);
        }
    }

    if matches.is_empty() {
        eprintln!(
            "There were no full or partial matches for the station name '{}'.",
            query
        );
        eprintln!("Please check the station name is correct and try again.");
        exit(1);
    } else if matches.len() == 1 {
        return Rc::clone(matches[0]);
    } else {
        eprintln!(
            "There were {} possible matches for the station name '{}'.\n",
            matches.len(),
            query
        );

        if matches.len() > 20 {
            eprintln!("The first 20 matches were:\n");
        } else {
            eprintln!("The matches were:\n");
        }

        for station in matches.iter().take(20) {
            let stn = station.borrow();
            eprintln!("  {}", stn.label);
        }

        eprintln!("\nPlease provide a more specific station name and try again.");
        exit(1);
    }
}
