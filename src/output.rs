use crate::data::Node;
use prettytable::{format, row, Table};
use std::time::Instant;

pub fn print_path_as_table(path: &[Node]) {
    let mut path_table = Table::new();
    path_table.set_titles(row!["Station label", "Coords", "Leg Dist", "Total Dist"]);

    let mut distance = 0.0_f64;
    let mut leg_distance = 0.0_f64;
    for (i, node) in path.iter().enumerate() {
        if i > 0 {
            leg_distance = node.distance(&path[i - 1]);
            distance += leg_distance;
        }

        path_table.add_row(row![
            node.short_name(),
            format!("{}", node.coords),
            format!("{:.2}m", leg_distance),
            format!("{:.2}m", distance),
        ]);
    }

    path_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    path_table.printstd();
}

pub fn print_metadata_as_table(path: &Vec<Node>, distance: &f64, time_taken: Instant) {
    let start = path
        .first()
        .expect("The path will always have at least one node");
    let end = path
        .last()
        .expect("The path will always have at least one node");

    let mut table = Table::new();
    table.set_titles(row!["Metadata", "Value"]);
    table.add_row(row!["Origin station", &start.label]);
    table.add_row(row!["Destination station", &end.label]);
    table.add_row(row!["Path length", format!("{} stations", path.len())]);
    table.add_row(row![
        "Average leg length",
        format!("{:.2}m", distance / path.len() as f64)
    ]);
    table.add_row(row![
        "Straight line distance",
        format!("{:.2}m", start.distance(end))
    ]);
    table.add_row(row!["Walking/survey distance", format!("{:.2}m", distance)]);
    table.add_row(row!["Time taken", format!("{:.2?}", time_taken.elapsed())]);

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}
