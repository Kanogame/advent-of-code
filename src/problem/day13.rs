use regex::Regex;

use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day13"),
        day_id: 13,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Game {
    //cram method from linear algebra (finally, math strikes back!)
    fn solve(&self) -> Option<i64> {
        let det = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        let d_0 = self.prize.0 * self.button_b.1 - self.prize.1 * self.button_b.0;
        let d_1 = self.button_a.0 * self.prize.1 - self.button_a.1 * self.prize.0;

        if d_0 % det == 0 && d_1 % det == 0 {
            Some(3 * d_0 / det + d_1 / det)
        } else {
            None
        }
    }
}

fn parse(lines: Vec<String>) -> Vec<Game> {
    let re: Regex = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)Button B: X\+([0-9]+), Y\+([0-9]+)Prize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();

    let mut res: Vec<Game> = Vec::new();
    for [a_x, a_y, b_x, b_y, p_x, p_y] in re
        .captures_iter(
            lines
                .iter()
                .fold("".to_string(), |res, s| format!("{}{}", res, s))
                .as_str(),
        )
        .map(|x| x.extract().1.map(|el| el.parse::<i64>().unwrap_or(-1)))
    {
        res.push(Game {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (p_x, p_y),
        });
    }
    res
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let games = parse(input.lines);

    let res = games.iter().filter_map(|x| x.solve()).sum::<i64>();

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let games = parse(input.lines);

    let res = games
        .iter()
        .filter_map(|x| {
            let nx = Game {
                button_a: x.button_a,
                button_b: x.button_b,
                prize: (x.prize.0 + 10000000000000, x.prize.1 + 10000000000000),
            };
            nx.solve()
        })
        .sum::<i64>();

    println!("{}", res);
}
