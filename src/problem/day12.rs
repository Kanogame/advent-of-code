use std::{
    char,
    collections::{hash_set, BTreeSet, HashMap, HashSet},
    ptr::null,
};

use itertools::Itertools;

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

fn get_perimeter(diff_x: i32, diff_y: i32, letter: char, lines: &Vec<Vec<char>>) -> i32 {
    let left = diff_x - 1 >= 0 && lines[diff_y as usize][(diff_x - 1) as usize] == letter;
    let right = diff_x + 1 < lines[0].len() as i32
        && lines[diff_y as usize][(diff_x + 1) as usize] == letter;
    let top = diff_y - 1 >= 0 && lines[(diff_y - 1) as usize][diff_x as usize] == letter;
    let bottom =
        diff_y + 1 < lines.len() as i32 && lines[(diff_y + 1) as usize][diff_x as usize] == letter;
    let sum = left as i32 + right as i32 + top as i32 + bottom as i32;

    if sum == 4 {
        return -4;
    } else if sum == 3 {
        return -2;
    } else if sum == 2 {
        return 0;
    } else if sum == 1 {
        return 2;
    }
    return 4;
}

fn rec_region_p1(
    x: i32,
    y: i32,
    letter: char,
    replace: char,
    lines: &mut Vec<Vec<char>>,
) -> (i32, i32) {
    if lines[y as usize][x as usize] != letter || lines[y as usize][x as usize] == replace {
        return (0, 0);
    }
    let mut area = 1;
    let mut perimeter = get_perimeter(x, y, replace, &lines);
    lines[y as usize][x as usize] = replace;
    if x - 1 >= 0 {
        let res = rec_region_p1(x - 1, y, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if x + 1 < lines[0].len() as i32 {
        let res = rec_region_p1(x + 1, y, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if y - 1 >= 0 {
        let res = rec_region_p1(x, y - 1, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if y + 1 < lines.len() as i32 {
        let res = rec_region_p1(x, y + 1, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }

    return (area, perimeter);
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut res = 0;

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            let ch = lines[i as usize][j as usize];
            if ch.is_uppercase() {
                let rec = rec_region_p1(j, i, ch, ch.to_ascii_lowercase(), &mut lines);
                res += rec.0 * rec.1;
            }
        }
    }
    println!("{}", res);
}

fn rec_region_p2(
    x: i32,
    y: i32,
    letter: char,
    lines: &Vec<Vec<char>>,
    group: &mut HashSet<(i32, i32)>,
) {
    if group.contains(&(x, y)) {
        return;
    }
    if lines[y as usize][x as usize] != letter {
        return;
    }

    group.insert((x, y));

    if x - 1 >= 0 {
        rec_region_p2(x - 1, y, letter, lines, group);
    }
    if x + 1 < lines[0].len() as i32 {
        rec_region_p2(x + 1, y, letter, lines, group);
    }
    if y - 1 >= 0 {
        rec_region_p2(x, y - 1, letter, lines, group);
    }
    if y + 1 < lines.len() as i32 {
        rec_region_p2(x, y + 1, letter, lines, group);
    }
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
    let lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut res = 0;
    let mut used: HashSet<(i32, i32)> = HashSet::new();

    let mut lines_map: HashMap<(i32, i32), char> = HashMap::new();

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            lines_map.insert((j, i), lines[i as usize][j as usize]);
        }
    }

    for (pos, val) in &lines_map {
        if !used.contains(pos) {
            let mut group: HashSet<(i32, i32)> = HashSet::new();

            rec_region_p2(pos.0, pos.1, *val, &lines, &mut group);

            let mut corners = 0;
            for pos in group.clone() {
                used.insert(pos);
                corners += get_corners(pos, &lines_map, *val);
            }
            res += group.len() as i32 * corners;
        }
    }

    println!("{}", res);
}
