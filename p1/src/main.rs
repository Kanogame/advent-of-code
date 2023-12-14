use std::fs::read_to_string;

fn main() {
    let input = read_lines("data.txt");
    
    part_two(input);
}

fn part_two(input: Vec<String>) {
    let nums = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven", 
        "eight",
        "nine"
    ];

    let mut res = 0;
    for line in input {
        let mut first = 10;
        let mut last = 0;
        let mut offset: usize = 0;
        for (i, data) in line.chars().enumerate() {
            for (j, num) in nums.iter().enumerate() {
                if line[offset..i + 1].contains(num) {
                    if first == 10 {
                        first = j + 1;
                    }
                    last = j + 1;
                    offset = i;
                } else if data.is_digit(10) == true {
                    if first == 10 {
                        first = data.to_digit(10).unwrap() as usize;
                    }
                    last = data.to_digit(10).unwrap() as usize;
                }
            }
        }
        res += first * 10 + last
    }
    println!("{}", res);
}

fn part_one(input: Vec<String>) {
    let mut res = 0;
    let mut first = 10;
    let mut last = 0;
    for line in input {
        for i in line.chars() {
            if i.is_digit(10) == true {
                if first == 10 {
                    first = i.to_digit(10).unwrap()
                }
                last = i.to_digit(10).unwrap()
            }
        }
        res += (first * 10) + last;
        first = 10;
        last = 0;
    }
    println!("{}", res);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}