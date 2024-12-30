use std::collections::HashMap;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day1"),
        day_id: 1,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(input: generic_problem::ProblemInput) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines {
        let parts: Vec<i32> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        left.push(parts[0]);
        right.push(parts[1]);
    }

    return (left, right);
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let mut diff: i32 = 0;
    for i in 0..left.len() {
        diff += (left[i] - right[i]).abs();
    }

    println!("{}", diff);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (left, right) = parse(input);

    let mut right_map: HashMap<i32, i32> = HashMap::new();

    let mut res: i32 = 0;
    for i in right {
        let val = right_map.get_mut(&i);
        if val == None {
            right_map.insert(i, 1);
        } else {
            *val.unwrap() += 1;
        }
    }

    for i in left {
        let val = right_map.get(&i);
        if val != None {
            res += i * right_map.get(&i).unwrap();
        }
    }

    println!("{}", res);
}
