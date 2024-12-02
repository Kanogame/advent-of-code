use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day2"),
        day_id: 2,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(input: generic_problem::ProblemInput) -> Vec<Vec<i32>> {
    let mut report: Vec<Vec<i32>> = Vec::new();

    for line in input.lines {
        let parts: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        report.push(parts);
    }

    report
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let reports = parse(input);

    let mut res: usize = 0;
    let mut old_diff: i32 = 0;
    let mut diff: i32;
    let mut flag: bool;
    for report in reports {
        flag = true;
        for level in 1..report.len() {
            diff = report[level - 1] - report[level];
            if diff.abs() > 3 || diff.abs() < 1 {
                flag = false;
                break;
            }

            if old_diff != 0 && (diff.signum() != old_diff.signum()) {
                flag = false;
                break;
            }

            old_diff = diff;
        }

        if flag {
            res += 1;
        }
        old_diff = 0;
    }

    println!("{}", res);
}

fn is_safe(vec: &Vec<i32>) -> bool {
    let mut diff_arr: Vec<i32> = Vec::new();

    for level in 1..vec.len() {
        diff_arr.push(vec[level - 1] - vec[level]);
    }
    for i in diff_arr.iter() {
        if (*i).abs() > 3 || (*i).abs() < 1 || diff_arr[0].signum() != (*i).signum() {
            return false;
        }
    }

    return true;
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let reports = parse(input);

    let mut res = 0;

    for report in reports {
        if is_safe(&report) {
            res += 1;
        } else {
            for i in 0..report.len() {
                let mut tolerate: Vec<i32> = Vec::new();
                tolerate.append(&mut report[0..i].to_vec());
                tolerate.append(&mut report[i + 1..].to_vec());
                if is_safe(&tolerate) {
                    res += 1;
                    break;
                }
            }
        }
    }

    println!("{}", res);
}
