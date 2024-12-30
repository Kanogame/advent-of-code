use std::{cmp::Ordering, collections::HashMap};

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day5"),
        day_id: 5,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse_pt1(input: &Vec<String>) -> (Vec<(i32, i32)>, Vec<(HashMap<i32, i32>, i32)>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();

    // vec of (value -> position, mid)
    let mut updates: Vec<(HashMap<i32, i32>, i32)> = Vec::new();

    let mut rule_flag = true;
    for line in input {
        if line.len() == 0 {
            rule_flag = false;
            continue;
        }

        if rule_flag {
            let parts: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        } else {
            let parts: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            let mut val_pos: HashMap<i32, i32> = HashMap::new();

            for (id, val) in parts.iter().enumerate() {
                val_pos.insert(*val, id as i32);
            }

            updates.push((val_pos, parts[parts.len() / 2]));
        }
    }

    (rules, updates)
}

fn parse_pt2(
    input: &Vec<String>,
) -> (
    HashMap<i32, Vec<i32>>,
    Vec<HashMap<i32, i32>>,
    Vec<Vec<i32>>,
) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<HashMap<i32, i32>> = Vec::new();
    let mut raw_updates: Vec<Vec<i32>> = Vec::new();
    let mut val_pos: HashMap<i32, i32> = HashMap::new();
    let mut parts: Vec<i32>;

    let mut rule_flag = true;
    for line in input {
        if line.len() == 0 {
            rule_flag = false;
            continue;
        }

        if rule_flag {
            parts = line.split("|").map(|x| x.parse().unwrap()).collect();
            if rules.contains_key(&parts[0]) {
                rules.get_mut(&parts[0]).unwrap().push(parts[1]);
            } else {
                rules.insert(parts[0], vec![parts[1]]);
            }
        } else {
            parts = line.split(",").map(|x| x.parse().unwrap()).collect();

            for (id, val) in parts.iter().enumerate() {
                val_pos.insert(*val, id as i32);
            }

            updates.push(val_pos.clone());
            raw_updates.push(parts);
            val_pos.clear();
        }
    }

    (rules, updates, raw_updates)
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let (rules, updates) = parse_pt1(&input.lines);

    let mut res = 0;
    let mut flag;

    for (seq, mid_value) in updates {
        flag = true;
        for (before, after) in &rules {
            if !seq.contains_key(before) || !seq.contains_key(after) {
                continue;
            }

            if seq.get(after).unwrap() < seq.get(before).unwrap() {
                flag = false;
                break;
            }
        }

        if flag {
            res += mid_value;
        }
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let (rules, updates, mut sorted) = parse_pt2(&input.lines);

    let mut res = 0;
    let mut flag;

    for (i, seq) in updates.iter().enumerate() {
        flag = true;
        for (before, af_arr) in &rules {
            for after in af_arr {
                if !seq.contains_key(before) || !seq.contains_key(after) {
                    continue;
                }

                if seq.get(after).unwrap() < seq.get(before).unwrap() {
                    flag = false;
                    break;
                }
            }
        }

        if flag {
            continue;
        }

        sorted[i].sort_by(|a, b| {
            if rules.get(a).is_some_and(|value| value.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        res += sorted[i][sorted[i].len() / 2];
    }

    println!("{}", res);
}
