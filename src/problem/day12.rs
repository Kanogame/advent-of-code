use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("test"),
        day_id: 12,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

fn get_perimeter(diff_x: i32, diff_y: i32, letter: char, lines: &Vec<Vec<char>>) -> i32 {
    let left = diff_x - 1 >= 0 && lines[diff_y as usize][(diff_x - 1) as usize] == letter;
    let right = diff_x + 1 < lines[0].len() as i32
        && lines[diff_y as usize][(diff_x + 1) as usize] == letter;
    let top = diff_y - 1 >= 0 && lines[(diff_y - 1) as usize][diff_x as usize] == letter;
    let bottom =
        diff_y + 1 < lines.len() as i32 && lines[(diff_y + 1) as usize][diff_x as usize] == letter;
    let sum = left as i32 + right as i32 + top as i32 + bottom as i32;

    if sum == 4 {
        return -4;
    } else if sum == 3 {
        return -2;
    } else if sum == 2 {
        return 0;
    } else if sum == 1 {
        return 2;
    }
    return 4;
}

fn rec_region_p1(
    x: i32,
    y: i32,
    letter: char,
    replace: char,
    lines: &mut Vec<Vec<char>>,
) -> (i32, i32) {
    if lines[y as usize][x as usize] != letter || lines[y as usize][x as usize] == replace {
        return (0, 0);
    }
    let mut area = 1;
    let mut perimeter = get_perimeter(x, y, replace, &lines);
    lines[y as usize][x as usize] = replace;
    if x - 1 >= 0 {
        let res = rec_region_p1(x - 1, y, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if x + 1 < lines[0].len() as i32 {
        let res = rec_region_p1(x + 1, y, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if y - 1 >= 0 {
        let res = rec_region_p1(x, y - 1, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }
    if y + 1 < lines.len() as i32 {
        let res = rec_region_p1(x, y + 1, letter, replace, lines);
        area += res.0;
        perimeter += res.1;
    }

    return (area, perimeter);
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let mut lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut res = 0;

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            let ch = lines[i as usize][j as usize];
            if ch.is_uppercase() {
                let rec = rec_region_p1(j, i, ch, ch.to_ascii_lowercase(), &mut lines);
                res += rec.0 * rec.1;
            }
        }
    }
    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let mut lines = input
        .lines
        .into_iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut res = 0;

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            let ch = lines[i as usize][j as usize];
            if ch.is_uppercase() {
                let rec = rec_region_p2(j, i, ch, ch.to_ascii_lowercase(), &mut lines);
                res += rec.0 * rec.1;
            }
        }
    }
    println!("{}", res);
}
