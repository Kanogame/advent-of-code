use std::fs::read_to_string;
use regex::Regex;


fn main() {
    let input = read_lines("data.txt");
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