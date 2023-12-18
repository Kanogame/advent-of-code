use std::{fs::read_to_string, collections::HashSet};

fn main() {
    let data = vec![
    "467..114..".to_string(),
    "...*......".to_string(),
    "..35..633.".to_string(),
    "......#...".to_string(),
    "617*......".to_string(),
    ".....+.58.".to_string(),
    "..592.....".to_string(),
    "......755.".to_string(),
    "...$.*....".to_string(),
    ".664.598..".to_string()];
    //let data = read_lines("data.txt");
    println!("{}", run(data))
}

fn run(mut input:Vec<String>) -> u32 {
    let grid: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut sum = 0;

    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if *char == '*' {
                let n = gear_neighbors(&grid, row_num, col_num);
                if n.len() == 2 {
                    sum += n[0] * n[1];
                }
            }
        }
    }

    sum
}

fn gear_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<u32> {
    let mut neighbors: Vec<(usize, usize)> = vec![];

    for r in row.saturating_sub(1)..=row + 1 {
        if r >= grid.len() {
            continue;
        }
        for c in col.saturating_sub(1)..=col + 1 {
            if c >= grid[0].len() || (r == row && c == col) {
                continue;
            }
            neighbors.push((r, c));
        }
    }

    println!("{:?}", neighbors);

    let mut numbers = vec![];
    let mut added_points = HashSet::new();

    for (r, c) in neighbors {
        if !added_points.contains(&(r, c)) && grid[r][c].is_ascii_digit() {
            println!("{}, {}", r, c);
            let mut c_left = c;
            while c_left > 0 && grid[r][c_left - 1].is_ascii_digit() {
                c_left -= 1;
            }

            let mut c_right = c;
            while c_right + 1 < grid[r].len() && grid[r][c_right + 1].is_ascii_digit() {
                c_right += 1;
            }

            for y in c_left..=c_right {
                added_points.insert((r, y));
            }

            let num_str: String = grid[r][c_left..=c_right].iter().collect();

            numbers.push(num_str.parse().unwrap());
        }
    }
    println!("{:?}", numbers);
    numbers
}

fn part_one(data: Vec<String>) {
    let mut res = 0;
    for (i, line) in data.clone().into_iter().enumerate() {
        let mut curnum = "".to_string();
        let mut start: i32 = -1;
        for (j, letter) in line.chars().enumerate() {
            if letter.is_digit(10) {
                if start == -1 {
                    start = j as i32;
                }
                curnum.push(letter);
            } 
            if (!letter.is_digit(10) || j == 139) && curnum != "".to_string() && start != -1{
                let mut adder = 0;
                if i == 0 {
                    if check_slice(&data[i], start as usize, j) || check_slice(&data[i + 1], start as usize, j) {
                        adder = curnum.parse::<i32>().unwrap();
                    }
                }
                else if i == data.len() -1 {
                    if check_slice(&data[i], start as usize, j) || check_slice(&data[i - 1], start as usize, j)  {
                        adder = curnum.parse::<i32>().unwrap();
                    }
                }
                else {
                    if check_slice(&data[i], start as usize, j) || check_slice(&data[i + 1], start as usize, j) || check_slice(&data[i - 1], start as usize, j)  {
                        adder = curnum.parse::<i32>().unwrap();
                    }
                }
                res += adder;
                curnum = "".to_string();
                start = -1;
            }
        }         
    }
    println!("{}", res)
}


fn check_slice(slice: &String, start: usize, end: usize) -> bool {
    let symbols = "1234567890.";
    let mut start = start;
    let mut end = end;
    if start != 0 {
        start -= 1;
    }
    if end < 140 {
        end += 1;
    }
    for i in start..end {
        if !symbols.contains(slice.chars().nth(i).unwrap()) {
            return true;
        }
    }
    false
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}