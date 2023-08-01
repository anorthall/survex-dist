use std::fmt::Display;

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
    pub legs: Vec<Leg>,
}

impl Display for Dump3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Date: {}, CS: {:?}, Version: {}, Extended elevation: {}.",
            self.title, self.date_numeric, self.cs, self.version, self.extended_elev,
        )
    }
}

impl Dump3D {
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
            nodes: Vec::new(),
            legs: Vec::new(),
        }
    }
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

#[derive(Debug)]
pub struct Leg {
    pub from_x: f64,
    pub from_y: f64,
    pub from_z: f64,
    pub to_x: f64,
    pub to_y: f64,
    pub to_z: f64,
    pub label: Option<String>,
}

impl Leg {
    pub fn new(
        from_x: f64,
        from_y: f64,
        from_z: f64,
        to_x: f64,
        to_y: f64,
        to_z: f64,
        label: Option<String>,
    ) -> Self {
        Self {
            from_x,
            from_y,
            from_z,
            to_x,
            to_y,
            to_z,
            label,
        }
    }
}
