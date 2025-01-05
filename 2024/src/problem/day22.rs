use std::i64;

use crate::generic_problem::{self, Day};

const STEPS: i32 = 2000;
const MODULA: i64 = i64::MAX >> 39;

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day22"),
        day_id: 22,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let secrets = input
        .lines
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut res: i64 = 0;

    for secret in secrets {
        let mut secret = secret;
        for _ in 0..STEPS {
            secret = ((secret * 64) ^ secret) % 16777216;
            secret = ((secret / 32) ^ secret) % 16777216;
            secret = (secret * 2048 ^ secret) % 16777216;
        }

        res += secret;
    }

    println!("{}", res);
}

fn get_unique(arr: &[i64; 4]) -> i64 {
    return arr
        .iter()
        .map(|x| *x + 9)
        .enumerate()
        .fold(0, |accum, (i, x)| accum + 19_i64.pow(i as u32) * x);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let secrets = input
        .lines
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut seq_map = vec![(0, STEPS as usize); 137568];

    for (m, secret) in secrets.iter().enumerate() {
        let mut secret = *secret;
        let mut diff_q = [10; 4];
        let mut prev_price = secret % 10;
        for _ in 0..STEPS {
            secret = ((secret << 6) ^ secret) & MODULA;
            secret = (secret >> 5) ^ secret;
            secret = ((secret << 11) ^ secret) & MODULA;
            let price = secret % 10;

            diff_q[0] = diff_q[1];
            diff_q[1] = diff_q[2];
            diff_q[2] = diff_q[3];
            diff_q[3] = price - prev_price;
            prev_price = price;

            if diff_q[0] == 10 {
                continue;
            }
            let q = get_unique(&diff_q);
            if seq_map[q as usize].1 == m {
                continue;
            }
            seq_map[q as usize] = (price + seq_map[q as usize].0, m);
        }
    }

    let mut max = 0;

    for (i, _) in seq_map {
        if i > max {
            max = i;
        }
    }

    println!("{:?}", max);
}
