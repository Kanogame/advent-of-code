use regex::Regex;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day3"),
        day_id: 3,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let re: Regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

    let mut res: i32 = 0;
    for line in input.lines {
        for (_, [a, b]) in re.captures_iter(&line).map(|x| x.extract()) {
            res += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let re: Regex = Regex::new(r"mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\)").unwrap();
    let numbers: Regex = Regex::new(r"[0-9]+").unwrap();

    let mut res: i32 = 0;
    let mut state: bool = true;
    for line in input.lines {
        for command in re.find_iter(&line) {
            match command.as_str() {
                "do()" => {
                    state = true;
                }
                "don't()" => {
                    state = false;
                }
                _ => {
                    if state {
                        //fancy rust shenanigans
                        res += numbers
                            .find_iter(command.as_str())
                            .map(|x| x.as_str().parse::<i32>().unwrap())
                            .fold(1, |state, x| state * x)
                    }
                }
            }
        }
    }

    println!("{}", res);
}
