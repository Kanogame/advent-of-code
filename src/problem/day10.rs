use std::collections::HashSet;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day10"),
        day_id: 10,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn rec_pt1(x: i32, y: i32, prev: i32, map: &Vec<Vec<i32>>, res: &mut HashSet<(i32, i32)>) {
    if prev + 1 != map[y as usize][x as usize] {
        return;
    }
    if map[y as usize][x as usize] == 9 {
        res.insert((x, y));
        return;
    }
    if x - 1 >= 0 {
        rec_pt1(x - 1, y, map[y as usize][(x) as usize], map, res);
    }
    if x + 1 < map[0].len() as i32 {
        rec_pt1(x + 1, y, map[y as usize][(x) as usize], map, res);
    }
    if y - 1 >= 0 {
        rec_pt1(x, y - 1, map[(y) as usize][x as usize], map, res);
    }
    if y + 1 < map.len() as i32 {
        rec_pt1(x, y + 1, map[(y) as usize][x as usize], map, res);
    }
}

fn rec_pt2(x: i32, y: i32, prev: i32, map: &Vec<Vec<i32>>) -> i32 {
    if prev + 1 != map[y as usize][x as usize] {
        return 0;
    }
    if map[y as usize][x as usize] == 9 {
        return 1;
    }
    let mut res: i32 = 0;
    if x - 1 >= 0 {
        res += rec_pt2(x - 1, y, map[y as usize][(x) as usize], map);
    }
    if x + 1 < map[0].len() as i32 {
        res += rec_pt2(x + 1, y, map[y as usize][(x) as usize], map);
    }
    if y - 1 >= 0 {
        res += rec_pt2(x, y - 1, map[(y) as usize][x as usize], map);
    }
    if y + 1 < map.len() as i32 {
        res += rec_pt2(x, y + 1, map[(y) as usize][x as usize], map);
    }
    return res;
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let lines = input
        .lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = 0;

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 0 {
                rec_pt1(x as i32, y as i32, -1, &lines, &mut set);
                res += set.len();
                set.clear();
            }
        }
    }
    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let lines = input
        .lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 0 {
                res += rec_pt2(x as i32, y as i32, -1, &lines);
            }
        }
    }
    println!("{}", res);
}
