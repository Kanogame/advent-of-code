use std::{fs::read_to_string};
use regex::Regex;

#[derive(Clone)]
struct seed {
    muted: bool, 
    value: i64
}

impl std::fmt::Debug for seed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(muted: {}, value: {})", self.muted, self.value)
    }
}

fn main() {
    let input = read_lines("data.txt");
    let (seeds, raw) = parse_input_pt2(input);
    let result = process_raw(&seeds, &raw);
    println!("{}", result.iter().min().unwrap())
}

fn process_raw(seeds: &Vec<i64>,raw: &Vec<Vec<Vec<i64>>>) -> Vec<i64> {
    let mut tempseed: Vec<seed> = Vec::new();
    for i in seeds {
        tempseed.push(seed { muted: false, value: *i })
    }
    
    for maptype in 0..raw.len() {
        println!("{}", maptype);
        for i in raw[maptype].iter() {
            for (j, seed) in (&mut tempseed).into_iter().enumerate() {
                if seed.value <= i[1] + i[2] - 1  && seed.value >= i[1] && !seed.muted {
                    seed.value = seed.value + (i[0] - i[1]);
                    seed.muted = true;
                } 
            }
        }
        for i in &mut tempseed {
            i.muted = false;
        }
    }

    let mut result: Vec<i64> = Vec::new();
    for i in tempseed {
        result.push(i.value)
    }
    return result; 
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


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}
