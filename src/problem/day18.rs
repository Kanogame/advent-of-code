use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use regex::Regex;

use crate::{
    aoc_lib::grid::grid::{g_add, DIRECTIONS},
    generic_problem::{self, Day},
};

const GRID_SIZE: i32 = 71;
const BYTES: usize = 1024;

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day18"),
        day_id: 18,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(lines: Vec<String>) -> Vec<(i32, i32)> {
    let re: Regex = Regex::new(r"([0-9]+),([0-9]+)").unwrap();

    let mut res: Vec<(i32, i32)> = Vec::new();
    for line in lines {
        for [b_x, b_y] in re
            .captures_iter(&line)
            .map(|x| x.extract().1.map(|el| el.parse::<i32>().unwrap()))
        {
            res.push((b_y, b_x));
        }
    }

    res
}

fn sim(bytes: Vec<(i32, i32)>) -> HashMap<(i32, i32), char> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            grid.insert((i, j), '.');
        }
    }

    for (y, x) in bytes[0..BYTES].iter() {
        grid.insert((*y, *x), '#');
    }

    grid
}

fn s_dijkstra(grid: &HashMap<(i32, i32), char>) -> (i32, HashSet<(i32, i32)>) {
    let mut min_scores: HashMap<(i32, i32), (i32, (i32, i32))> = HashMap::new();
    let mut best_score = -1;

    let mut heap: BinaryHeap<Reverse<(i32, (i32, i32))>> = BinaryHeap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    heap.push(Reverse((0, (0, 0))));

    while heap.len() > 0 {
        let (score, pos) = heap.pop().unwrap().0;
        if visited.contains(&pos) {
            continue;
        }

        if pos == (GRID_SIZE - 1, GRID_SIZE - 1) {
            best_score = score;
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

    let mut best_path = HashSet::new();

    if best_score == -1 {
        return (best_score, HashSet::new());
    }

    let mut prev = min_scores.get(&(GRID_SIZE - 1, GRID_SIZE - 1)).unwrap().1;
    while prev != (0, 0) {
        prev = min_scores.get(&prev).unwrap().1;
        best_path.insert(prev);
    }

    (best_score, best_path)
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let grid = sim(parse(input.lines));

    println!("{}", s_dijkstra(&grid).0);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let bytes = parse(input.lines);

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            grid.insert((i, j), '.');
        }
    }

    let mut i = 0;

    let (_, mut path) = s_dijkstra(&grid);

    loop {
        grid.insert(bytes[i], '#');
        if path.contains(&bytes[i]) {
            let (value, new_path) = s_dijkstra(&grid);
            if value == -1 {
                break;
            }
            path = new_path;
        }
        //println!("{}", i);
        i += 1;
    }

    println!("{},{}", bytes[i].1, bytes[i].0);
}
