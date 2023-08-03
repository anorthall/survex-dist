use crate::command::fatal_error;
use crate::data::Node;
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;

type PathfindingResult = (Vec<Node>, OrderedFloat<f64>);

pub fn pathfind(start: &Node, end: &Node) -> PathfindingResult {
    let result = astar(
        start,
        |node| node.get_successors(),
        |node| OrderedFloat(node.distance(end)),
        |node| *node == *end,
    );

    if let Some((path, cost)) = result {
        (path, cost)
    } else {
        let msg = format!(
            "Unable to find path between nodes {} and {}.",
            start.label, end.label
        );
        fatal_error(msg);
    }
}
