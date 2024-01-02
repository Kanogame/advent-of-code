use std::{fs::read_to_string};
use regex::Regex;

fn main() {
    let input = parse_input(read_lines("data.txt"));
    solution_pt1(input);
}

fn solution_pt1(lines: Vec<Vec<i32>>) {
    let res = lines.iter().map(|i| solve_line_pt2(i) as i64).collect::<Vec<i64>>().iter().sum::<i64>();
    print!("{:?}", res);
}

fn solve_line_pt2(dataset: &Vec<i32>) -> i32 {
    let mut data: Vec<Vec<i32>> = Vec::new();
    data.push(dataset.clone());

    let mut newarr: Vec<i32> = Vec::new();
    let mut prewarr = dataset.clone();
    while !check_vec(&prewarr) {
        for i in 0..prewarr.len() - 1 {
            newarr.push(prewarr[i + 1] - prewarr[i])
        }
        data.push(newarr.clone());
        prewarr = newarr;
        newarr = Vec::new();
    }

    let temp = data.last().unwrap()[0].clone();
    data.iter_mut().last().unwrap().push(temp);
    for i in 2..data.len() + 1 {
        let j = data.len() - i;
        let temp = data[j][0] - data[j + 1].last().unwrap();
        data[j].push(temp);
    }
    *data[0].last().unwrap()
}

fn solve_line_pt1(dataset: &Vec<i32>) -> i32 {
    let mut res = 0;

    let mut newarr: Vec<i32> = Vec::new();
    let mut prewarr = dataset.clone();
    while !check_vec(&prewarr) {
        for i in 0..prewarr.len() - 1 {
            newarr.push(prewarr[i + 1] - prewarr[i])
        }   
        res += *newarr.last().unwrap();
        prewarr = newarr;
        newarr = Vec::new();
    }

    res + dataset.iter().last().unwrap()
}

fn check_vec(inp: &Vec<i32>) -> bool {
    let first = inp[0];
    inp.into_iter().filter(|i| **i != first).collect::<Vec<_>>().len() == 0
}

fn parse_input(input: Vec<String>) -> Vec<Vec<i32>> {
    let re: Regex = Regex::new(r"[0-9-]+").unwrap();
    let mut res: Vec<Vec<i32>> = Vec::new();
    for line in input {
        res.push(re.find_iter(line.as_str()).filter_map(|digits| digits.as_str().parse::<i32>().ok()).collect::<Vec<_>>());
    }
    res
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}