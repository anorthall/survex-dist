use crate::command::fatal_error;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use survex_rs::data::SurveyData;

pub fn pathfind_route(data: &SurveyData, route: Vec<NodeIndex>) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let mut i = 0;

    while i < route.len() - 1 {
        let start = route[i];
        let end = route[i + 1];
        let (_, sub_path) = pathfind(data, start, end);

        path.pop();
        path.extend(sub_path);
        i += 1;
    }

    path
}

fn pathfind(data: &SurveyData, start: NodeIndex, end: NodeIndex) -> (f64, Vec<NodeIndex>) {
    let end_coords = data.get_by_index(end).unwrap().borrow().coords;
    astar(
        &data.graph,
        start,
        |stn| stn == end,
        |e| *e.weight(),
        |stn| {
            data.get_by_index(stn)
                .unwrap()
                .borrow()
                .coords
                .distance(&end_coords)
        },
    )
    .unwrap_or_else(|| {
        let msg = format!("Unable to find path from '{:?}' to '{:?}'.", start, end);
        fatal_error(msg);
    })
}
