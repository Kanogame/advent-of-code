use std::{collections::HashMap, i64::MAX};

use crate::{
    aoc_lib::grid::grid::{g_add, DIRECTIONS},
    generic_problem::{self, Day},
};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day21"),
        day_id: 21,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

// fr?
#[rustfmt::skip]
const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

#[rustfmt::skip]
const DIRPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>'],
];

const DIR_CH: [char; 4] = ['>', 'v', '<', '^'];

fn bfs_pad(
    map: &HashMap<(i32, i32), char>,
    current: (i32, i32),
    goal: (i32, i32),
    current_path: Vec<char>,
    max_len: usize,
    paths: &mut Vec<Vec<char>>,
) {
    if current == goal {
        paths.push(current_path);
        return;
    }

    if current_path.len() > max_len {
        return;
    }
    for (i, dir_val) in DIRECTIONS.iter().enumerate() {
        let new_pos = g_add(*dir_val, current);
        if map.get(&new_pos).is_some_and(|x| *x != ' ') {
            let mut new_path = current_path.clone();
            new_path.push(DIR_CH[i]);
            bfs_pad(map, new_pos, goal, new_path.clone(), max_len, paths);
        }
    }
}

fn fastest_b2b_paths(map: HashMap<(i32, i32), char>) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut b2b = HashMap::new();

    for (a_pos, a) in map.clone() {
        for (b_pos, b) in map.clone() {
            let mut paths = Vec::new();
            let diff_y = a_pos.0 - b_pos.0;
            let diff_x = a_pos.1 - b_pos.1;
            let max_len = diff_x.abs() + diff_y.abs();

            bfs_pad(&map, a_pos, b_pos, vec![], max_len as usize, &mut paths);
            b2b.insert((a, b), paths);
        }
    }

    b2b
}

fn get_b2b_cache() -> (
    HashMap<(char, char), Vec<Vec<char>>>,
    HashMap<(char, char), Vec<Vec<char>>>,
) {
    let mut num_map: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..NUMPAD.len() {
        for j in 0..NUMPAD[0].len() {
            num_map.insert((i as i32, j as i32), NUMPAD[i][j]);
        }
    }
    let num_b2b = fastest_b2b_paths(num_map);

    let mut dir_map: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..DIRPAD.len() {
        for j in 0..DIRPAD[0].len() {
            dir_map.insert((i as i32, j as i32), DIRPAD[i][j]);
        }
    }

    let dir_b2b = fastest_b2b_paths(dir_map);
    (num_b2b, dir_b2b)
}

fn translate(
    keys: &Vec<char>,
    index: usize,
    prev_key: char,
    current_path: Vec<char>,
    result: &mut Vec<Vec<char>>,
    b2b: &HashMap<(char, char), Vec<Vec<char>>>,
) {
    if index > keys.len() - 1 {
        result.push(current_path);
        return;
    }

    for path in b2b.get(&(prev_key, keys[index])).unwrap() {
        let mut new_path = current_path.clone();
        new_path.append(&mut path.clone());
        new_path.push('A');
        translate(keys, index + 1, keys[index].clone(), new_path, result, b2b);
    }
}

fn shortest_seq(
    keys: Vec<char>,
    depth: i32,
    cache: &mut HashMap<(Vec<char>, i32), i64>,
    dir_b2b: &HashMap<(char, char), Vec<Vec<char>>>,
) -> i64 {
    if depth == 0 {
        return keys.len() as i64;
    }

    if cache.get(&(keys.clone(), depth)).is_some() {
        return *cache.get(&(keys.clone(), depth)).unwrap();
    }

    let subkey: Vec<Vec<char>> = keys
        .split_inclusive(|x| *x == 'A')
        .map(|x| x.to_vec())
        .filter(|x: &Vec<char>| x.len() != 0)
        .collect();

    let mut total = 0;
    for i in subkey {
        let mut path = Vec::new();
        translate(&i, 0, 'A', vec![], &mut path, &dir_b2b);

        let mut min = MAX;
        for j in path {
            let tmp = shortest_seq(j, depth - 1, cache, dir_b2b);
            if tmp < min {
                min = tmp;
            }
        }
        total += min;
    }

    cache.insert((keys, depth), total);

    total
}

fn run(input: Vec<String>, depth: i32) -> i64 {
    let (num_b2b, dir_b2b) = get_b2b_cache();

    let mut cache = HashMap::new();
    let mut res: i64 = 0;

    for line in input {
        //number -> button
        let mut path = Vec::new();
        translate(&line.chars().collect(), 0, 'A', vec![], &mut path, &num_b2b);

        let mut min = MAX;
        for j in path {
            let tmp = shortest_seq(j, depth, &mut cache, &dir_b2b);
            if tmp < min {
                min = tmp;
            }
        }

        res += line[0..line.len() - 1].parse::<i64>().unwrap() * min;
    }

    res
}

pub fn part_one(input: generic_problem::ProblemInput) {
    println!("{}", run(input.lines, 2));
}

pub fn part_two(input: generic_problem::ProblemInput) {
    println!("{}", run(input.lines, 25));
}
