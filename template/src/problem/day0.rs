use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day0"),
        day_id: 0,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

pub fn part_one(input: generic_problem::ProblemInput) {
    println!("{}", input.lines[0]);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    println!("{}", input.lines[0]);
}
