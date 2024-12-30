use std::collections::{HashMap, HashSet};

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day6"),
        day_id: 6,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&mut self) {
        let new_value = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };

        *self = new_value
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize, Direction) {
    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                '^' => {
                    return (x, y, Direction::Up);
                }
                '>' => {
                    return (x, y, Direction::Right);
                }
                '<' => {
                    return (x, y, Direction::Left);
                }
                'v' => {
                    return (x, y, Direction::Down);
                }
                _ => {}
            }
        }
    }

    return (0, 0, Direction::Up);
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let height = input.lines.len();
    let width = input.lines[0].len();
    let mut res = 0;

    let lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (mut x, mut y, mut dir) = find_guard(&lines);
    let mut been: HashSet<(usize, usize)> = HashSet::new();
    let mut next_x: usize = x;
    let mut next_y: usize = y;

    loop {
        match dir {
            Direction::Up => {
                if next_y == 0 {
                    break;
                }
                next_y = y - 1;
                next_x = x;
            }
            Direction::Left => {
                if next_x == 0 {
                    break;
                }
                next_x = x - 1;
                next_y = y;
            }
            Direction::Right => {
                next_x = x + 1;
                if next_x >= width {
                    break;
                }
                next_y = y;
            }
            Direction::Down => {
                next_y = y + 1;
                if next_y >= height {
                    break;
                }
                next_x = x;
            }
        }
        if next_x >= height || next_y >= width {
            break;
        }

        if lines[next_y][next_x] == '#' {
            dir.turn_right();
            continue;
        }
        x = next_x;
        y = next_y;

        if !been.contains(&(next_x, next_y)) {
            been.insert((next_x, next_y));
            res += 1;
        }
    }

    println!("{}", res)
}

pub fn part_two(input: generic_problem::ProblemInput) {
    //sadly brute force =(
    let height = input.lines.len();
    let width = input.lines[0].len();
    let mut res = 0;
    let mut set: HashMap<(usize, usize), i32> = HashMap::new();

    let lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (base_x, base_y, base_dir) = find_guard(&lines);
    let mut next_x;
    let mut next_y;
    let mut x;
    let mut y;
    let mut dir;

    for new_y in 0..height {
        for new_x in 0..width {
            x = base_x;
            y = base_y;
            next_x = x;
            next_y = y;
            dir = base_dir.clone();
            loop {
                match dir {
                    Direction::Up => {
                        if next_y == 0 {
                            break;
                        }
                        next_y = y - 1;
                        next_x = x;
                    }
                    Direction::Left => {
                        if next_x == 0 {
                            break;
                        }
                        next_x = x - 1;
                        next_y = y;
                    }
                    Direction::Right => {
                        next_x = x + 1;
                        if next_x >= width {
                            break;
                        }
                        next_y = y;
                    }
                    Direction::Down => {
                        next_y = y + 1;
                        if next_y >= height {
                            break;
                        }
                        next_x = x;
                    }
                }

                if lines[next_y][next_x] == '#' || (next_x == new_x && next_y == new_y) {
                    if set.contains_key(&(next_x, next_y)) {
                        *(set.get_mut(&(next_x, next_y)).unwrap()) += 1;
                        if *set.get(&(next_x, next_y)).unwrap() == 3 {
                            res += 1;
                            break;
                        }
                    } else {
                        set.insert((next_x, next_y), 1);
                    }
                    dir.turn_right();
                    continue;
                }
                x = next_x;
                y = next_y;
            }
            set.clear();
        }
    }

    println!("{}", res)
}
