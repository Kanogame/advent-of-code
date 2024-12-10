
use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("test"),
        day_id: 10,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn rec(x: i32, y: i32, prev: i32, map: &mut Vec<Vec<i32>>) -> i32 {
    if prev + 1 != map[y as usize][x as usize] {
        return 0;
    }
    if map[y as usize][x as usize] == 9 {
        return 1;
    }
    let mut res: i32 = 0;
    if x - 1 >= 0 {
        res += rec(x - 1, y, map[y as usize][(x) as usize], map);
map[y as usize][(x) as usize] = -1;
    }
    if x + 1 < map[0].len() as i32 {
        res += rec(x + 1, y, map[y as usize][(x) as usize], map);
map[y as usize][(x) as usize] = -1;
    }
    if y - 1 >= 0 {
        res += rec(x, y - 1, map[(y ) as usize][x as usize], map);
map[y as usize][(x) as usize] = -1;
    }
    if y + 1 < map.len() as i32 {
        res += rec(x, y + 1, map[(y) as usize][x as usize], map);
map[y as usize][(x) as usize] = -1;
    }
    return res;
}



pub fn part_one(input: generic_problem::ProblemInput) {
    let mut lines = input.lines.iter().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

            println!("{:?}", lines);
    let mut res = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 0 {
                let tmp = rec(x as i32, y as i32, -1, &mut lines);
                res += tmp;
    println!("{}", tmp);
            }
        }
    }
    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
//
}
