use std::collections::HashMap;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day8"),
        day_id: 8,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(input: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch != '.' {
                if antennas.contains_key(&ch) {
                    antennas.get_mut(&ch).unwrap().push((x as i32, y as i32));
                } else {
                    antennas.insert(*ch, vec![(x as i32, y as i32)]);
                }
            }
        }
    }

    antennas
}

fn get_antinodes_pt1(positions: Vec<(i32, i32)>, field: &mut Vec<Vec<char>>) -> i32 {
    let height = field.len() as i32;
    let width = field[0].len() as i32;
    let mut res = 0;

    for (i, (x1, y1)) in positions.clone().into_iter().enumerate() {
        for j in i..positions.len() {
            let (x2, y2) = positions[j];
            if x1 == x2 && y1 == y2 {
                continue;
            }

            //for each two different antennas there should be two different antinodes
            let new_x1 = x1 + x1 - x2;
            let new_y1 = y1 + y1 - y2;

            if in_map(new_x1, new_y1, width, height) {
                if field[new_y1 as usize][new_x1 as usize] != '#' {
                    field[new_y1 as usize][new_x1 as usize] = '#';
                    res += 1;
                }
            }

            let new_x2 = x2 - x1 + x2;
            let new_y2 = y2 - y1 + y2;

            if in_map(new_x2, new_y2, width, height) {
                if field[new_y2 as usize][new_x2 as usize] != '#' {
                    field[new_y2 as usize][new_x2 as usize] = '#';
                    res += 1;
                }
            }
        }
    }
    res
}

fn get_antinodes_pt2(positions: Vec<(i32, i32)>, field: &mut Vec<Vec<char>>) -> i32 {
    let height = field.len() as i32;
    let width = field[0].len() as i32;
    let mut res = 0;

    for (i, (x1, y1)) in positions.clone().into_iter().enumerate() {
        for j in i..positions.len() {
            let (x2, y2) = positions[j];
            if x1 == x2 && y1 == y2 {
                continue;
            }
            let diff_x = x1 - x2;
            let diff_y = y1 - y2;
            let mut new_x = x1;
            let mut new_y = y1;

            while in_map(new_x, new_y, width, height) {
                if field[new_y as usize][new_x as usize] != '#' {
                    field[new_y as usize][new_x as usize] = '#';
                    res += 1;
                }
                new_x += diff_x;
                new_y += diff_y;
            }

            new_x = x1;
            new_y = y1;

            while in_map(new_x, new_y, width, height) {
                if field[new_y as usize][new_x as usize] != '#' {
                    field[new_y as usize][new_x as usize] = '#';
                    res += 1;
                }
                new_x -= diff_x;
                new_y -= diff_y;
            }
        }
    }
    res
}

fn in_map(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x >= width || y >= height {
        return false;
    }
    true
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut lines = input
        .lines
        .clone()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let antennas = parse(&lines);
    let mut res = 0;

    for (_, positions) in antennas {
        res += get_antinodes_pt1(positions, &mut lines);
    }

    println!("{}", res)
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut lines = input
        .lines
        .clone()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let antennas = parse(&lines);
    let mut res = 0;

    for (_, positions) in antennas {
        res += get_antinodes_pt2(positions, &mut lines);
    }

    println!("{}", res)
}
