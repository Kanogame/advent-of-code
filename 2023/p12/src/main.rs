use std::fs::read_to_string;
use cached::proc_macro::cached;
use regex::Regex;

fn main() {
    remove_crap(parse_input_pt2(read_lines("data.txt")));
}

fn remove_crap(records: Vec<(String, Vec<i32>)>) {
    let mut res = 0;
    for (record, data) in records {
        res += count(record, data) as i64;
    }
    println!("{}", res);
}

#[cached]
fn count(cfg: String, nums: Vec<i32>) -> i64 {
    let mut result = 0;
    if nums.len() == 0 {
        return !cfg.contains("#") as i64;
    }
    
    let current = nums[0];
    let mut nums = nums;
    nums.remove(nums.iter().position(|x| *x == nums[0]).unwrap());

    for i in 0..(cfg.len() - nums.iter().sum::<i32>() as usize - nums.len() - current as usize + 1) {
        if cfg[..i].contains("#") { break; }
        let nxt = i as i32 + current;
        if nxt <= cfg.len() as i32 && !cfg[i..nxt as usize].contains(".") && (cfg.chars().nth(nxt as usize) == None || cfg.chars().nth(nxt as usize).unwrap() != '#') {
            result += count(if ((nxt as usize) < cfg.len()) {cfg[(nxt as usize + 1)..].to_string()} else {"".to_string()}, nums.clone());
        }
    }
    return result;
}

fn parse_input_pt2(input: Vec<String>) -> Vec<(String, Vec<i32>)> {
    let mut records = Vec::<(String, Vec<i32>)>::new();
    let re = Regex::new(r"[?.#]+|[0-9]+").unwrap();
    for line in input {
        let data = re.find_iter(line.as_str()).map(|x| x.as_str().to_string()).collect::<Vec<String>>();
        let record = data[0].clone();
        let mut damaged = Vec::<i32>::new();
        for i in 1..data.len() {
            damaged.push(data[i].parse::<i32>().unwrap())
        }

        let mut five_record = record.clone();
        for _ in 1..5 {
            five_record += &("?".to_string() + &record.clone());
        } 
        let mut five_damaged = Vec::<i32>::new();
        for i in 0..damaged.len() * 5 {
            five_damaged.push(damaged[i % damaged.len()]);
        }
        records.push((five_record, five_damaged));
    }
    records
}

fn parse_input_pt1(input: Vec<String>) -> Vec<(String, Vec<i32>)> {
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