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

    match pathfind(&args, nodes, legs) {
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

    let duration = start.elapsed();
    println!("Execution time: {:20.2?}", duration);
}

fn pathfind(args: &Args, nodes: Vec<Node>, legs: Vec<Leg>) -> Result<(), Box<dyn Error>> {
    let start_node = Node::get_by_name(&nodes, &args.start)
        .expect(&format!("Unable to locate node: {}", &args.start));
    let end_node = Node::get_by_name(&nodes, &args.end)
        .expect(&format!("Unable to locate node: {}", &args.end));
    info!("Start node: {}", &start_node);
    info!("End node: {}", &end_node);
    info!(
        "Straight line distance between nodes: {:.2}m",
        start_node.distance(&end_node)
    );

    for leg in legs.iter().map(|leg| leg.clone()) {
        for node in nodes.iter() {
            if node.coords == leg.from_coords {
                let candidate_node = Node::get_by_coords(&nodes, &leg.to_coords);
                if candidate_node.is_none() {
                    info!("Unable to find node with coords: {}", &leg.to_coords);
                } else {
                    node.add_successor(candidate_node.unwrap());
                }
            } else if node.coords == leg.to_coords {
                let candidate_node = Node::get_by_coords(&nodes, &leg.from_coords);
                if candidate_node.is_none() {
                    info!("Unable to find node with coords: {}", &leg.from_coords);
                } else {
                    node.add_successor(candidate_node.unwrap());
                }
            }
        }
    }

    let result = astar(
        start_node,
        |node| node.get_successors(),
        |node| OrderedFloat(node.distance(&end_node)),
        |node| *node == *end_node,
    );

    match result {
        Some(result) => {
            let (path, _) = result;
            let mut distance = 0.0_f64;
            for (i, node) in path.iter().enumerate() {
                if i > 0 {
                    distance += node.distance(&path[i - 1]);
                }
                println!(
                    "{:3} {:8.2}m   {:65} X: {:8}   Y: {:8}   Z: {:8}",
                    i, distance, node.label, node.coords.x, node.coords.y, node.coords.z
                );
            }

            println!("\nOrigin station: {}", &start_node.label);
            println!("Destination station: {}", &end_node.label);
            println!("Path length: {} stations", path.len());
            println!(
                "Straight line distance between stations: {:.2}m",
                start_node.distance(&end_node)
            );
            println!("Walking/survey distance between stations: {:.2}m", distance);
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
