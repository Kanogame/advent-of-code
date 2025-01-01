use std::collections::HashMap;

use crate::{
    aoc_lib::grid::grid::{g_add, DIRECTIONS},
    generic_problem::{self, Day},
};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("test"),
        day_id: 21,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

#[rustfmt::skip]
const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['6', '5', '4'],
    ['3', '2', '1'],
    [' ', '0', 'A'],
];

#[rustfmt::skip]
const DIRPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>'],
];

const DIR_CH: [char; 4] = ['>', 'v', '<', '^'];

// to much
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

//get rid of zig-zags
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
    prev: char,
    seq: Vec<char>,
    current_path: Vec<char>,
    path: &mut Vec<Vec<char>>,
    b2b: &HashMap<(char, char), Vec<Vec<char>>>,
) {
    if seq.len() == 0 {
        path.push(current_path);
        return;
    }

    let cache = b2b.get(&(prev, seq[0])).unwrap();
    for i in cache.clone().iter_mut() {
        let mut new_path = current_path.clone();
        new_path.append(i);
        new_path.push('A');
        translate(seq[0], seq[1..].to_vec(), new_path, path, b2b);
    }
}

fn d2d_rec(
    paths: Vec<Vec<char>>,
    depth: i32,
    dir_b2b: &HashMap<(char, char), Vec<Vec<char>>>,
    cache: &mut HashMap<(Vec<char>, i32), Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if depth == 0 {
        return paths;
    }

    let mut res_paths = Vec::new();
    for i in paths.iter() {
        let mut new_paths = Vec::new();
        if cache.contains_key(&(i.clone(), depth)) {
            res_paths.append(&mut (cache.get(&(i.clone(), depth)).unwrap()).clone());
        } else {
            translate('A', i.clone(), vec![], &mut new_paths, dir_b2b);
            cache.insert((i.clone(), depth), new_paths.clone());
        }
        res_paths.append(&mut d2d_rec(new_paths, depth - 1, dir_b2b, cache));
    }

    res_paths
}

fn run_part_one(input: String) -> i32 {
    let (num_b2b, dir_b2b) = get_b2b_cache();

    //number -> button
    let mut path = Vec::new();
    translate('A', input.chars().collect(), vec![], &mut path, &num_b2b);

    let mut cache: HashMap<(Vec<char>, i32), Vec<Vec<char>>> = HashMap::new();
    let path_d2 = d2d_rec(path, 2, &dir_b2b, &mut cache);
    let mut min = path_d2[0].len();
    for i in path_d2 {
        if i.len() < min {
            println!("{}", min);
            min = i.len();
        }
    }
    println!("{}", min);
    return min as i32;
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut res = 0;
    for i in input.lines.iter() {
        res += (i[..i.len() - 1].to_string().parse::<i32>().unwrap()) * run_part_one(i.clone());
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    //
}
