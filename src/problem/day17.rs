use regex::Regex;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day17"),
        day_id: 17,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn parse(lines: Vec<String>) -> Machine {
    let re: Regex = Regex::new(r": (.+)").unwrap();
    let mut reg: [i64; 3] = [0, 0, 0];
    let mut ins = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for [value] in re.captures_iter(line).map(|x| x.extract().1) {
            match i {
                4 => {
                    ins = value
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i64>>()
                }
                _ => reg[i] = value.parse().unwrap(),
            }
        }
    }

    Machine {
        reg_a: reg[0],
        reg_b: reg[1],
        reg_c: reg[2],
        insturctions: ins,
        pointer: 0,
        out: String::new(),
    }
}

#[derive(Debug)]
struct Machine {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    insturctions: Vec<i64>,
    pointer: usize,
    out: String,
}

impl Machine {
    fn run(&mut self) -> String {
        while !self.halt() {
            let (command, value) = self.next();

            match command {
                0 => self.adv(value),
                1 => self.bxl(value),
                2 => self.bst(value),
                3 => self.jnz(value),
                4 => self.bxc(value),
                5 => self.out(value),
                6 => self.bdv(value),
                7 => self.cdv(value),
                _ => {
                    println!("ERROR: illegal instruction");
                }
            };
        }
        return self.out[1..].to_string();
    }

    fn halt(&self) -> bool {
        self.pointer >= self.insturctions.len()
    }

    fn next(&self) -> (i64, i64) {
        return (
            self.insturctions[self.pointer],
            self.insturctions[self.pointer + 1],
        );
    }

    fn combo(&mut self, value: i64) -> i64 {
        match value {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => {
                println!("ERROR: illegal combo value");
                -1
            }
            _ => value,
        }
    }

    fn adv(&mut self, combo: i64) {
        // 0
        self.reg_a = self.reg_a >> self.combo(combo);
        self.pointer += 2;
    }

    fn bxl(&mut self, value: i64) {
        // 1
        self.reg_b = self.reg_b ^ value;
        self.pointer += 2;
    }

    fn bst(&mut self, combo: i64) {
        // 2
        self.reg_b = self.combo(combo) % 8;
        self.pointer += 2;
    }

    fn jnz(&mut self, value: i64) {
        // 3
        if self.reg_a != 0 {
            self.pointer = value as usize;
        } else {
            self.pointer += 2;
        }
    }

    fn bxc(&mut self, _: i64) {
        // 4
        self.reg_b = self.reg_b ^ self.reg_c;
        self.pointer += 2;
    }

    fn out(&mut self, combo: i64) {
        // 5
        let value = self.combo(combo) % 8;
        self.out += &format!(",{}", value);
        self.pointer += 2;
    }

    fn bdv(&mut self, combo: i64) {
        // 6
        self.reg_b = self.reg_a >> self.combo(combo);
        self.pointer += 2;
    }

    fn cdv(&mut self, combo: i64) {
        // 7
        self.reg_c = self.reg_a >> self.combo(combo);
        self.pointer += 2;
    }
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut m = parse(input.lines);
    println!("{}", m.run());
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let m = parse(input.lines);

    let mut a: i64 = 0;

    for b in m.insturctions.into_iter().rev() {
        // iterating over a % 8 variants
        for h in 0..8 {
            let a_tmp = (a << 3) + h;
            let g = h ^ 2;
            let c = a_tmp >> g;
            if ((g ^ c) ^ 7) % 8 == b {
                a <<= 3;
                a += h;
                break;
            }
        }
    }
    println!("{}", a);
}
