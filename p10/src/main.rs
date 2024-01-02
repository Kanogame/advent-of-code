use std::{fs::read_to_string, path};

struct Pos {
    x: usize,
    y: usize,
}

fn main() {
    let (mut map, pos) = get_matrix(read_lines("data.txt"));
    let mut path: Vec<i32> = Vec::new();
    findLoop(&mut map, pos, 'S', &mut path);
    print_matrix(&map);
    //print!("{}", (path.len() + 1) / 2);
}

fn findLoop(map: &mut Vec<Vec<char>>, pos: Pos, cur: char, path: &mut Vec<i32>) {
    match cur {
        'S' => {
            findBottom(map, &pos, path);
            findLeft(map, &pos, path);
            findTop(map, &pos, path);
            findRight(map, &pos, path);}
        '-' => {
            findRight(map, &pos, path);
            findLeft(map, &pos, path);
        }
        '|' => {
            findTop(map, &pos, path);
            findBottom(map, &pos, path);
        }
        'L' => {
            findTop(map, &pos, path);
            findRight(map, &pos, path);
        }
        'J' => {
            findTop(map, &pos, path);
            findLeft(map, &pos, path);
        }
        'F' => {
            findBottom(map, &pos, path);
            findRight(map, &pos, path);
        }
        '7' => {
            findBottom(map, &pos, path);
            findLeft(map, &pos, path);
        }
        _ => {}
    }
}


fn findRight(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>) {
    let right = map[pos.y][pos.x + 1];
    if right != ' ' {
        if right == '-' || right == 'J' || right == '7' {
            let temp = map[pos.y][pos.x + 1];
            map[pos.y][pos.x + 1] = 'B';
            path.push(1);
            findLoop(map, Pos{ x: pos.x + 1, y: pos.y }, temp, path)
        }
    }
}


fn findLeft(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>) {
    let left = map[pos.y][pos.x - 1];
    if left != ' ' {
        if left == '-' || left == 'L' || left == 'F' {
            let temp = map[pos.y][pos.x - 1];
            map[pos.y][pos.x - 1] = 'B';
            path.push(1);
            findLoop(map, Pos{ x: pos.x - 1, y: pos.y }, temp, path)
        }
    }
}

fn findTop(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>) {
    let top = map[pos.y - 1][pos.x];
    if top != ' ' {
        if top == '|' || top == '7' || top == 'F' {
            let temp = map[pos.y - 1][pos.x];
            map[pos.y - 1][pos.x] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x, y: pos.y - 1 }, temp, path)
        }
    }
}

fn findBottom(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>) {
    let bottom = map[pos.y + 1][pos.x];
    if bottom != ' ' {
        if bottom == '|' || bottom == 'J' || bottom == 'L' {
            let temp = map[pos.y + 1][pos.x];
            map[pos.y + 1][pos.x] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x, y: pos.y + 1 }, temp, path)
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        for charc in line {
            print!("{}", charc);
        }
        println!();
    }
}

fn get_matrix(input: Vec<String>) -> (Vec<Vec<char>>, Pos) {
    let mut res: Vec<Vec<char>> = Vec::new();
    let mut pos = Pos{x: 0, y: 0};

    for (y, line) in input.iter().enumerate() {
        let mut lineVec:Vec<char> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                if char == 'S' {
                    pos = Pos{x: x, y: y}
                }
                lineVec.push(char)
            } else {
                lineVec.push(' ')
            }
        }
        res.push(lineVec)
    }
    (res, pos)
}
 
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}