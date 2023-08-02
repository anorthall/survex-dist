use crate::data::{Headers, Leg, Node, Point};
use log::{info, trace};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Parsed = (Headers, Vec<Node>, Vec<Leg>);

pub fn parse_dump3d(file: File) -> Result<Parsed, Box<dyn Error>> {
    // Create a reader
    let mut reader = BufReader::new(&file);
    let mut current_line = 0;

    // Parse headers
    info!("Reading headers.");
    let headers = parse_headers(&mut reader, &mut current_line)?;
    info!("Parsed {} header lines.", current_line - 1);
    info!("{}", headers);

    // Parse the data in the file
    // Reference for dump3d format: https://github.com/ojwb/survex/blob/master/src/dump3d.c
    let mut legs = Vec::new();
    let mut nodes = Vec::new();
    info!("Parsing data from dump3d file.");
    for line in reader.lines() {
        current_line += 1;
        let line = line?;
        if let Some(param) = line.strip_prefix("NODE ") {
            match parse_node(param.to_string()) {
                Ok(node) => {
                    trace!("Parsed NODE: {:?}", &node);
                    nodes.push(node);
                }
                Err(e) => {
                    return Err(
                        format!("Unable to parse node on line {}: {}", current_line, e).into(),
                    );
                }
            }
        } else if let Some(param) = line.strip_prefix("LEG ") {
            match parse_leg(param.to_string()) {
                Ok(leg) => {
                    trace!("Parsed LEG: {:?}", &leg);
                    legs.push(leg);
                }
                Err(e) => {
                    return Err(
                        format!("Unable to parse leg on line {}: {}", current_line, e).into(),
                    );
                }
            }
        } else if let Some(param) = line.strip_prefix("MOVE ") {
            // TODO: Parse MOVE
            trace!("Saw MOVE: {}", param);
        } else if let Some(param) = line.strip_prefix("LINE ") {
            // TODO: Parse LINE
            trace!("Saw LINE: {}", param);
        } else if let Some(param) = line.strip_prefix("XSECT ") {
            // TODO: Parse XSECT
            // This will involve parsing multiple lines followed by XSECT_END
            trace!("Saw XSECT: {}", param);
        } else if let Some(param) = line.strip_prefix("ERROR_INFO ") {
            // TODO: Parse ERROR_INFO
            trace!("Saw ERROR_INFO: {}", param);
        } else if line == "STOP" {
            info!(
                "STOP reached. {} lines processed. Parsing complete.",
                current_line
            );
            let num_nodes = nodes.len();
            let num_legs = legs.len();
            info!("Parsed {} nodes.", num_nodes);
            info!("Parsed {} legs.", num_legs);
            break;
        } else {
            trace!("Unknown line: {}", line);
        }
    }

    Ok((headers, nodes, legs))
}

fn parse_headers(
    reader: &mut BufReader<&File>,
    current_line: &mut u64,
) -> Result<Headers, Box<dyn Error>> {
    let mut title = String::new();
    let mut date = String::new();
    let mut date_numeric = 0_u64;
    let mut cs = None;
    let mut version = 0_u8;
    let mut extended_elev = false;
    let mut separator = ' ';

    let mut buffer = String::new();
    loop {
        *current_line += 1;

        buffer.clear();
        let _ = reader.read_line(&mut buffer)?;
        let buffer = buffer.trim_end().to_string();

        trace!("Header found: {}", buffer);
        if let Some(param) = buffer.strip_prefix("TITLE \"") {
            title = param.to_string();
            title.pop();
        } else if let Some(param) = buffer.strip_prefix("DATE \"") {
            date = param.to_string();
            date.pop();
        } else if let Some(param) = buffer.strip_prefix("DATE_NUMERIC ") {
            date_numeric = param.parse::<u64>()?;
        } else if let Some(param) = buffer.strip_prefix("CS ") {
            cs = Some(param.to_string());
        } else if let Some(param) = buffer.strip_prefix("VERSION ") {
            version = param.parse::<u8>()?;
        } else if let Some(param) = buffer.strip_prefix("SEPARATOR '") {
            separator = param.chars().next().unwrap();
        } else if buffer == "EXTENDED ELEVATION" {
            extended_elev = true;
        } else if buffer == "--" {
            trace!("End of headers.");
            break;
        } else {
            return Err(format!("Unknown header: {}", buffer).into());
        }
    }

    Ok(Headers::new(
        title,
        date,
        date_numeric,
        cs,
        version,
        extended_elev,
        separator,
    ))
}

