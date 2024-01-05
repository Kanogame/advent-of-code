use std::fs::read_to_string;

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn main() {
    let (mut map, pos) = get_matrix(read_lines("data.txt"));
    let mut path: Vec<i32> = Vec::new();
    let mut loopmap: Vec<Vec<char>> = Vec::new();
    for (i, line) in map.iter().enumerate() {
        loopmap.push(Vec::new());
        for _ in line {
            loopmap[i].push(' ');
        }
    }
    findLoop(&mut map, pos.clone(), 'S', &mut path, &mut loopmap);
    loopmap[pos.y][pos.x] = '7';
    //print_matrix(&loopmap);
    replace_crap(&mut loopmap);
}

fn replace_crap(map: &mut Vec<Vec<char>>) {
    let mut res = 0;
    for line in map {
        let mut linedata = 0;
        let mut start = ' ';
        for val in line {
            if *val == '|' {
                linedata += 1;
            } else if *val == '-' {
                continue;
            } else if *val != ' ' {
                if *val == 'F' || *val == 'L' {
                    start = *val;
                } else if start != ' ' {
                    if (start == 'F' && *val == 'J') || (start == 'L' && *val == '7') {
                        linedata += 1;
                    }
                    start = ' ';
                }
            } else {
                if linedata % 2 == 1 {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}

fn findLoop(map: &mut Vec<Vec<char>>, pos: Pos, cur: char, path: &mut Vec<i32>, loopmap: &mut Vec<Vec<char>>) {
    match cur {
        'S' => {
            findBottom(map, &pos, path, loopmap);
            findLeft(map, &pos, path, loopmap);
            findTop(map, &pos, path, loopmap);
            findRight(map, &pos, path, loopmap);}
        '-' => {
            findRight(map, &pos, path, loopmap);
            findLeft(map, &pos, path, loopmap);
        }
        '|' => {
            findTop(map, &pos, path, loopmap);
            findBottom(map, &pos, path, loopmap);
        }
        'L' => {
            findTop(map, &pos, path, loopmap);
            findRight(map, &pos, path, loopmap);
        }
        'J' => {
            findTop(map, &pos, path, loopmap);
            findLeft(map, &pos, path, loopmap);
        }
        'F' => {
            findBottom(map, &pos, path, loopmap);
            findRight(map, &pos, path, loopmap);
        }
        '7' => {
            findBottom(map, &pos, path, loopmap);
            findLeft(map, &pos, path, loopmap);
        }
        _ => {}
    }
}


fn findRight(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>, newmap: &mut Vec<Vec<char>>) {
    let right = map[pos.y][pos.x + 1];
    if right != ' ' {
        if right == '-' || right == 'J' || right == '7' {
            newmap[pos.y][pos.x + 1] = right;
            map[pos.y][pos.x + 1] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x + 1, y: pos.y }, right, path, newmap)
        }
    }
}

fn findLeft(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>, newmap: &mut Vec<Vec<char>>) {
    let left = map[pos.y][pos.x - 1];
    if left != ' ' {
        if left == '-' || left == 'L' || left == 'F' {
            newmap[pos.y][pos.x - 1] = left;
            map[pos.y][pos.x - 1] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x - 1, y: pos.y }, left, path, newmap)
        }
    }
}

fn findTop(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>, newmap: &mut Vec<Vec<char>>) {
    if pos.y == 0 {
        return;
    }
    let top = map[pos.y - 1][pos.x];
    if top != ' ' {
        if top == '|' || top == '7' || top == 'F' {
            newmap[pos.y - 1][pos.x] = top;
            map[pos.y - 1][pos.x] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x, y: pos.y - 1 }, top, path, newmap)
        }
    }
}

fn findBottom(map: &mut Vec<Vec<char>>, pos: &Pos, path: &mut Vec<i32>, newmap: &mut Vec<Vec<char>>) {
    let bottom = map[pos.y + 1][pos.x];
    if bottom != ' ' {
        if bottom == '|' || bottom == 'J' || bottom == 'L' {
            newmap[pos.y + 1][pos.x] = bottom;
            map[pos.y + 1][pos.x] = 'A';
            path.push(1);
            findLoop(map, Pos{ x: pos.x, y: pos.y + 1 }, bottom, path, newmap)
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