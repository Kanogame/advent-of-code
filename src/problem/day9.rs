use std::{
    collections::{HashMap, HashSet},
    str,
};

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("test"),
        day_id: 9,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let values: Vec<i64> = input.lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();
    let mut reverse: Vec<i64> = values
        .clone()
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .collect();

    let mut res: i64 = 0;
    let mut actual_position: i64 = -1;
    let mut reverse_position: i64 = 0;

    let max_len: i64 = (reverse.iter().sum::<i64>()) - 1;

    for (id, block_size) in values.into_iter().enumerate() {
        if id % 2 == 0 {
            //data

            for _ in 0..block_size {
                actual_position += 1;
                res += (id as i64 / 2) * actual_position;
                if actual_position == max_len {
                    break;
                }
            }
        } else {
            //free space
            for _ in 0..block_size {
                actual_position += 1;
                res += (reverse.len() as i64 - reverse_position - 1) * actual_position;
                reverse[reverse_position as usize] -= 1;
                if reverse[reverse_position as usize] == 0 {
                    reverse_position += 1;
                }
                if actual_position == max_len {
                    break;
                }
            }
        }

        if actual_position == max_len {
            break;
        }
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {

    //TODO

    //println!("{}", res);
}