fn parse_node(node_line: String) -> Result<Node, Box<dyn Error>> {
    let split = node_line.split(' ');
    let params = split.collect::<Vec<&str>>();

    if params.len() < 4 {
        return Err(format!("Invalid node line: {}", node_line).into());
    }

    let coords = Point::new(
        params[0].parse::<f64>()?,
        params[1].parse::<f64>()?,
        params[2].parse::<f64>()?,
    );
    let mut label = params[3][1..].to_string();
    label.pop(); // Remove trailing ']'

    let mut node = Node::new(coords, label);

    if params.len() > 4 {
        for param in params[4..].iter() {
            match *param {
                "SURFACE" => node.surface = true,
                "UNDERGROUND" => node.underground = true,
                "ENTRANCE" => node.entrance = true,
                "EXPORTED" => node.exported = true,
                "FIXED" => node.fixed = true,
                "ANON" => node.anon = true,
                "WALL" => node.wall = true,
                _ => return Err(format!("Unknown node parameter: {}", param).into()),
            }
        }
    }

    Ok(node)
}

fn parse_leg(leg_line: String) -> Result<Leg, Box<dyn Error>> {
    let split = leg_line.split(' ');
    let params = split.collect::<Vec<&str>>();

    if params.len() < 7 {
        return Err(format!("Invalid leg line: {}", leg_line).into());
    }

    let from_coords = Point::new(
        params[0].parse::<f64>()?,
        params[1].parse::<f64>()?,
        params[2].parse::<f64>()?,
    );
    let to_coords = Point::new(
        params[3].parse::<f64>()?,
        params[4].parse::<f64>()?,
        params[5].parse::<f64>()?,
    );

    let mut label = None;
    let mut raw_label = params[6][1..].trim().to_string();
    raw_label.pop(); // Remove trailing ']' from label

    if !raw_label.is_empty() {
        label = Some(raw_label.to_string());
    }

    Ok(Leg::new(from_coords, to_coords, label))
}

#[cfg(test)]
mod tests {
    use crate::data::Point;

    #[test]
    fn test_all_valid_headers() {
        let file = std::fs::File::open("tests/files/valid_headers.txt").unwrap();
        let mut current_line = 0;
        let mut reader = std::io::BufReader::new(&file);
        let headers = super::parse_headers(&mut reader, &mut current_line).unwrap();
        assert_eq!(headers.title, "Test Headers");
        assert_eq!(headers.date, "@1690877650");
        assert_eq!(headers.date_numeric, 1690877650);
        assert_eq!(headers.cs, Some(String::from("UTM60S")));
        assert_eq!(headers.version, 8);
        assert_eq!(headers.extended_elev, true);
        assert_eq!(headers.separator, '.');
    }

    #[test]
    fn test_valid_headers_without_extended_elev_or_cs() {
        let file = std::fs::File::open("tests/files/valid_headers_without_optional.txt").unwrap();
        let mut current_line = 0;
        let mut reader = std::io::BufReader::new(&file);
        let headers = super::parse_headers(&mut reader, &mut current_line).unwrap();
        assert_eq!(headers.title, "Test Headers");
        assert_eq!(headers.date, "@1690877650");
        assert_eq!(headers.date_numeric, 1690877650);
        assert_eq!(headers.cs, None);
        assert_eq!(headers.version, 8);
        assert_eq!(headers.extended_elev, false);
        assert_eq!(headers.separator, '.');
    }

