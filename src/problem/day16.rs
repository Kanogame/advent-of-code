use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
    graph::{self, NodeIndex},
    visit::{EdgeRef, VisitMap, Visitable},
    Graph, Undirected,
};

use crate::generic_problem::{self, Day};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("test"),
        day_id: 16,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse_input(lines: Vec<String>) -> (Graph<(i32, i32), i32, Undirected>, NodeIndex, NodeIndex) {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut lines_map: BTreeMap<(i32, i32), char> = BTreeMap::new();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            lines_map.insert((i as i32, j as i32), lines[i][j]);
        }
    }

    build_graph(lines_map)
}

fn build_graph(
    lines_map: BTreeMap<(i32, i32), char>,
) -> (Graph<(i32, i32), i32, Undirected>, NodeIndex, NodeIndex) {
    let mut graph: Graph<(i32, i32), i32, Undirected> = Graph::new_undirected();
    let mut nodeMap: BTreeMap<(i32, i32), NodeIndex> = BTreeMap::new();

    let mut start: NodeIndex = 0.into();
    let mut end: NodeIndex = 0.into();

    for (pos, value) in lines_map.iter() {
        if *value == '#' {
            continue;
        } else if *value == 'S' {
            let id = graph.add_node(*pos);
            nodeMap.insert(*pos, id);
            start = id;
        } else if *value == 'E' {
            let id = graph.add_node(*pos);
            nodeMap.insert(*pos, id);
            end = id;
        } else {
            nodeMap.insert(*pos, graph.add_node(*pos));
        }
    }

    //scanning left to right, top to bottom for any connections
    for i in graph.clone().raw_nodes() {
        let pos = i.weight;

        //right
        let mut x = pos.1 + 1;
        while lines_map.get(&(pos.0, x)).is_some_and(|a| *a != '#') {
            if nodeMap.get(&(pos.0, x)).is_some() {
                graph.add_edge(
                    *nodeMap.get(&pos).unwrap(),
                    *nodeMap.get(&(pos.0, x)).unwrap(),
                    x - pos.1,
                );
                break;
            }
            x += 1;
        }

        //bottom
        let mut y = pos.0 + 1;
        while lines_map.get(&(y, pos.1)).is_some_and(|a| *a != '#') {
            if nodeMap.get(&(y, pos.1)).is_some() {
                graph.add_edge(
                    *nodeMap.get(&pos).unwrap(),
                    *nodeMap.get(&(y, pos.1)).unwrap(),
                    y - pos.0,
                );
                break;
            }
            y += 1;
        }
    }

    println!(
        "{:?}",
        Dot::with_config(&graph, &[Config::GraphContentOnly])
    );
    println!("{:?}", (start, end));
    (graph, start, end)
}

fn s_dijkstra(graph: Graph<(i32, i32), i32, Undirected>, start: NodeIndex, end: NodeIndex) {
    min_scores: HashMap<>;
    best_a: HashSet<>;
    best_score = -1;
    heap = BinaryHeap<(0, start, (0, 1), [])> BinaryHeap::new();

    while heap.len() > 0 {
        score, pos, dir, seats = heap.pop();
        if pos == 
    }
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (graph, start, end) = parse_input(input.lines);

    s_dijkstra(graph, start, end);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    //
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
