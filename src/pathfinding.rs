use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use survex_rs::data::SurveyData;

pub fn pathfind_route(data: &SurveyData, route: Vec<NodeIndex>) -> Option<Vec<NodeIndex>> {
    let mut path = Vec::new();
    let mut i = 0;

    while i < route.len() - 1 {
        let start = route[i];
        let end = route[i + 1];
        let result = pathfind(data, start, end);

        if result.is_some() {
            let (_, sub_path) = result.unwrap();
            // Remove the last element of the previous path, as it will be duplicated.
            path.pop();
            path.extend(sub_path);
            i += 1;
        } else {
            return None;
        }
    }

    Some(path)
}

fn pathfind(data: &SurveyData, start: NodeIndex, end: NodeIndex) -> Option<(f64, Vec<NodeIndex>)> {
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
}
