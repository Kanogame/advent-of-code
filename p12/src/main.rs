use core::num;
use std::{fs::read_to_string, result};
use regex::Regex;

fn main() {
    let mut records = parseInput(read_lines("data.txt"));
    RemoveCrap(records);
}

fn RemoveCrap(records: Vec<(String, Vec<i32>)>) {
    let mut res = 0;
    for (record, data) in records {
        res += count(record, data);
    }
    println!("{}", res);
}

fn count(cfg: String, nums: Vec<i32>) -> i32 {
    let mut result = 0;
    if nums.len() == 0 {
        if !cfg.contains("#") {
            return 1;
        } else {
            return 0;
        }
    }

    let current = nums[0];
    let mut Nnums: Vec<i32> = Vec::new();
    Nnums.resize(nums.len() - 1, 0);
    Nnums.copy_from_slice(&nums[1..]);
    let nums = Nnums;

    for i in 0..(cfg.len() - nums.iter().sum::<i32>() as usize - nums.len() - current as usize + 1) {
        if cfg[..i].contains("#") { break; }
        let nxt = i as i32 + current;
        if nxt <= cfg.len() as i32 && !cfg[i..nxt as usize].contains(".") && (cfg.chars().nth(nxt as usize) == None || cfg.chars().nth(nxt as usize).unwrap() != '#') {
            if (nxt as usize) < cfg.len() {
                result += count(cfg[(nxt as usize + 1)..].to_string(), nums.clone());
            } else {
                result += count("".to_string(), nums.clone());
            }
        }
    }
    return result;
}

fn parseInput(input: Vec<String>) -> Vec<(String, Vec<i32>)> {
    let mut records = Vec::<(String, Vec<i32>)>::new();
    let re = Regex::new(r"[?.#]+|[0-9]+").unwrap();
    for line in input {
        let data = re.find_iter(line.as_str()).map(|x| x.as_str().to_string()).collect::<Vec<String>>();
        let record = data[0].clone();
        let mut damaged = Vec::<i32>::new();
        for i in 1..data.len() {
            damaged.push(data[i].parse::<i32>().unwrap())
        }
        records.push((record, damaged));
    }
    records
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}  