use std::{fs::read_to_string};

fn main() {
    let input = read_lines("data.txt");
    
    
}


fn part_one(input: Vec<String>) {
    let mut res = 0;
    for line in input {
        let mut isData = false;
        let mut win: Vec<i32> = Vec::new();
        let mut data: Vec<i32> = Vec::new();
        for i in line[9..].split(" ") {
            if i != "" {
                if isData {
                    data.push(i.parse::<i32>().unwrap())
                } else {
                    if i == "|" {
                        isData = true;
                        continue;
                    }
                    win.push(i.parse::<i32>().unwrap())
                }
            }
        }
        data.retain(|x| win.contains(x));
        if data.len() > 0 {
            res += 2_i32.pow(data.len() as u32 - 1);
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