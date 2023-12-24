use std::{fs::read_to_string};
use regex::Regex;

use indicatif::ProgressBar;

fn main() {
    let input = read_lines("data.txt");
    let (mut seeds, raw) = parse_input_pt2(input);
    process_raw(&mut seeds, &raw);
    println!("{}", seeds.iter().min().unwrap())
}

fn process_raw(seeds: &mut Vec<i64>,raw: &Vec<Vec<Vec<i64>>>) {
    //let pb = ProgressBar::new(seeds.len() as u64);
    for (j, seed) in seeds.iter_mut().enumerate() {
        //pb.inc(1); - adds 2 minutes
        'typeloop: for maptype in 0..raw.len() {
            for i in raw[maptype].iter() {
                if *seed <= i[1] + i[2] - 1  && *seed >= i[1] {
                    *seed = *seed + (i[0] - i[1]);
                    continue 'typeloop;
                } 
            }
        }
    }
} 

fn parse_input_pt1(input: Vec<String>) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
    let seeds = re.find_iter(input[0].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
    let mut raw: Vec<Vec<Vec<i64>>> = Vec::new();

    let mut maptype = 0;
    raw.push(Vec::new());
    for i in 3..input.len() {
        if input[i] == "" { continue; }
        let res = re.find_iter(input[i].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
        if res.len() > 0 {
            raw[maptype].push(res);
        } else {
            maptype += 1;
            raw.push(Vec::new());
        }
    }

    (seeds, raw)
}

fn parse_input_pt2(input: Vec<String>) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
    let seeds = re.find_iter(input[0].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
    let mut newseeds: Vec<i64> = Vec::new();
    for i in 0..seeds.len() / 2 {
        let mut a = (seeds[i * 2]..seeds[i * 2]+seeds[i * 2 + 1]).collect();
        newseeds.append(&mut a);
    }
    println!("12");
    let mut raw: Vec<Vec<Vec<i64>>> = Vec::new();

    let mut maptype = 0;
    raw.push(Vec::new());
    for i in 3..input.len() {
        if input[i] == "" { continue; }
        let res = re.find_iter(input[i].as_str()).filter_map(|digits| digits.as_str().parse::<i64>().ok()).collect::<Vec<_>>();
        if res.len() > 0 {
            raw[maptype].push(res);
        } else {
            maptype += 1;
            raw.push(Vec::new());
        }
    }

    (newseeds, raw)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}
