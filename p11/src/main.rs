use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize
}

fn main() {
    calculateDistance(getStarsCords(processInput(read_lines("data.txt"))));
}

fn processInput(input: Vec<String>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let mut doubleX = Vec::<usize>::new();
    'x: for x in 0..input[0].len() {
        for y in 0..input.len() {
            if input[y].chars().nth(x).unwrap() == '#' {
                continue 'x;
            }
        }
        doubleX.push(x);    
    }

    for line in input {
        let mut string = String::new();
        for (x, charV) in line.chars().enumerate() {
            string.push(charV);
            if doubleX.contains(&x) {
                string.push(charV);
            }
        }
        res.push(string);
    } 
    let input = res;
    let mut res = Vec::<String>::new();

    for line in input {
        res.push(line.clone());
        if !line.contains("#") {
            res.push(line);
        }
    }

    return res; 
}

fn calculateDistance(pos :Vec<Pos>) {
    let mut res = 0;
    for (start, start_pos) in pos.clone().iter().enumerate() {
        for i in start..pos.len() {
            let end_pos = pos[i];
            let diff_y: i32 = (start_pos.y as i32 - end_pos.y as i32).abs();
            let diff_x: i32 = (start_pos.x as i32 - end_pos.x as i32).abs();
            res += diff_y + diff_x;
        }
    }
    println!("{}", res);
}

fn getStarsCords(input: Vec<String>) -> Vec<Pos> {
    let mut res: Vec<Pos> = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, star) in line.chars().enumerate() {
            if star == '#' {    
                res.push(Pos{x: x, y: y})
            }
        }
    }
    res
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}