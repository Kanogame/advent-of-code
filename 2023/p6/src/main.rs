use std::{fs::read_to_string, fmt::Result};
use regex::Regex;

fn main() {
    let arr = parseInput_pt2(read_lines("data.txt"));
    solution(arr);

}

fn solution(arr: Vec<(i64, i64)>) {
    let mut res = 1;
    for data in arr {
        let mut cur = 0;
        for wait in 1..data.0 {
            let distance = (data.0 - wait) * wait;
            if distance > data.1 {
                cur += 1;
            }
        }
        res *= cur;
    }

    println!("{}", res);
}

fn parseInput_pt2(input: Vec<String>) -> Vec<(i64, i64)> {
    let re: Regex = Regex::new(r"\D+").unwrap();
    let Time = re.replace_all(input[0].as_str(), "").parse::<i64>().unwrap();
    let Distance = re.replace_all(input[1].as_str(), "").parse::<i64>().unwrap();
    return vec![(Time, Distance)];
}

fn parseInput_pt1(input: Vec<String>) -> Vec<(i64, i64)> {
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
    let Time = re.find_iter(input[0].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
    let Distance = re.find_iter(input[1].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();

    let mut Result: Vec<(i64, i64)> = Vec::new();
    
    for i in 0..Time.len() {
        Result.push((Time[i], Distance[i]))
    }

    Result
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}