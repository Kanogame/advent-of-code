use std::{env, error, fmt::Error, fs::read_to_string, num::ParseIntError};

use generic_problem::{Day, ProblemInput};
use problem::MODULE_LIST;
mod problem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (day, part) = parse_arguments(args);
    let mut ran = false;

    for problem in MODULE_LIST.iter() {
        let init_value: Day = problem();
        if init_value.day_id == day {
            let lines = load_file(format!("./inputs/{}.txt", init_value.name));

            if part == -1 {
                (init_value.part_one)(ProblemInput {
                    lines: lines.clone(),
                });
                (init_value.part_two)(ProblemInput { lines: lines });
            } else if part == 1 {
                (init_value.part_one)(ProblemInput { lines: lines });
            } else if part == 2 {
                (init_value.part_two)(ProblemInput { lines: lines });
            }
            ran = true;
        }
        break;
    }

    if !ran {
        println!(
            "No day number was supplied. Use -d [number] to select day, -p [number] to select part"
        )
    }
}

fn parse_arguments(args: Vec<String>) -> (i32, i32) {
    let mut day_number: i32 = -1;
    let mut part_number: i32 = -1;

    for el in 0..args.len() {
        match args[el].as_str() {
            "-d" => {
                if el != args.len() - 1 {
                    day_number = match args[el + 1].parse::<i32>() {
                        Ok(file) => file,
                        Err(error) => {
                            println!("Error while parsing arguments: Not an integer");
                            0
                        }
                    };
                }
            }
            "-p" => {
                if el != args.len() - 1 {
                    part_number = match args[el + 1].parse::<i32>() {
                        Ok(file) => file,
                        Err(error) => {
                            println!("Error while parsing arguments: Not an integer");
                            0
                        }
                    };
                }
            }
            _ => {}
        }
    }
    (day_number, part_number)
}

fn load_file(path: String) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

mod generic_problem {

    pub struct ProblemInput {
        pub lines: Vec<String>,
    }

    // test?
    pub struct Tests {
        pub part_one: String,
        pub part_two: String,
    }

    pub struct Day {
        pub name: String,
        pub day_id: i32,
        pub part_one: Box<dyn Fn(ProblemInput)>,
        pub part_two: Box<dyn Fn(ProblemInput)>,
    }
}
