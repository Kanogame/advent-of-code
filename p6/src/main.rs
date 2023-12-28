use std::{fs::read_to_string};
use regex::Regex;

fn main() {
    let arr = parseInput(read_lines("data.txt"));
    solution_pt1(arr);

}

fn solution_pt1(arr: Vec<(i32, i32)>) {
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

fn parseInput(input: Vec<String>) -> Vec<(i32, i32)> {
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
    let Time = re.find_iter(input[0].as_str()).filter_map(|digits| digits.as_str().parse::<i32>().ok()).collect::<Vec<_>>();
    let Distance = re.find_iter(input[1].as_str()).filter_map(|digits| digits.as_str().parse::<i32>().ok()).collect::<Vec<_>>();

    let mut Result: Vec<(i32, i32)> = Vec::new();
    
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