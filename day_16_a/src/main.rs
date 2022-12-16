use std::{error::Error, fs};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    flow_rate: i32,
    connections: Vec<String>,
}

fn solve(
    time: i32,
    released_flow: i32,
    pos: i32,
    distances: &std::collections::HashMap<(i32, i32), i32>,
    flow_map: &std::collections::HashMap<i32, i32>,
    to_visit: std::collections::HashSet<i32>,
    best_released_flow: &mut i32,
) {
    if released_flow > *best_released_flow {
        *best_released_flow = released_flow;
    }
    if to_visit.is_empty() || time <= 0 {
        return;
    }

    for new_pos in &to_visit {
        let dist = distances.get(&(pos, *new_pos)).unwrap();

        let new_time = time - dist - 1;
        let flow_rate = flow_map.get(new_pos).unwrap();

        let new_released_flow = released_flow + flow_rate * new_time;

        let mut new_to_visit = to_visit.clone();
        new_to_visit.remove(new_pos);

        solve(
            new_time,
            new_released_flow,
            *new_pos,
            distances,
            flow_map,
            new_to_visit,
            best_released_flow,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let nodes = fs::read_to_string("input")?
        .replace("rate=", "")
        .replace([';', ','], "")
        .lines()
        .map(|s| {
            //Valve EV has flow 0 tunnels lead to valves WG IB
            let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
            Valve {
                name: parts[1].to_owned(),
                flow_rate: parts[4].parse::<i32>().unwrap(),
                connections: parts[9..].iter().map(|s| (*s).to_owned()).collect(),
            }
        })
        .collect::<Vec<_>>();

    let valves = nodes
        .iter()
        .filter(|n| n.flow_rate != 0)
        .map(|n| (*n).clone())
        .collect::<Vec<Valve>>();

    let mut flow_map = std::collections::HashMap::new();
    for v in &valves {
        flow_map.insert(&v.name, v.flow_rate);
    }

    let origin = "AA".to_owned();
    let mut distances = std::collections::HashMap::new();
    for i in 0..valves.len() + 1 {
        let v0 = if i != 0 { &valves[i - 1].name } else { &origin };
        for j in 0..valves.len() {
            let v1 = &valves[j].name;
            let dist = pathfinding::prelude::dijkstra(
                v0,
                |p| {
                    nodes
                        .iter()
                        .find(|n| n.name == *p)
                        .unwrap()
                        .connections
                        .iter()
                        .map(|s| (s.to_owned(), 1))
                },
                |p| p == v1,
            )
            .unwrap();
            distances.insert((v0, v1), dist.1);
            distances.insert((v1, v0), dist.1);
        }
    }

    let mut index_map = std::collections::HashMap::new();
    for n in nodes.iter().enumerate() {
        index_map.insert(n.1.name.clone(), n.0);
    }

    let mut indexed_distances = std::collections::HashMap::new();
    for d in &distances {
        let i0 = index_map.get(d.0 .0).unwrap();
        let i1 = index_map.get(d.0 .1).unwrap();
        indexed_distances.insert((*i0 as i32, *i1 as i32), *d.1);
    }

    let mut indexed_flow_map = std::collections::HashMap::new();
    for f in flow_map {
        let i = index_map.get(f.0).unwrap();
        indexed_flow_map.insert(*i as i32, f.1);
    }

    let mut to_visit = std::collections::HashSet::new();
    for v in &valves {
        to_visit.insert(*index_map.get(&v.name).unwrap() as i32);
    }

    let mut best_released_flow = 0;

    solve(
        30,
        0,
        *index_map.get("AA").unwrap() as i32,
        &indexed_distances,
        &indexed_flow_map,
        to_visit,
        &mut best_released_flow,
    );

    println!("{best_released_flow}");
    // 1743 too low

    Ok(())
}
