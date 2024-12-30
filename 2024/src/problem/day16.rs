use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
};

use petgraph::{graph::NodeIndex, Graph, Undirected};

use crate::{
    aoc_lib::grid::grid::g_sub,
    generic_problem::{self, Day},
};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day16"),
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
    let mut node_map: BTreeMap<(i32, i32), NodeIndex> = BTreeMap::new();

    let mut start: NodeIndex = 0.into();
    let mut end: NodeIndex = 0.into();

    for (pos, value) in lines_map.iter() {
        if *value == '#' {
            continue;
        } else if *value == 'S' {
            let id = graph.add_node(*pos);
            node_map.insert(*pos, id);
            start = id;
        } else if *value == 'E' {
            let id = graph.add_node(*pos);
            node_map.insert(*pos, id);
            end = id;
        } else {
            node_map.insert(*pos, graph.add_node(*pos));
        }
    }

    //scanning left to right, top to bottom for any connections
    for i in graph.clone().raw_nodes() {
        let pos = i.weight;

        for (d0, d1) in [(0, 1), (1, 0)] {
            if lines_map
                .get(&(pos.0 + d0, pos.1 + d1))
                .is_some_and(|a| *a != '#')
            {
                graph.add_edge(
                    *node_map.get(&pos).unwrap(),
                    *node_map.get(&(pos.0 + d0, pos.1 + d1)).unwrap(),
                    0,
                );
            }
        }
    }
    (graph, start, end)
}

fn s_dijkstra(
    graph: Graph<(i32, i32), i32, Undirected>,
    start: NodeIndex,
    end: NodeIndex,
) -> (i32, i32) {
    let mut min_scores: HashMap<(NodeIndex, (i32, i32)), i32> = HashMap::new();
    let mut best_seats: HashSet<NodeIndex> = HashSet::new();
    let mut best_score = -1;

    // I spent so many time on this, binaryHeap - max-heap. Read the docs, idiot
    let mut heap: BinaryHeap<Reverse<(i32, NodeIndex, (i32, i32), Vec<NodeIndex>)>> =
        BinaryHeap::new();

    heap.push(Reverse((0, start, (0, 1), vec![])));
    while heap.len() != 0 {
        let (score, pos, dir, seats) = heap.pop().unwrap().0;
        if pos == end {
            if best_score == -1 || best_score >= score {
                best_seats.extend(seats.iter());
                best_score = score;
                continue;
            }
            break;
        }

        for new_node in graph.neighbors(pos) {
            let new_dir = g_sub(
                *graph.node_weight(new_node).unwrap(),
                *graph.node_weight(pos).unwrap(),
            );

            let new_score = score + if new_dir == dir { 1 } else { 1001 };

            // if node was never visited before
            if min_scores.get(&(new_node, new_dir)).is_none() {
                min_scores.insert((new_node, new_dir), new_score);
            }

            if min_scores
                .get(&(new_node, new_dir))
                .is_some_and(|x| *x >= new_score)
            {
                min_scores.insert((new_node, new_dir), new_score);
                let mut c_s = seats.clone();
                c_s.push(pos);
                heap.push(Reverse((new_score, new_node, new_dir, c_s)));
            }
        }
    }
    (best_score, best_seats.len() as i32 + 1)
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (graph, start, end) = parse_input(input.lines);

    println!("{}", s_dijkstra(graph, start, end).0);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (graph, start, end) = parse_input(input.lines);

    println!("{}", s_dijkstra(graph, start, end).1);
}
