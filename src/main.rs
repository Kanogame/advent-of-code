use std::{env, fs::read_to_string};

mod aoc_lib;
use generic_problem::{Day, ProblemInput};
use problem::MODULE_LIST;
mod problem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (day, part, all) = parse_arguments(args);
    let mut ran = false;

    for problem in MODULE_LIST {
        let init_value: Day = problem();
        if all || init_value.day_id == day {
            let lines = load_file(format!("./inputs/{}.txt", init_value.name));
            if all {
                println!("Running solution for day {}", init_value.day_id);
            }

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
            if !all {
                break;
            }
        }
    }

    if !ran {
        println!(
            "No day number was supplied. Use -d [number] to select day, -p [number] to select part"
        )
    }
}

fn parse_arguments(args: Vec<String>) -> (i32, i32, bool) {
    let mut day_number: i32 = -1;
    let mut part_number: i32 = -1;
    let mut all: bool = false;

    for el in 0..args.len() {
        match args[el].as_str() {
            "-d" => {
                if el != args.len() - 1 {
                    day_number = match args[el + 1].parse::<i32>() {
                        Ok(file) => file,
                        Err(_) => {
                            println!("Error while parsing arguments: Not an integer");
                            -1
                        }
                    };
                }
            }
            "-p" => {
                if el != args.len() - 1 {
                    part_number = match args[el + 1].parse::<i32>() {
                        Ok(file) => file,
                        Err(_) => {
                            println!("Error while parsing arguments: Not an integer");
                            -1
                        }
                    };
                }
            }
            "-a" => {
                all = true;
            }
            _ => {}
        }
    }
    if !all {
        return (day_number, part_number, false);
    }

    return (-1, -1, true);
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
    //struct Tests {
    //    pub part_one: String,
    //    pub part_two: String,
    //}

    pub struct Day {
        pub name: String,
        pub day_id: i32,
        pub part_one: Box<dyn Fn(ProblemInput)>,
        pub part_two: Box<dyn Fn(ProblemInput)>,
    }
}
