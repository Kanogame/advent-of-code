use std::fs::read_to_string;

fn main() {
    let input = read_lines("data.txt");
    
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