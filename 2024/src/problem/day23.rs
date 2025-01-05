use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph, Undirected};

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day23"),
        day_id: 23,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(input: Vec<String>) -> Graph<[char; 2], bool, Undirected> {
    let mut graph: Graph<[char; 2], bool, Undirected> = Graph::new_undirected();

    for line in input {
        let a: Vec<[char; 2]> = line
            .split("-")
            .map(|x| x.chars().collect::<Vec<char>>().try_into().unwrap())
            .collect();

        let mut a_id = None;
        let mut b_id = None;
        for (id, i) in graph.node_weights().enumerate() {
            if *i == a[0] {
                a_id = Some(NodeIndex::from(id as u32));
            }
            if *i == a[1] {
                b_id = Some(NodeIndex::from(id as u32));
            }
        }
        if a_id.is_none() {
            a_id = Some(graph.add_node(a[0]));
        }
        if b_id.is_none() {
            b_id = Some(graph.add_node(a[1]));
        }
        graph.add_edge(a_id.unwrap(), b_id.unwrap(), false);
    }

    return graph;
}

fn is_chief(a: &[char; 2], b: &[char; 2], c: &[char; 2]) -> bool {
    return a[0] == 't' || b[0] == 't' || c[0] == 't';
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let graph = parse(input.lines);

    let mut res = 0;

    let mut been = HashSet::new();

    for node_a in graph.node_indices() {
        for node_b in graph.neighbors(node_a) {
            if been.contains(&node_b) {
                continue;
            }
            for node_c in graph.neighbors(node_b) {
                if been.contains(&node_c) {
                    continue;
                }
                if graph.contains_edge(node_a, node_c) {
                    if is_chief(
                        graph.node_weight(node_a).unwrap(),
                        graph.node_weight(node_b).unwrap(),
                        graph.node_weight(node_c).unwrap(),
                    ) {
                        res += 1;
                    }
                }
            }
        }
        been.insert(node_a);
    }

    println!("{}", res / 2);
}

fn bron_kerbosch(
    current: HashSet<(char, char)>,
    mut potential: HashSet<(char, char)>,
    mut done: HashSet<(char, char)>,
    graph: &HashMap<(char, char), HashSet<(char, char)>>,
    res: &mut HashSet<(char, char)>,
) {
    if potential.len() == 0 && done.len() == 0 {
        if current.len() > res.len() {
            *res = current;
        }
        return;
    }

    for el in potential.clone() {
        bron_kerbosch(
            current.iter().copied().chain(once(el)).collect(),
            potential.intersection(&graph[&el]).copied().collect(),
            done.intersection(&graph[&el]).copied().collect(),
            graph,
            res,
        );
        potential.remove(&el);
        done.insert(el);
    }
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut map: HashMap<(char, char), HashSet<(char, char)>> = HashMap::new();

    for line in input.lines {
        let a: Vec<(char, char)> = line
            .split("-")
            .map(|x| x.chars().collect::<Vec<char>>())
            .map(|x| (x[0], x[1]))
            .collect();

        for (fs, sc) in [(a[0], a[1]), (a[1], a[0])] {
            if map.contains_key(&fs) {
                map.get_mut(&fs).unwrap().insert(sc);
            } else {
                let mut s = HashSet::new();
                s.insert(sc);
                map.insert(fs, s);
            }
        }
    }

    let mut c: HashSet<(char, char)> = HashSet::new();

    bron_kerbosch(
        HashSet::new(),
        map.keys().map(|x| *x).collect(),
        HashSet::new(),
        &map,
        &mut c,
    );

    println!(
        "{}",
        c.iter()
            .cloned()
            .sorted()
            .map(|x| format!("{}{}", x.0, x.1))
            .join(",")
    );
}