    #[test]
    fn test_invalid_headers() {
        let file = std::fs::File::open("tests/files/invalid_headers.txt").unwrap();
        let mut current_line = 0;
        let mut reader = std::io::BufReader::new(&file);
        let headers = super::parse_headers(&mut reader, &mut current_line);
        assert!(headers.is_err());
        assert!(headers
            .unwrap_err()
            .to_string()
            .starts_with("Unknown header: INVALID"));
    }

    #[test]
    fn test_parse_valid_node_without_flags() {
        let basic_node = String::from("12345.00 67890.00 100.00 [basic]");
        let node = super::parse_node(basic_node).unwrap();
        assert_eq!(node.coords, Point::new(12345.00, 67890.00, 100.00));
        assert_eq!(node.label, "basic");
        assert_eq!(node.underground, false);
        assert_eq!(node.surface, false);
        assert_eq!(node.entrance, false);
        assert_eq!(node.exported, false);
        assert_eq!(node.fixed, false);
        assert_eq!(node.anon, false);
        assert_eq!(node.wall, false);
    }

    #[test]
    fn test_parse_valid_node_with_flags() {
        let all_flags = String::from("54321.00 09876.00 200.00 [all_flags] SURFACE UNDERGROUND ENTRANCE EXPORTED FIXED ANON WALL");
        let node = super::parse_node(all_flags).unwrap();
        assert_eq!(node.coords, Point::new(54321.00, 09876.00, 200.00));
        assert_eq!(node.label, "all_flags");
        assert_eq!(node.underground, true);
        assert_eq!(node.surface, true);
        assert_eq!(node.entrance, true);
        assert_eq!(node.exported, true);
        assert_eq!(node.fixed, true);
        assert_eq!(node.anon, true);
        assert_eq!(node.wall, true);
    }

    #[test]
    fn test_invalid_node_flag() {
        let invalid_flag = String::from("12345.00 67890.00 100.00 [invalid] INVALID");
        let node = super::parse_node(invalid_flag);
        assert!(node.is_err());
        assert!(node
            .unwrap_err()
            .to_string()
            .starts_with("Unknown node parameter: INVALID"));
    }

    #[test]
    fn test_poorly_formatted_node_line() {
        let invalid_node = String::from("12345 67890 [invalid]");
        let node = super::parse_node(invalid_node.clone());
        assert!(node.is_err());
        assert!(node
            .unwrap_err()
            .to_string()
            .eq(&format!("Invalid node line: {}", invalid_node)));
    }

    #[test]
    fn test_parse_valid_leg_without_label() {
        let valid_leg = String::from("66660.17 78509.31 296.16 66660.17 78509.31 297.74 []");
        let leg = super::parse_leg(valid_leg).unwrap();
        assert_eq!(leg.from_coords, Point::new(66660.17, 78509.31, 296.16));
        assert_eq!(leg.to_coords, Point::new(66660.17, 78509.31, 297.74));
        assert_eq!(leg.label, None);
    }

    #[test]
    fn test_parse_valid_leg_with_label() {
        let valid_leg =
            String::from("66660.17 78509.31 296.16 66660.17 78509.31 297.74 [valid_leg]");
        let leg = super::parse_leg(valid_leg).unwrap();
        assert_eq!(leg.from_coords, Point::new(66660.17, 78509.31, 296.16));
        assert_eq!(leg.to_coords, Point::new(66660.17, 78509.31, 297.74));
        assert_eq!(leg.label, Some(String::from("valid_leg")));
    }

    #[test]
    fn test_poorly_formatted_leg_line() {
        let invalid_leg = String::from("66660.17 78509.31 296.16 66660.17 78509.31 297.74");
        let leg = super::parse_leg(invalid_leg.clone());
        assert!(leg.is_err());
        assert!(leg
            .unwrap_err()
            .to_string()
            .eq(&format!("Invalid leg line: {}", invalid_leg)));
    }
}
