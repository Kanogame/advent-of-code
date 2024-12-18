use std::{
    char,
    collections::{HashMap, HashSet},
};

use itertools::{Diff, Itertools};

use crate::generic_problem::{self, Day};

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day12"),
        day_id: 12,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn get_perimeter(
    x: i32,
    y: i32,
    been: &HashSet<(i32, i32)>,
    lines: &HashMap<(i32, i32), char>,
    letter: char,
) -> i32 {
    match DIRECTIONS
        .iter()
        .map(|[d_x, d_y]| {
            lines.get(&(x + d_x, y + d_y)).is_some_and(|c| *c == letter)
                && been.get(&(x + d_x, y + d_y)).is_some()
        })
        .fold(0, |state, el| state + el as i32)
    {
        4 => -4,
        3 => -2,
        2 => 0,
        1 => 2,
        _ => 4,
    }
}

fn rec_region_p1(
    x: i32,
    y: i32,
    letter: char,
    lines: &HashMap<(i32, i32), char>,
    been: &mut HashSet<(i32, i32)>,
) -> (i32, i32) {
    if !lines.get(&(x, y)).is_some_and(|c| *c == letter) || been.contains(&(x, y)) {
        return (0, 0);
    }
    let mut area = 1;
    let mut perimeter = get_perimeter(x, y, &been, &lines, letter);
    been.insert((x, y));
    for [d_x, d_y] in DIRECTIONS {
        let res = rec_region_p1(x + d_x, y + d_y, letter, lines, been);
        area += res.0;
        perimeter += res.1;
    }

    return (area, perimeter);
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let lines_map = parse_input(input.lines);
    let mut been: HashSet<(i32, i32)> = HashSet::new();
    let mut res = 0;

    for (pos, val) in &lines_map {
        if !been.contains(pos) {
            let rec = rec_region_p1(pos.0, pos.1, *val, &lines_map, &mut been);
            res += rec.0 * rec.1;
        }
    }
    println!("{}", res);
}

fn rec_region_p2(
    x: i32,
    y: i32,
    letter: char,
    lines: &HashMap<(i32, i32), char>,
    group: &mut HashSet<(i32, i32)>,
) {
    if group.contains(&(x, y)) || !lines.get(&(x, y)).is_some_and(|c| *c == letter) {
        return;
    }

    group.insert((x, y));
    for [d_x, d_y] in DIRECTIONS {
        rec_region_p2(x + d_x, y + d_y, letter, lines, group);
    }
}

fn parse_input(lines: Vec<String>) -> HashMap<(i32, i32), char> {
    //somehow faster than for on x.chars().nth(j).unwrap()
    let mut lines = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut lines_map: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            lines_map.insert((j, i), lines[i as usize][j as usize]);
        }
    }

    lines_map
}

fn get_corners(pos: (i32, i32), lines: &HashMap<(i32, i32), char>, current: char) -> i32 {
    let mut cnt = 0;
    //stolen, I was trying to figure out some clever ways of counting corners based on 2x2 array, but had little to no success due to overlapping & groups with similar letters
    //btw lines as hashmap is genius, It would probably take me ages to realize this neat and simple trick
    for ([x, y], [x1, y1]) in DIRECTIONS.iter().circular_tuple_windows() {
        let c_1 = lines
            .get(&(pos.0 + x, &pos.1 + y))
            .is_some_and(|c| *c == current);

        let c_2 = lines
            .get(&(pos.0 + x1, &pos.1 + y1))
            .is_some_and(|c| *c == current);

        if c_1
            && c_2
            && lines
                .get(&(pos.0 + x1 + x, &pos.1 + y1 + y))
                .is_some_and(|c| *c != current)
        {
            cnt += 1;
        } else if !c_1 && !c_2 {
            cnt += 1;
        }
    }
    cnt
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let lines_map = parse_input(input.lines);
    let mut res = 0;
    let mut used: HashSet<(i32, i32)> = HashSet::new();
    let mut group: HashSet<(i32, i32)> = HashSet::new();

    for (pos, val) in &lines_map {
        if !used.contains(pos) {
            rec_region_p2(pos.0, pos.1, *val, &lines_map, &mut group);

            let mut corners = 0;
            for pos in group.iter() {
                used.insert(*pos);
                corners += get_corners(*pos, &lines_map, *val);
            }
            res += group.len() as i32 * corners;
            group.clear();
        }
    }

    println!("{}", res);
}
