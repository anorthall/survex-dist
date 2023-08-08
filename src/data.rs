use crate::command::fatal_error;
use log::info;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::process::exit;
use std::rc::Rc;

type Successors = Rc<RefCell<Vec<(Node, OrderedFloat<f64>)>>>;

#[derive(Debug)]
pub struct Headers {
    pub title: String,
    pub date: String,
    pub date_numeric: u64,
    pub cs: Option<String>,
    pub version: u8,
    pub extended_elev: bool,
    pub separator: char,
}

impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Date: {}, CS: {:?}, Version: {}, Extended elevation: {}.",
            self.title, self.date_numeric, self.cs, self.version, self.extended_elev,
        )
    }
}

impl Headers {
    pub fn new(
        title: String,
        date: String,
        date_numeric: u64,
        cs: Option<String>,
        version: u8,
        extended_elev: bool,
        separator: char,
    ) -> Self {
        Self {
            title,
            date,
            date_numeric,
            cs,
            version,
            extended_elev,
            separator,
        }
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub z: OrderedFloat<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}, {:.2}, {:.2}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd)]
pub struct Node {
    pub coords: Point,
    pub label: String,
    pub underground: bool,
    pub surface: bool,
    pub entrance: bool,
    pub exported: bool,
    pub fixed: bool,
    pub anon: bool,
    pub wall: bool,
    successors: Successors,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
        self.label.hash(state);
        self.underground.hash(state);
        self.surface.hash(state);
        self.entrance.hash(state);
        self.exported.hash(state);
        self.fixed.hash(state);
        self.anon.hash(state);
        self.wall.hash(state);
        self.wall.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Node {
    pub fn new(coords: Point, label: String) -> Self {
        Self {
            coords,
            label: label.to_string(),
            underground: false,
            surface: false,
            entrance: false,
            exported: false,
            fixed: false,
            anon: false,
            wall: false,
            successors: Rc::from(RefCell::from(Vec::new())),
        }
    }

    /// Iterate through a vector of nodes and attach legs to them as successors.
    pub fn attach_legs(nodes: &[Node], legs: &[Leg]) -> Result<(), Box<dyn Error>> {
        for leg in legs.iter() {
            for node in nodes.iter() {
                if node.coords == leg.from_coords {
                    node.add_successor(nodes, &leg.to_coords);
                } else if node.coords == leg.to_coords {
                    node.add_successor(nodes, &leg.from_coords);
                }
            }
        }

        Ok(())
    }

    /// Iterate through a vector of nodes and exclude nodes which match the query. Return
    /// a tuple of remaining nodes and excluded node names.
    pub fn exclude_nodes(nodes: &[Node], exclude: &[String]) -> (Vec<Node>, Vec<String>) {
        let mut remaining_nodes = Vec::new();
        let mut excluded_node_names = Vec::new();

        for query in exclude.iter() {
            // This will check that the query is a valid node name and that it is unique.
            // After this check, it is safe to assume that any node name containing the query
            // will match exactly one node.
            let _ = Node::get_by_name(nodes, query);
        }

        for node in nodes.iter() {
            let mut excluded = false;
            for query in exclude.iter() {
                if node.label.contains(query) {
                    excluded = true;
                    break;
                }
            }

            if excluded {
                excluded_node_names.push(node.clone().short_name());
            } else {
                remaining_nodes.push(node.clone());
            }
        }

        (remaining_nodes, excluded_node_names)
    }

    pub fn short_name(&self) -> String {
        match self.label.clone().split_once('.') {
            Some((_, suffix)) => suffix.to_string(),
            None => self.label.clone(),
        }
    }

    fn get_by_coords(nodes: &[Node], coords: &Point) -> Option<Node> {
        for node in nodes.iter() {
            if node.coords == *coords {
                return Some(node.clone());
            }
        }
        None
    }

    #[allow(clippy::comparison_chain)]
    pub fn get_by_name<'a>(nodes: &'a [Node], query: &str) -> &'a Node {
        let matches = nodes
            .iter()
            .filter(|&node| node.label.contains(query))
            .collect::<Vec<_>>();

        // If there is only one match, return it, otherwise try to match the full name.
        if matches.len() == 1 {
            return matches[0];
        } else {
            for node in matches.iter() {
                if node.label == query {
                    return node;
                }
            }
        }

        if matches.is_empty() {
            let help = "No station name fully or partially matched the query.";
            fatal_error(format!("Unable to find station: {}\n{}", query, help));
        }

        // There must be multiple matches.
        eprintln!("The station name is ambiguous, try being more specific.\n");
        eprintln!("{} matched the following stations:\n", query);
        for (i, node) in matches.iter().enumerate() {
            if i < 20 {
                eprintln!("{}", node.label);
            } else if i == 20 {
                eprintln!("... ({} more)", matches.len() - 20);
            }
        }
        exit(1);
    }

    pub fn get_successors(&self) -> Vec<(Node, OrderedFloat<f64>)> {
        Rc::clone(&self.successors).borrow().clone()
    }

    pub fn add_successor(&self, nodes: &[Node], coords: &Point) {
        match Node::get_by_coords(nodes, coords) {
            Some(node) => {
                let distance = OrderedFloat(self.distance(&node));
                self.successors.borrow_mut().push((node, distance));
            }
            None => {
                info!("Unable to find node with coordinates: {}", coords);
            }
        }
    }

    pub fn distance(&self, other: &Node) -> f64 {
        self.coords.distance(&other.coords)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: X: {}, Y: {}, Z: {}",
            self.label, self.coords.x, self.coords.y, self.coords.z
        )
    }
}

#[derive(Debug, Clone)]
pub struct Leg {
    pub from_coords: Point,
    pub to_coords: Point,
    pub label: Option<String>,
}

impl Leg {
    pub fn new(from_coords: Point, to_coords: Point, label: Option<String>) -> Self {
        Self {
            from_coords,
            to_coords,
            label,
        }
    }
}

impl Display for Leg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.label {
            Some(label) => write!(
                f,
                "From: {}, To: {}, Label: {}",
                self.from_coords, self.to_coords, label
            ),
            None => write!(f, "From: {}, To: {}", self.from_coords, self.to_coords),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 1.0);
        assert_eq!(p1.distance(&p2), 1.7320508075688772);
        assert_eq!(p2.distance(&p1), 1.7320508075688772);

        let p3 = Point::new(0.0, 0.0, 0.0);
        let p4 = Point::new(0.0, 0.0, 0.0);
        assert_eq!(p3.distance(&p4), 0.0);
        assert_eq!(p4.distance(&p3), 0.0);

        let p5 = Point::new(0.0, 0.0, 0.0);
        let p6 = Point::new(0.0, 0.0, 1.0);
        assert_eq!(p5.distance(&p6), 1.0);

        let p7 = Point::new(0.0, 0.0, 0.0);
        let p8 = Point::new(0.0, 1.0, 0.0);
        assert_eq!(p7.distance(&p8), 1.0);

        let p9 = Point::new(0.0, 0.0, 0.0);
        let p10 = Point::new(1.0, 0.0, 0.0);
        assert_eq!(p9.distance(&p10), 1.0);
    }
}
