use std::{
    char,
    collections::{BTreeMap, LinkedList},
};

use crate::generic_problem::{self, Day};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day15"),
        day_id: 15,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse_input(lines: Vec<String>) -> (BTreeMap<(i32, i32), char>, (i32, i32), Vec<usize>) {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut lines_map: BTreeMap<(i32, i32), char> = BTreeMap::new();
    let mut rob = (0, 0);

    let mut i = 0;
    while lines[i].len() > 0 {
        for j in 0..lines[0].len() - 0 {
            lines_map.insert((i as i32, j as i32), lines[i][j]);
            if lines[i][j] == '@' {
                rob = (i as i32, j as i32)
            }
        }
        i += 1;
    }

    let mut alg: Vec<usize> = Vec::new();

    for line in i + 1..lines.len() {
        for ch in lines[line].clone() {
            match ch {
                '^' => alg.push(3),
                'v' => alg.push(1),
                '<' => alg.push(2),
                '>' => alg.push(0),
                _ => {}
            }
        }
    }

    (lines_map, rob, alg)
}

fn parse_input_two(lines: Vec<String>) -> (BTreeMap<(i32, i32), char>, (i32, i32), Vec<usize>) {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut lines_map: BTreeMap<(i32, i32), char> = BTreeMap::new();
    let mut rob = (0, 0);

    let mut i = 0;
    while lines[i].len() > 0 {
        for j in 0..lines[0].len() - 0 {
            if lines[i][j] == '@' {
                lines_map.insert((i as i32, j as i32 * 2), lines[i][j]);
                lines_map.insert((i as i32, j as i32 * 2 + 1), '.');
                rob = (i as i32, j as i32 * 2);
            } else if lines[i][j] == 'O' {
                lines_map.insert((i as i32, j as i32 * 2), '[');
                lines_map.insert((i as i32, j as i32 * 2 + 1), ']');
            } else {
                lines_map.insert((i as i32, j as i32 * 2), lines[i][j]);
                lines_map.insert((i as i32, j as i32 * 2 + 1), lines[i][j]);
            }
        }
        i += 1;
    }

    let mut alg: Vec<usize> = Vec::new();

    for line in i + 1..lines.len() {
        for ch in lines[line].clone() {
            match ch {
                '^' => alg.push(3),
                'v' => alg.push(1),
                '<' => alg.push(2),
                '>' => alg.push(0),
                _ => {}
            }
        }
    }

    (lines_map, rob, alg)
}

//ret = success of moving
fn move_box(pos: (i32, i32), direction: usize, map: &mut BTreeMap<(i32, i32), char>) -> bool {
    let next_pos = add(pos, DIRECTIONS[direction]);
    if map.get(&next_pos).is_some_and(|x| *x != '#') {
        let next_value = *map.get(&next_pos).unwrap();
        if next_value == '.' {
            //move
            *map.get_mut(&next_pos).unwrap() = *map.get(&pos).unwrap();
            return true;
        } else if next_value == '[' || next_value == ']' || next_value == 'O' {
            if move_box(next_pos, direction, map) {
                //move
                *map.get_mut(&next_pos).unwrap() = *map.get(&pos).unwrap();
                return true;
            }
        }
    }
    return false;
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn part_res(map: &BTreeMap<(i32, i32), char>, reference: char) -> i32 {
    let mut res = 0;
    for ((a, b), v) in map {
        if *v == reference {
            res += *b + (*a * 100);
        }
    }
    res
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (mut lines_map, mut rob, alg) = parse_input(input.lines);

    for i in alg {
        let next_pos = add(rob, DIRECTIONS[i]);
        if lines_map.get(&next_pos).is_some_and(|x| *x != '#') {
            let value = *lines_map.get(&next_pos).unwrap();
            if value == 'O' {
                if move_box(next_pos, i, &mut lines_map) {
                    *lines_map.get_mut(&next_pos).unwrap() = '@';
                    *lines_map.get_mut(&rob).unwrap() = '.';
                    rob = next_pos;
                }
            } else if value == '.' {
                *lines_map.get_mut(&next_pos).unwrap() = '@';
                *lines_map.get_mut(&rob).unwrap() = '.';
                rob = next_pos;
            }
        }
    }

    println!("{}", part_res(&lines_map, 'O'));
}

// pos - pos of any part of box
// direction - direction of movement
// map - map
// will always move the box by direction (no matter what), cleaning any remaining box debris
fn simple_clean_move(pos: (i32, i32), direction: (i32, i32), map: &mut BTreeMap<(i32, i32), char>) {
    let b_2 = match *map.get(&pos).unwrap() {
        ']' => DIRECTIONS[2],
        '[' => DIRECTIONS[0],
        _ => return,
    };
    let next_pos = add(pos, direction);

    *map.get_mut(&next_pos).unwrap() = *map.get(&pos).unwrap();
    *map.get_mut(&add(next_pos, b_2)).unwrap() = *map.get(&add(pos, b_2)).unwrap();

    // cleaning
    *map.get_mut(&pos).unwrap() = '.';
    *map.get_mut(&add(pos, b_2)).unwrap() = '.';
}

//ret = success of moving
fn move_wide_box(
    pos: (i32, i32),
    direction: usize,
    map: &mut BTreeMap<(i32, i32), char>,
    tx: &mut LinkedList<(i32, i32)>,
) -> bool {
    let pos_value = *map.get(&pos).unwrap();
    if pos_value == '.' {
        return true;
    }
    if pos_value == '#' {
        return false;
    }
    let next_pos = add(pos, DIRECTIONS[direction]);
    if map.get(&next_pos).is_none() {
        return false;
    }
    if direction == 0 || direction == 2 {
        if move_box(next_pos, direction, map) {
            *map.get_mut(&next_pos).unwrap() = pos_value;
            return true;
        }
        return false;
    }
    if move_wide_box(next_pos, direction, map, tx)
        && move_wide_box(
            add(
                next_pos,
                match pos_value {
                    ']' => DIRECTIONS[2],
                    '[' => DIRECTIONS[0],
                    _ => return false,
                },
            ),
            direction,
            map,
            tx,
        )
    {
        for i in tx.iter() {
            if *i == pos {
                return true;
            }
        }
        tx.push_back(pos);
        return true;
    }
    return false;
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (mut lines_map, mut rob, alg) = parse_input_two(input.lines);

    for i in alg {
        let next_pos = add(rob, DIRECTIONS[i]);
        if lines_map.get(&next_pos).is_some() {
            let mut tx: LinkedList<(i32, i32)> = LinkedList::new();
            if move_wide_box(next_pos, i, &mut lines_map, &mut tx) {
                while tx.len() > 0 {
                    let v = tx.pop_front().unwrap();
                    simple_clean_move(v, DIRECTIONS[i], &mut lines_map);
                }
                *lines_map.get_mut(&next_pos).unwrap() = '@';
                *lines_map.get_mut(&rob).unwrap() = '.';
                rob = next_pos;
            }
        }
    }

    println!("{}", part_res(&lines_map, '['));
}
