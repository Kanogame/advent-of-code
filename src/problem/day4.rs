use core::ops::Range;
use regex::Regex;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day4"),
        day_id: 4,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn word_search(pos_x: usize, pos_y: usize, lines: &Vec<String>) -> i32 {
    let mut res: i32 = 0;

    for y in avaliable_range(pos_y as i32, 1, lines.len()) {
        for x in avaliable_range(pos_x as i32, 1, lines[y].len()) {
            if lines[y].chars().nth(x).unwrap() == 'M' {
                let dir = get_direction(pos_x, pos_y, x, y);

                if follow_direction(dir, pos_x, pos_y, lines) {
                    res += 1;
                }
            }
        }
    }
    return res;
}

fn follow_direction(dir: Direction, x: usize, y: usize, lines: &Vec<String>) -> bool {
    let mut cur_x = x;
    let mut cur_y = y;

    for letter in "MAS".chars() {
        match dir {
            Direction::Up => {
                if cur_y == 0 {
                    return false;
                }
                cur_y -= 1;
            }
            Direction::Down => {
                if cur_y + 1 >= lines.len() {
                    return false;
                }
                cur_y += 1;
            }
            Direction::Left => {
                if cur_x == 0 {
                    return false;
                }
                cur_x -= 1;
            }
            Direction::Right => {
                if cur_x + 1 >= lines[cur_y].len() {
                    return false;
                }
                cur_x += 1;
            }
            Direction::UpLeft => {
                if cur_y == 0 || cur_x == 0 {
                    return false;
                }
                cur_y -= 1;
                cur_x -= 1;
            }
            Direction::UpRight => {
                if cur_y == 0 || cur_x + 1 >= lines[cur_y].len() {
                    return false;
                }
                cur_y -= 1;
                cur_x += 1;
            }
            Direction::DownLeft => {
                if cur_y + 1 >= lines.len() || cur_x == 0 {
                    return false;
                }
                cur_y += 1;
                cur_x -= 1;
            }
            Direction::DownRight => {
                if cur_y + 1 >= lines.len() || cur_x + 1 >= lines[cur_y].len() {
                    return false;
                }
                cur_y += 1;
                cur_x += 1;
            }
        }

        if lines[cur_y].chars().nth(cur_x).unwrap() != letter {
            return false;
        }
    }
    return true;
}

fn avaliable_range(x: i32, rad: i32, array_length: usize) -> Range<usize> {
    let mut start: i32 = x - rad;
    let mut end: i32 = x + rad + 1;

    if end > array_length as i32 {
        end = array_length as i32;
    }
    if start < 0 {
        start = 0;
    }

    return Range {
        start: start as usize,
        end: end as usize,
    };
}

fn get_direction(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Direction {
    if start_y < end_y {
        if start_x > end_x {
            return Direction::DownLeft;
        } else if start_x == end_x {
            return Direction::Down;
        }
        return Direction::DownRight;
    } else if start_y == end_y {
        if start_x > end_x {
            return Direction::Left;
        }
        return Direction::Right;
    }

    if start_x > end_x {
        return Direction::UpLeft;
    } else if start_x == end_x {
        return Direction::Up;
    }
    return Direction::UpRight;
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut res: i32 = 0;

    for (y, line) in input.lines.iter().enumerate() {
        for (x, ch) in line.chars().into_iter().enumerate() {
            if ch == 'X' {
                res += word_search(x, y, &input.lines);
            }
        }
    }

    println!("{}", res)
}

fn x_max(pos_x: usize, pos_y: usize, lines: &Vec<String>) -> bool {
    const xmas: [[[char; 3]; 3]; 4] = [
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
    ];

    let mut flag;
    for patern in xmas {
        flag = true;
        for y in 0..3 {
            for x in 0..3 {
                let ch = patern[y][x];
                if ch == '.' {
                    continue;
                }
                if lines[y + pos_y].chars().nth(x + pos_x).unwrap() != patern[y][x] {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }

        if flag {
            return true;
        }
    }
    return false;
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut res = 0;

    for y in 0..input.lines.len() - 2 {
        for x in 0..input.lines[0].chars().collect::<Vec<char>>().len() - 2 {
            if x_max(x, y, &input.lines) {
                res += 1;
            }
        }
    }

    println!("{}", res)
}
