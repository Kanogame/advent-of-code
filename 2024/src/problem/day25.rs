use std::iter::repeat;

use itertools::Itertools;

use crate::generic_problem::{self, Day};

const WIDHT: usize = 5;
const HEIGHT: usize = 7;

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day25"),
        day_id: 25,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(lines: Vec<String>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let shemas = lines
        .split(|x| *x == "".to_string())
        .map(|x| x.to_vec().iter().map(|y| y.chars().collect()).collect())
        .collect::<Vec<Vec<Vec<char>>>>();

    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();

    for shematic in shemas {
        let lock = shematic[0].iter().join("") == repeat("#").take(WIDHT).join("");
        let mut vec = vec![];
        for j in 0..WIDHT {
            for i in 1..HEIGHT {
                if lock && shematic[i][j] == '.' {
                    vec.push(i);
                    break;
                }
                if !lock && shematic[i][j] == '#' {
                    vec.push(HEIGHT - i);
                    break;
                }
            }
        }
        if lock {
            locks.push(vec);
        } else {
            keys.push(vec);
        }
    }

    (keys, locks)
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (keys, lk) = parse(input.lines);
    let mut res = 0;

    for key in keys {
        'lk: for lock in lk.iter() {
            for i in 0..WIDHT {
                if key[i] + lock[i] > HEIGHT {
                    continue 'lk;
                }
            }
            res += 1;
        }
    }

    println!("{}", res);
}

pub fn part_two(_: generic_problem::ProblemInput) {
    println!("Merry christmas!");
    // After many hours of hard work (sometimes even comprehending and re-reading texts), I finally finished AOC!
    // All 25 day were solved, thought not all of them will work for any input.
    // I really enjoed solving every problem, even if some required very specific algorithms
    // In next year (this year) I will try my best at solving every puzzle.
    // For future reference: 31.61s on Ryzen system
}
