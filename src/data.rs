#[allow(dead_code)] // TODO: Remove this
#[derive(Debug)]
pub struct Dump3D {
    pub title: String,
    pub date: String,
    pub date_numeric: u64,
    pub cs: Option<String>,
    pub version: u8,
    pub extended_elev: bool,
    pub separator: char,
    pub nodes: Vec<Node>,
}

#[allow(dead_code)] // TODO: Remove this
#[derive(Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub label: String,
    pub underground: bool,
    pub surface: bool,
    pub entrance: bool,
    pub exported: bool,
    pub fixed: bool,
    pub anon: bool,
    pub wall: bool,
}

impl Node {
    pub fn new(x: f64, y: f64, z: f64, label: String) -> Self {
        Self {
            x,
            y,
            z,
            label: label.to_string(),
            underground: false,
            surface: false,
            entrance: false,
            exported: false,
            fixed: false,
            anon: false,
            wall: false,
        }
    }
}
