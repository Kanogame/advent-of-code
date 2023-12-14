use std::fs::read_to_string;
use regex::Regex;


fn main() {
    let input = read_lines("data.txt");
    part_two(input);
}

fn part_two(input: Vec<String>) {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let re = Regex::new("([0-9]*).(blue|red|green)").unwrap();
    let mut res = 0;
    for (i, line) in input.into_iter().enumerate() {
        let mut minBlue = 0;
        let mut minGreen = 0;
        let mut minRed = 0;
        for (_, [num, color]) in re.captures_iter(&line).map(|c| c.extract()) {
            let num = num.parse::<i32>().unwrap();
            match color {
                "blue" => if num > minBlue {minBlue = num}
                "green" => if num > minGreen {minGreen = num}
                "red" => if num > minRed {minRed = num}
                other => ()
            }
        }
        res += minBlue * minGreen * minRed;
    }
    println!("{}", res)
}

fn part_one(input: Vec<String>) {
    const maxRed: i32 = 12;
    const maxGreen: i32 = 13;
    const maxBlue: i32 = 14;
    let mut res = 0;
    let re = Regex::new("([0-9]*).(blue|red|green)").unwrap();
    for (i, line) in input.into_iter().enumerate() {
        let mut correct = true;
        for (_, [num, color]) in re.captures_iter(&line).map(|c| c.extract()) {
            let num = num.parse::<i32>().unwrap();
            match color {
                "blue" => if num > maxBlue {correct = false}
                "green" => if num > maxGreen {correct = false}
                "red" => if num > maxRed {correct = false}
                other => ()
            }
        }
        if correct {
            res += i + 1
        }
    }
    println!("{}", res)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}