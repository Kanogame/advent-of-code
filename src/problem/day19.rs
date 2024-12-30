use std::collections::HashMap;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day19"),
        day_id: 19,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let towels: Vec<String> = lines[0].split(", ").map(|x| x.to_string()).collect();
    let seq: Vec<String> = lines[2..].iter().map(|x| x.clone()).collect();
    return (towels, seq);
}

fn rec(pattern: String, towels: &Vec<String>, cache: &mut HashMap<String, bool>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if cache.get(&pattern).is_some() {
        return *cache.get(&pattern).unwrap();
    }

    for tw in towels {
        if pattern.len() < tw.len() {
            continue;
        }
        if pattern[..tw.len()].to_string() == *tw {
            if rec(pattern[tw.len()..].to_string(), towels, cache) {
                cache.insert(pattern, true);
                return true;
            }
        }
    }

    cache.insert(pattern, false);
    false
}

fn rec_pt2(pattern: String, towels: &Vec<String>, cache: &mut HashMap<String, i64>) -> i64 {
    //println!("{}", pattern);
    if pattern.len() == 0 {
        return 1;
    }

    if cache.get(&pattern).is_some() {
        return *cache.get(&pattern).unwrap();
    }

    let mut possible = 0;
    for tw in towels {
        if pattern.len() < tw.len() {
            continue;
        }

        if pattern[..tw.len()].to_string() == *tw {
            let v = rec_pt2(pattern[tw.len()..].to_string(), towels, cache);
            if v > 0 {
                cache.insert(pattern.clone(), v);
                possible += v;
            }
        }
    }

    cache.insert(pattern, possible);
    possible
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (towels, seq) = parse(input.lines);

    let mut cache = HashMap::new();
    let mut res = 0;

    for i in seq {
        if rec(i.clone(), &towels, &mut cache) {
            res += 1;
        }
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (towels, seq) = parse(input.lines);

    let mut cache = HashMap::new();
    let mut res = 0;

    for i in seq {
        // i64? fr?
        let v = rec_pt2(i.clone(), &towels, &mut cache);
        if v > 0 {
            res += v;
        }
    }

    println!("{}", res);
}
