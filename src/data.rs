use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type Successors = Rc<RefCell<Vec<(Node, OrderedFloat<f64>)>>>;

#[allow(dead_code)] // TODO: Remove this
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

    pub fn x_distance(&self, other: &Point) -> f64 {
        if *self.x > *other.x {
            *self.x - *other.x
        } else {
            *other.x - *self.x
        }
    }

    pub fn y_distance(&self, other: &Point) -> f64 {
        if *self.y > *other.y {
            *self.y - *other.y
        } else {
            *other.y - *self.y
        }
    }

    pub fn z_distance(&self, other: &Point) -> f64 {
        if *self.z > *other.z {
            *self.z - *other.z
        } else {
            *other.z - *self.z
        }
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
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[allow(dead_code)] // TODO: Remove this
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

    pub fn get_by_coords(nodes: &[Node], coords: &Point) -> Option<Node> {
        for node in nodes.iter() {
            if node.coords == *coords {
                return Some(node.clone());
            }
        }
        None
    }

    pub fn get_by_name<'a>(nodes: &'a [Node], name: &str) -> Option<&'a Node> {
        nodes.iter().find(|&node| node.label == name)
    }

    pub fn get_successors(&self) -> Vec<(Node, OrderedFloat<f64>)> {
        Rc::clone(&self.successors).borrow().clone()
    }

    pub fn add_successor(&self, node: Node) {
        let distance = OrderedFloat(self.distance(&node));
        self.successors.borrow_mut().push((node, distance));
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

    pub fn has_point(&self, other: &Point) -> bool {
        if self.from_coords == *other || self.to_coords == *other {
            return true;
        }
        false
    }

    pub fn is_from(&self, x: f64, y: f64, z: f64) -> bool {
        if self.from_coords.x == x && self.from_coords.y == y && self.from_coords.z == z {
            return true;
        }
        false
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
