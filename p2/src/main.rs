use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


fn main() {
    let filename = "p2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re: Regex = Regex::new(r"((?<type>[a-z]*) (?<value>\d{1}))").unwrap();

    let mut height: usize = 0;
    let mut aim: usize = 0;
    let mut pos: usize = 0;

    for line in reader.lines() {
        let hay = line.unwrap();
        let a = re.captures(hay.as_str()).unwrap();
        let datatype = &a["type"];
        match datatype {
            "down" => aim += &a["value"].parse().unwrap(),
            "up" => aim -= &a["value"].parse().unwrap(),
            "forward" => {
                let pos_val = &a["value"].parse().unwrap(); 
                pos += pos_val;
                height += aim * pos_val;
        },
            _ => (),
        }
    }

    println!("{}", height * pos);

}