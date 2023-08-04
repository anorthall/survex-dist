use crate::command::fatal_error;
use crate::data::Node;
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;

pub fn pathfind<'a>(start: &'a Node, end: &'a Node) -> Vec<Node> {
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
