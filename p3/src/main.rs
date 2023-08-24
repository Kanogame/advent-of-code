use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::read_to_string;

fn main() {
    let data = read_lines("p3.txt");

    let len = data[0].len();
    let datalen: i32 = data.len().try_into().unwrap();

    let mut common = vec![0; len];

    for (line) in data {
        for (i, num) in line.chars().enumerate() {
            common[i] += num.to_string().parse::<i32>().unwrap();
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for val in common {
        if val > datalen / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            epsilon += "1";
            gamma += "0";
        }
    }

    println!("{}", string_convert(gamma) * string_convert(epsilon));
}

fn string_convert(data: String) -> i32 {
    let res = i32::from_str_radix(&data, 2).unwrap();
    res
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}