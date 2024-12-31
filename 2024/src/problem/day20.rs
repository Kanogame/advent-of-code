use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{
    aoc_lib::grid::grid::{g_add, DIRECTIONS},
    generic_problem::{self, Day},
};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day20"),
        day_id: 20,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse_input(lines: Vec<String>) -> (HashMap<(i32, i32), char>, (i32, i32), (i32, i32)) {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut lines_map: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            lines_map.insert((i as i32, j as i32), lines[i][j]);
            if lines[i][j] == 'S' {
                start = (i as i32, j as i32);
            }

            if lines[i][j] == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    (lines_map, start, end)
}

fn s_dijkstra(
    grid: &HashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
) -> Vec<((i32, i32), i32)> {
    let mut min_scores: HashMap<(i32, i32), (i32, (i32, i32))> = HashMap::new();

    let mut heap: BinaryHeap<Reverse<(i32, (i32, i32))>> = BinaryHeap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut best_path: Vec<((i32, i32), i32)> = Vec::new();

    heap.push(Reverse((0, start)));

    while heap.len() > 0 {
        let (score, pos) = heap.pop().unwrap().0;
        best_path.push((pos, score));
        if visited.contains(&pos) {
            continue;
        }

        if pos == end {
            continue;
        }

        for i in DIRECTIONS {
            let new_pos = g_add(i, pos);
            if visited.contains(&new_pos) {
                continue;
            }

            if min_scores.get(&new_pos).is_none() {
                min_scores.insert(new_pos, (score + 1, pos));
            }

            if min_scores
                .get(&new_pos)
                .is_some_and(|(x, _)| *x >= score + 1)
                && grid.get(&new_pos).is_some_and(|x| *x != '#')
            {
                min_scores.insert(new_pos, (score + 1, pos));
                heap.push(Reverse((score + 1, new_pos)));
            }
        }

        visited.insert(pos);
    }

    best_path
}

fn search(path: Vec<((i32, i32), i32)>, radius: i32) -> i32 {
    let mut shortcuts = 0;
    for (id, (a, cost_a)) in path.iter().enumerate() {
        let (a_x, a_y) = *a;

        for j in id..path.len() {
            let ((b_x, b_y), cost_b) = path[j];

            let distance = (a_x - b_x).abs() + (a_y - b_y).abs();
            if distance < radius + 1 && cost_b - cost_a - distance >= 100 {
                shortcuts += 1;
            }
        }
    }
    shortcuts
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (map, start, end) = parse_input(input.lines);
    let res = search(s_dijkstra(&map, start, end), 2);

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (map, start, end) = parse_input(input.lines);
    let res = search(s_dijkstra(&map, start, end), 20);

    println!("{}", res);
}
