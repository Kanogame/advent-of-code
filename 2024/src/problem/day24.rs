use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Write,
};

use itertools::Itertools;
use petgraph::{
    dot::{Config, Dot},
    graph, Directed, Graph,
};

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day24"),
        day_id: 24,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(lines: Vec<String>) -> (HashMap<String, bool>, VecDeque<String>) {
    let mut values: HashMap<String, bool> = HashMap::new();

    let mut i = 0;
    while lines[i].len() > 0 {
        let st: Vec<String> = lines[i].split(": ").map(|x| x.to_string()).collect();
        values.insert(st[0].clone(), st[1].parse::<i32>().unwrap() != 0);
        i += 1;
    }
    i += 1;

    return (values, VecDeque::from_iter(lines[i..].iter().cloned()));
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (mut wires, mut logic) = parse(input.lines);

    while let Some(gate) = logic.pop_front() {
        let syntax: Vec<&str> = gate.split(" ").collect();
        let res = syntax[4].to_string();
        let a = wires.get(syntax[0]);
        let b = wires.get(syntax[2]);
        if a == None || b == None {
            logic.push_back(gate);
            continue;
        }
        let a = *a.unwrap();
        let b = *b.unwrap();
        wires.insert(
            res,
            match syntax[1] {
                "AND" => a && b,
                "OR" => a || b,
                "XOR" => a ^ b,
                _ => false,
            },
        );
    }

    println!(
        "{:?}",
        wires
            .iter()
            .filter(|(a, _)| (**a).chars().nth(0).unwrap() == 'z')
            .sorted_by(|(a1, _), (a2, _)| (**a1).cmp(*a2))
            .rev()
            .collect::<Vec<(&String, &bool)>>()
    );

    let res = wires
        .iter()
        .filter(|(a, _)| (**a).chars().nth(0).unwrap() == 'z')
        .sorted_by(|(a1, _), (a2, _)| (**a1).cmp(*a2))
        .rev()
        .fold(0_i64, |val, (_, b)| (val << 1) + if *b { 1 } else { 0 });

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (_, mut logic) = parse(input.lines);

    let mut gh_id = HashMap::new();

    let mut graph: Graph<&str, bool, Directed> = Graph::new();

    let mut i = 0;
    while i < logic.len() {
        let syntax: Vec<&str> = logic[i].split(" ").collect();
        let res = if gh_id.contains_key(syntax[4]) {
            *gh_id.get(syntax[4]).unwrap()
        } else {
            let id = graph.add_node(syntax[4]);
            gh_id.insert(syntax[4], id);
            id
        };

        let a = if gh_id.contains_key(syntax[0]) {
            *gh_id.get(syntax[0]).unwrap()
        } else {
            let id = graph.add_node(syntax[0]);
            gh_id.insert(syntax[0], id);
            id
        };
        let b = if gh_id.contains_key(syntax[2]) {
            *gh_id.get(syntax[2]).unwrap()
        } else {
            let id = graph.add_node(syntax[2]);
            gh_id.insert(syntax[2], id);
            id
        };

        let gate_id = graph.add_node(syntax[1]);

        graph.add_edge(a, gate_id, true);
        graph.add_edge(b, gate_id, true);
        graph.add_edge(gate_id, res, true);
        i += 1;
    }

    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("graph.dot").unwrap();
    file.write_all(dot_output.as_bytes()).unwrap();

    println!(
        "{}",
        vec!["z07", "swt", "z13", "pqc", "wsv", "rjm", "bgs", "z31"]
            .iter()
            .sorted()
            .join(",")
    );
}
