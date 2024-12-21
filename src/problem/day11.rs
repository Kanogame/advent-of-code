use std::collections::HashMap;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day11"),
        day_id: 11,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn fast_len(val: i64) -> i64 {
    return val.ilog10() as i64 + 1;
}

fn fast_len_u(val: u64) -> u64 {
    return val.ilog10() as u64 + 1;
}

fn fast_split(val: i64) -> Vec<i64> {
    let pow = 10_i64.pow((fast_len(val) / 2) as u32);
    return vec![val / pow, val % pow];
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut stones: Vec<i64> = input.lines[0]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<i64> = Vec::new();

        for i in stones.clone().into_iter() {
            if i == 0 {
                new_stones.push(1);
            } else if fast_len(i) % 2 == 0 {
                new_stones.append(&mut fast_split(i));
            } else {
                new_stones.push(i * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{:?}", stones.len());
}

fn rec(value: u64, step: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if step == 0 {
        return 1;
    }
    if cache.contains_key(&(step, value)) {
        return *cache.get(&(step, value)).unwrap();
    }
    let res: u64;
    if value == 0 {
        res = rec(1, step - 1, cache);
    } else if fast_len_u(value) % 2 == 0 {
        let pow = 10_u64.pow((fast_len_u(value) / 2) as u32);
        res = rec(value / pow, step - 1, cache) + rec(value % pow, step - 1, cache);
    } else {
        res = rec(2024 * value, step - 1, cache);
    }
    cache.insert((step, value), res);
    res
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let stones: Vec<u64> = input.lines[0]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut res = 0;
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    for i in stones {
        res += rec(i, 75, &mut cache);
    }

    println!("{:?}", res);
}
