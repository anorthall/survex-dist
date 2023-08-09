use crate::command::fatal_error;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use survex_rs::data::StationGraph;

pub fn pathfind_route(graph: &StationGraph, route: Vec<NodeIndex>) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let mut i = 0;

    while i < route.len() - 1 {
        let start = route[i];
        let end = route[i + 1];
        let (_, sub_path) = pathfind(graph, start, end);

        path.pop();
        path.extend(sub_path);
        i += 1;
    }

    path
}

fn pathfind(graph: &StationGraph, start: NodeIndex, end: NodeIndex) -> (f64, Vec<NodeIndex>) {
    astar(graph, start, |stn| stn == end, |e| *e.weight(), |_| 0.0).unwrap_or_else(|| {
        let msg = format!("Unable to find path from '{:?}' to '{:?}'.", start, end);
        fatal_error(msg);
    })
}
