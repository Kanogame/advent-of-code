use std::collections::HashMap;

use regex::Regex;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day14"),
        day_id: 14,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}
const ROOM_HEIGHT: i32 = 103;
const ROOM_WIDTH: i32 = 101;

const Q_HEIGHT: i32 = ROOM_HEIGHT / 2;
const Q_WIDTH: i32 = ROOM_WIDTH / 2;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn state(&mut self, x: i32) {
        let mut new_x = (self.velocity.0 * x + self.pos.0) % ROOM_WIDTH;
        let mut new_y = (self.velocity.1 * x + self.pos.1) % ROOM_HEIGHT;
        if new_x < 0 {
            new_x = (new_x + ROOM_WIDTH).abs()
        }
        if new_y < 0 {
            new_y = (new_y + ROOM_HEIGHT).abs()
        }
        self.pos = (new_x, new_y);
    }
}

fn parse(lines: Vec<String>) -> Vec<Robot> {
    let re: Regex = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let mut res: Vec<Robot> = Vec::new();
    for line in lines {
        for [p_x, p_y, v_x, v_y] in re
            .captures_iter(&line)
            .map(|x| x.extract().1.map(|el| el.parse::<i32>().unwrap_or(-1)))
        {
            res.push(Robot {
                pos: (p_x, p_y),
                velocity: (v_x, v_y),
            });
        }
    }
    res
}

fn get_q(x: i32, y: i32) -> i32 {
    if x < Q_WIDTH {
        if y < Q_HEIGHT {
            return 1;
        } else if y > Q_HEIGHT {
            return 3;
        }
        return 0;
    } else if x > Q_WIDTH {
        if y < Q_HEIGHT {
            return 2;
        } else if y > Q_HEIGHT {
            return 4;
        }
        return 0;
    }
    return 0;
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut robots = parse(input.lines);

    let mut res = vec![0, 0, 0, 0, 0];

    for i in robots.iter_mut() {
        i.state(100);
        res[get_q(i.pos.0, i.pos.1) as usize] += 1;
    }

    let res = res[1..].iter().fold(1, |a, x| a * x);
    println!("{:?}", res);
}

fn is_tree(robots: &Vec<Robot>) -> bool {
    let mut lines: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in robots {
        if lines.contains_key(&i.pos.1) {
            lines.get_mut(&i.pos.1).unwrap().push(i.pos.0);
        } else {
            lines.insert(i.pos.1, vec![i.pos.0]);
        }
    }
    let mut res = false;

    'out: for (_, vec) in lines.iter_mut() {
        if vec.len() < 30 {
            continue;
        }
        vec.sort();
        let mut ones = 0;
        for i in 0..vec.len() - 1 {
            if (vec[i] - vec[i + 1]).abs() == 1 {
                ones += 1;
                if ones == 30 {
                    res = true;
                    break 'out;
                }
            }
        }
    }
    res
}

fn print_robots(robots: &Vec<Robot>) {
    let mut lines: Vec<Vec<char>> = vec![vec!['.'; ROOM_WIDTH as usize]; ROOM_HEIGHT as usize];
    for i in robots {
        lines[i.pos.1 as usize][i.pos.0 as usize] = '*';
    }

    for i in lines {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut robots = parse(input.lines);

    let mut res = 0;

    loop {
        res += 1;
        for i in robots.iter_mut() {
            i.state(1);
        }
        if is_tree(&robots) {
            // bruh. What a day: no examples, nothing, just 'a Christmas tree'
            print_robots(&robots);
            break;
        }
    }

    println!("{:?}", res);
}
