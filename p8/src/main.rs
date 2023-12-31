use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;
use gcd::Gcd;

fn main() {
    let (route, map) = parse_input(read_lines("data.txt"));
    find_route(route, map);
}

fn find_route(route: String, map: HashMap<String, (String, String)>) {
    let mut starts: Vec<(String, String)> = Vec::new();
    for i in map.keys() {
        if i.ends_with("A") {
            starts.push(map.get(i).unwrap().clone());
        }
    }
    let mut ress = Vec::<i64>::new();
    for start in starts.iter_mut() {
        let mut pos = 0;
        let mut res = 0; 
        let mut start = start.clone();
        loop {
            res += 1;
            let mut new = String::new();
            if route.chars().nth(pos).unwrap() == 'L' {
                new = start.0.clone();
                start = map.get(&start.0).unwrap().clone();
            } else {
                new = start.1.clone();
                start = map.get(&start.1).unwrap().clone();
            }
            pos += 1;
            if new.ends_with("Z") {
                break;
            }
            if pos == route.len() {
                pos = 0;
            }
        }
        ress.push(res as i64);
    }

    ress.sort();
    
    let mut lcm = ress.into_iter().reduce(|a, b| (a * (b as u64 / (a as u64).gcd(b as u64)) as i64)).unwrap();
    println!("{}", lcm);
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

    let re = Regex::new(r"[A-Z0-9]+").unwrap();
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