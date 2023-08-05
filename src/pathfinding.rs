use crate::command::fatal_error;
use crate::data::Node;
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;

pub fn pathfind_route(route: Vec<&Node>) -> Vec<Node> {
    let mut path: Vec<Node> = Vec::new();
    let mut i = 0;

    while i < route.len() - 1 {
        let start = route[i];
        let end = route[i + 1];
        let sub_path = pathfind(start, end);
        path.pop(); // Remove the last node from the path as it will be duplicated.
        path.extend(sub_path);
        i += 1;
    }

    path
}

fn pathfind<'a>(start: &'a Node, end: &'a Node) -> Vec<Node> {
    let result = astar(
        start,
        |node| node.get_successors(),
        |node| OrderedFloat(node.distance(end)),
        |node| *node == *end,
    );

    if let Some((path, _)) = result {
        path
    } else {
        let msg = format!(
            "Unable to find path between nodes {} and {}.",
            start.label, end.label
        );
        fatal_error(msg);
    }
}
