use crate::data::Node;
use serde::Serialize;
use std::error::Error;
use std::time::Instant;

#[derive(Serialize)]
pub struct CommandOutput {
    path: Vec<PathLine>,
    metadata: Vec<MetadataItem>,
    #[serde(skip)]
    start_time: Instant,
    #[serde(skip)]
    format: Format,
    #[serde(skip)]
    print_path: bool,
    #[serde(skip)]
    print_metadata: bool,
}

impl CommandOutput {
    pub fn new(start_time: Instant, format: Format, path: Vec<Node>) -> CommandOutput {
        let expect_msg = "Path must have at least one node.";
        let start_node = path.first().expect(expect_msg).clone();
        let end_node = path.last().expect(expect_msg).clone();

        let mut path_distance = 0.0_f64;
        let mut leg_distance = 0.0_f64;
        let mut path_lines = Vec::new();
        for (i, node) in path.iter().enumerate() {
            if i > 0 {
                leg_distance = node.distance(&path[i - 1]);
                path_distance += leg_distance;
            }

            path_lines.push(PathLine::new(i + 1, node, leg_distance, path_distance));
        }

        let mut output = CommandOutput {
            start_time,
            format,
            path: path_lines,
            metadata: Vec::new(),
            print_path: true,
            print_metadata: true,
        };

        output.build_metadata(start_node, end_node, path_distance);
        output
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        match self.format {
            Format::Table => self.print_table(),
            Format::JSON => self.print_json()?,
            Format::Text => self.print_text(),
        }

        Ok(())
    }

    fn print_table(&self) {
        if self.print_path {
            table::print_path(self);
        }

        if self.print_metadata {
            println!();
            table::print_metadata(self);
        }
    }

    fn print_json(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        println!("{}", json);

        Ok(())
    }

    fn print_text(&self) {
        if self.print_path {
            text::print_path(self);
        }

        if self.print_metadata {
            println!("\n");
            text::print_metadata(self);
        }
    }

    fn build_metadata(&mut self, start_node: Node, end_node: Node, path_distance: f64) {
        let sl_distance = start_node.distance(&end_node);
        self.add_metadata("Start station", start_node.label.as_str());
        self.add_metadata("End station", end_node.label.as_str());
        self.add_metadata("Path length", &format!("{:.2}", self.path.len()));
        self.add_metadata("Path distance", &format!("{:.2}m", path_distance));
        self.add_metadata("Straight line distance", &format!("{:.2}m", sl_distance));
        self.add_metadata("Time taken", &format!("{:.2?}", self.start_time.elapsed()));
    }

    fn add_metadata(&mut self, name: &str, value: &str) {
        self.metadata.push(MetadataItem {
            name: name.to_string(),
            value: value.to_string(),
        });
    }
}

#[derive(clap::ValueEnum, Clone)]
pub enum Format {
    Table,
    JSON,
    Text,
}

#[derive(Serialize)]
#[serde(rename = "station")]
struct PathLine {
    id: usize,
    name: String,
    coords: String,
    leg_distance: String,
    total_distance: String,
}

impl PathLine {
    fn new(id: usize, node: &Node, leg_distance: f64, total_distance: f64) -> PathLine {
        PathLine {
            id,
            name: node.short_name(),
            coords: format!("{}", node.coords),
            leg_distance: format!("{:.2}m", leg_distance),
            total_distance: format!("{:.2}m", total_distance),
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "metadata")]
struct MetadataItem {
    name: String,
    value: String,
}

pub mod table {
    use crate::output::CommandOutput;
    use prettytable::{format, row, Table};

    pub fn print_path(output: &CommandOutput) {
        let path = &output.path;
        let mut path_table = Table::new();
        path_table.set_titles(row!["Station label", "Coords", "Leg Dist", "Total Dist"]);

        for line in path.iter() {
            path_table.add_row(row![
                line.name,
                line.coords,
                line.leg_distance,
                line.total_distance
            ]);
        }

        path_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        path_table.printstd();
    }

    pub fn print_metadata(output: &CommandOutput) {
        let mut table = Table::new();
        table.set_titles(row!["Metadata", "Value"]);
        for item in output.metadata.iter() {
            table.add_row(row![item.name, item.value]);
        }
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}

pub mod text {
    use crate::output::CommandOutput;

    pub fn print_path(output: &CommandOutput) {
        let path = &output.path;
        println!("Route taken\n-----------\n");
        for line in path.iter() {
            println!(
                "{}: {} - {} - {}",
                line.id, line.name, line.leg_distance, line.total_distance
            );
        }
    }

    pub fn print_metadata(output: &CommandOutput) {
        println!("Path metadata\n-------------\n");
        for item in output.metadata.iter() {
            println!("{}: {}", item.name, item.value);
        }
    }
}
