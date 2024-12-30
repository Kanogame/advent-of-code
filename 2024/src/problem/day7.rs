use std::collections::HashMap;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day7"),
        day_id: 7,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(input: generic_problem::ProblemInput) -> Vec<(i64, Vec<i64>)> {
    let mut res: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in input.lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts[0].parse::<i64>().unwrap();
        res.push((
            target,
            parts[1].split(" ").map(|x| x.parse().unwrap()).collect(),
        ));
    }

    res
}

fn iterate_solutions_pt1(target: i64, nums: Vec<i64>) -> bool {
    let mut res: i64;

    let combinations: Vec<Vec<i32>> = combinations(2, nums.len() - 1);

    for comb in combinations {
        res = nums[0];
        for (id, val) in comb.iter().enumerate() {
            if *val == 1 {
                res += nums[id + 1]
            } else {
                res *= nums[id + 1]
            }
        }
        if res == target {
            return true;
        }
    }
    return false;
}

fn iterate_solutions_pt2(target: i64, nums: &Vec<i64>, combinations: &Vec<Vec<i32>>) -> bool {
    let mut res: i64;

    for comb in combinations {
        res = nums[0];
        for (id, val) in comb.iter().enumerate() {
            if *val == 0 {
                res += nums[id + 1]
            } else if *val == 1 {
                res *= nums[id + 1]
            } else {
                //yay first real usage of logarithms in my life
                res = res * 10_i64.pow(nums[id + 1].ilog10() as u32 + 1) + nums[id + 1]
            }
        }
        if res == target {
            return true;
        }
    }
    return false;
}

fn combinations(num: i32, len: usize) -> Vec<Vec<i32>> {
    //simple n-ary counter, implementing this was 10x faster that finding a decent rust lib
    let total_combinations = num.pow(len as u32);

    (0..total_combinations)
        .map(|i| {
            (0..len)
                .rev()
                .map(|j| i / num.pow(j as u32) % num)
                .collect()
        })
        .collect()
}

pub fn part_one(input: generic_problem::ProblemInput) {
    //brute force for both parts again, but a clever use of log! (18s -> 4s (+cache))
    let values = parse(input);
    let mut res = 0;

    for (target, nums) in values {
        if iterate_solutions_pt1(target, nums) {
            res += target;
        }
    }
    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let values = parse(input);
    let mut cache: HashMap<usize, Vec<Vec<i32>>> = HashMap::new();
    let mut res = 0;

    for (target, nums) in values {
        if cache.contains_key(&(nums.len())) {
            if iterate_solutions_pt2(target, &nums, &cache.get(&(nums.len())).unwrap()) {
                res += target;
            }
        } else {
            let value = combinations(3, nums.len() - 1);
            if iterate_solutions_pt2(target, &nums, &value) {
                res += target;
            }
            cache.insert(nums.len(), value);
        }
    }
    println!("{}", res);
}
