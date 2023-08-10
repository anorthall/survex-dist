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
/// survex-dist: a tool for calculating the distance between stations in a Survex file.
///
/// Provide a Survex 3D file, as well as an start and end station, and the tool will calculate the
/// distance between the two stations and display the route taken.
///
/// If you wish to use a station as a via point, use the --via flag. Alternatively, use the
/// --avoid flag to ensure a station is not included in the route. Both flags can be used multiple
/// times to specify multiple via or avoid points.
///
/// Visit https://docs.rs/survex-dist or https://github.com/anorthall/survex-dist for more
/// information.
pub struct Args {
    /// The Survex 3D file to process.
    pub file: PathBuf,
    /// The survey station to start from
    #[clap(required_unless_present = "analyse")]
    pub start: Option<String>,
    /// The survey station to end at
    #[clap(required_unless_present = "analyse")]
    pub end: Option<String>,
    /// Force inclusion of a survey station in the route.
    #[clap(short, long)]
    pub via: Vec<String>,
    /// Exclude a survey station from the route. To specify multiple stations, use the flag
    /// multiple times.
    #[clap(short, long)]
    pub avoid: Vec<String>,
    /// The output format to use.
    #[clap(short, long, default_value = "table")]
    pub format: output::Format,
    /// Do not print the path taken.
    #[clap(short, long)]
    pub no_path: bool,
    /// Analyse the file provided and print information about the survey. Not yet implemented.
    #[clap(long)]
    pub analyse: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let start_time = std::time::Instant::now();

    // Initialise the program and parse the command line arguments.
    let args = Args::parse();
    let mut data = load_from_path(args.file.clone()).unwrap_or_else(|_| {
        eprintln!(
            "Unable to open or read file '{}'. Is it a valid Survex 3D file?",
            args.file.display()
        );
        exit(1);
    });

    // If the user has specified the --analyse flag, print information about the survey and exit.
    if args.analyse {
        run_analysis(data)?;
        return Ok(());
    }

    // Find the start and end nodes.
    let start = get_station_by_label(&data, &args.start.clone().unwrap());
    let start = start.borrow();
    let start_id = start.index;

    let end = get_station_by_label(&data, &args.end.clone().unwrap());
    let end = end.borrow();
    let end_id = end.index;

    // Avoid any nodes specified by the user by removing them from the graph.
    let mut avoided = Vec::new();
    for query in &args.avoid {
        let station = get_station_by_label(&data, query);
        let station = station.borrow();
        data.graph.remove_node(station.index);
        avoided.push(station.label.clone());
    }

    // Build the route including any via nodes specified by the user.
    let mut route = vec![start_id];
    let mut via = Vec::new();
    for query in &args.via {
        let station = get_station_by_label(&data, query);
        let station = station.borrow();
        via.push(station.label.clone());
        route.push(station.index);
    }
    route.push(end_id);

    // Run the pathfinding algorithm.
    let path = pathfind_route(&data, route);

    let path = match path {
        Some(path) => path,
        None => {
            eprintln!(
                "Unable to find a route between '{}' and '{}'.",
                args.start.unwrap(),
                args.end.unwrap()
            );
            exit(1);
        }
    };

    // Convert the vector of NodeIndexes to a vector of stations.
    let mut route = Vec::new();
    for index in path {
        let station = data.get_by_index(index).unwrap();
        route.push(station);
    }

    // Output the results.
    let output = CommandOutput::new(start_time, args, route, avoided, via);
    output.print()?;

    Ok(())
}

fn run_analysis(_data: SurveyData) -> Result<(), Box<dyn Error>> {
    println!("Analysis not yet implemented.");
    Ok(())
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
