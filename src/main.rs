use clap::Parser;
use log::{error, info};
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use survex_dist::data::{Leg, Node};
use survex_dist::parser::parse_dump3d;

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

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
    let start = Instant::now();
    env_logger::init();

    let args = Args::parse();
    info!("Processing file '{}'.", args.file.display(),);

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

    let (_headers, nodes, legs) = match parse_dump3d(file) {
        Ok(data) => data,
        Err(e) => {
            let msg = format!("Unable to parse file '{}': {}", args.file.display(), e);
            error!("{}", msg);
            eprintln!("{}", msg);
            exit(1);
        }
    };

    info!("Successfully parsed file '{}'.", args.file.display());
    info!("Found {} nodes and {} legs.", nodes.len(), legs.len());

    match pathfind(&args, nodes, legs, start) {
        Ok(_) => {
            info!("Successfully calculated distance.");
        }
        Err(e) => {
            let msg = format!("Unable to calculate distance: {}", e);
            error!("{}", msg);
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

fn pathfind(
    args: &Args,
    nodes: Vec<Node>,
    legs: Vec<Leg>,
    start: Instant,
) -> Result<(), Box<dyn Error>> {
    let start_node = Node::get_by_name(&nodes, &args.start)
        .unwrap_or_else(|| panic!("Unable to locate node: {}", &args.start));
    let end_node = Node::get_by_name(&nodes, &args.end)
        .unwrap_or_else(|| panic!("Unable to locate node: {}", &args.end));
    info!("Start node: {}", &start_node);
    info!("End node: {}", &end_node);
    info!(
        "Straight line distance between nodes: {:.2}m",
        start_node.distance(end_node)
    );

    for leg in legs.iter().cloned() {
        for node in nodes.iter() {
            if node.coords == leg.from_coords {
                let candidate_node = Node::get_by_coords(&nodes, &leg.to_coords);
                if let Some(candidate_node) = candidate_node {
                    node.add_successor(candidate_node);
                } else {
                    info!("Unable to find node with coords: {}", &leg.to_coords);
                }
            } else if node.coords == leg.to_coords {
                let candidate_node = Node::get_by_coords(&nodes, &leg.from_coords);
                if let Some(candidate_node) = candidate_node {
                    node.add_successor(candidate_node);
                } else {
                    info!("Unable to find node with coords: {}", &leg.from_coords);
                }
            }
        }
    }

    let result = astar(
        start_node,
        |node| node.get_successors(),
        |node| OrderedFloat(node.distance(end_node)),
        |node| *node == *end_node,
    );

    match result {
        Some(result) => {
            let (path, _) = result;
            let mut distance = 0.0_f64;
            let mut path_table = Table::new();
            path_table.set_titles(row!["Station and label", "Coords", "Distance"]);
            for (i, node) in path.iter().enumerate() {
                if i > 0 {
                    distance += node.distance(&path[i - 1]);
                }
                path_table.add_row(row![
                    node.label,
                    format!("{}", node.coords),
                    format!("{:.2}m", distance)
                ]);
            }

            let mut meta_table = Table::new();
            meta_table.add_row(row!["Origin station", &start_node.label]);
            meta_table.add_row(row!["Destination station", &end_node.label]);
            meta_table.add_row(row!["Path length", format!("{} stations", path.len())]);
            meta_table.add_row(row![
                "Straight line distance",
                format!("{:.2}m", start_node.distance(end_node))
            ]);
            meta_table.add_row(row!["Walking/survey distance", format!("{:.2}m", distance)]);
            meta_table.add_row(row!["Time taken", format!("{:.2?}", start.elapsed())]);

            path_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            path_table.printstd();

            println!();

            meta_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            meta_table.printstd();
        }
        None => {
            let msg = format!(
                "Unable to find path between nodes {} and {}.",
                &args.start, &args.end
            );
            error!("{}", msg);
            eprintln!("{}", msg);
            exit(1);
        }
    }

    Ok(())
}
