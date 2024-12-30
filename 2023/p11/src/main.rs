use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize
}

fn main() {
    //calculateDistance(getStarsCords_pt1(processInput_pt1(read_lines("data.txt"))));
    let(emptyY, emptyX, input) = processInput_pt2(read_lines("data.txt"));
    calculateDistance(getStarsCords_pt2(input, emptyY, emptyX, 1000000));
}

fn processInput_pt2(input: Vec<String>) -> (Vec<usize>, Vec<usize>, Vec<String>) {
    let mut emptyX = Vec::<usize>::new();
    let mut emptyY = Vec::<usize>::new();
    'x: for x in 0..input[0].len() {
        for y in 0..input.len() {
            if input[y].chars().nth(x).unwrap() == '#' {
                continue 'x;
            }
        }
        emptyX.push(x);    
    }
    
    for (y, line) in input.iter().enumerate() {
        if !line.contains("#") {
            emptyY.push(y)
        }
    } 
    return (emptyY, emptyX, input);
}

fn getStarsCords_pt2(input: Vec<String>, emptyY: Vec<usize>, emptyX: Vec<usize>, times: usize) -> Vec<Pos> {
    let mut res: Vec<Pos> = Vec::new();
    let mut yoffset = 0;
    for (y, line) in input.iter().enumerate() {
        let mut xoffset = 0;
        if emptyY.contains(&y) {
            yoffset += times - 1;
        }
        for (x, star) in line.chars().enumerate() {
            if emptyX.contains(&x) {
                xoffset += times - 1;
            }
            if star == '#' {    
                res.push(Pos{x: x + xoffset, y: y + yoffset});
            }
        }
    }
    res
}

fn processInput_pt1(input: Vec<String>) -> Vec<String> {
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
    let mut res: i64 = 0;
    for (start, start_pos) in pos.clone().iter().enumerate() {
        for i in start..pos.len() {
            let end_pos = pos[i];
            let diff_y: i32 = (start_pos.y as i32 - end_pos.y as i32).abs();
            let diff_x: i32 = (start_pos.x as i32 - end_pos.x as i32).abs();
            res += diff_y as i64 + diff_x as i64;
        }
    }
    println!("{}", res);
}

fn getStarsCords_pt1(input: Vec<String>) -> Vec<Pos> {
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