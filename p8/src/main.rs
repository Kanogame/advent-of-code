use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;

fn main() {
    let (route, map) = parse_input(read_lines("data.txt"));
    find_route(route, map);
}

fn find_route_pt1(route: String, map: HashMap<String, (String, String)>) {
    let mut pos = 0;
    let mut res = 0; 
    let mut start = map.get("AAA").unwrap();
    loop {
        res += 1;
        let mut new = String::new();
        if route.chars().nth(pos).unwrap() == 'L' {
            new = start.0.clone();
            start = map.get(&start.0).unwrap();
        } else {
            new = start.1.clone();
            start = map.get(&start.1).unwrap();
        }
        pos += 1;
        if new == "ZZZ" {
            break;
        }
        if pos == route.len() {
            pos = 0;
        }
    }
    println!("{}", res);
}

fn parse_input(input: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let path = input[0].clone();

    let re = Regex::new(r"[A-Z]+").unwrap();
    let mut mapDesc: HashMap<String, (String, String) >= HashMap::new();
    for i in 2..input.len() {
        let data = re.find_iter(input[i].as_str()).map(|x| x.as_str().to_string()).collect::<Vec<_>>();
        let temp: (String, String, String) = (data[0].clone(), data[1].clone(), data[2].clone());
        mapDesc.insert(temp.0, (temp.1, temp.2));
    }

    (path, mapDesc)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}