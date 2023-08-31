use std::fs::read_to_string;
use regex::Regex;

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

const array_size: usize = 10;

fn main() {
    //parse
    let input = read_lines("p5.txt");
    let mut input = parse_input(input);

    //solve
    let mut field: [[i32; array_size]; array_size] = [[0; array_size]; array_size];
    fill_field(&mut field, input);
    println!("{}", calc_field(&field));
    print_field(&field);
}

fn print_field(field: &[[i32; array_size]; array_size]) {
    for i in field {
        for j in i {
            print!("{j}");
        }
        println!();
    }
}

fn calc_field(field: &[[i32; array_size]; array_size]) -> i32{
    let mut res = 0;
    for i in field {
        for j in i {
            if *j > 1 {res += 1}; 
        }
    }

    res
}

fn fill_field(field: &mut [[i32; array_size]; array_size], data: Vec<Line>) {
    for line in data {
        if line.start.x == line.end.x {
            if line.end.y > line.start.y {
                for y in line.start.y..line.end.y + 1 {
                    field[y as usize][line.start.x as usize] += 1;
                }
            } else {
                for y in line.end.y..line.start.y + 1 {
                    field[y as usize][line.start.x as usize] += 1;
                }
            }
        } else if line.start.y == line.end.y {
            if line.end.x > line.start.x {
                for x in line.start.x..line.end.x + 1 {
                    field[ line.start.y as usize][x as usize] += 1;
                }
            } else {
                for x in line.end.x..line.start.x + 1{
                    field[ line.start.y as usize][x as usize] += 1;
                }
            }
        } else {
            let diff = (line.start.y - line.end.y).abs();
            let (startx, starty) = if line.start.x > line.end.x || line.start.y > line.end.y {(line.end.x, line.end.y)} else {(line.start.x, line.start.y)};
            println!("{}, {}", (starty), (startx));
            let posx = if line.start.x < line.end.x {-1} else{1};
            let posy = if line.start.y < line.end.y {-1} else{1};
            for i in 0..diff + 1{
                println!("{}, {}, {}", (starty + i * posy), (startx + i* posx), diff);
                field[(starty + i * posy).abs() as usize][(startx + i* posx).abs() as usize] += 1;
            }
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Line> {
    let re: Regex = Regex::new(r"(?<x1>\d+),(?<y1>\d+) -> (?<x2>\d+),(?<y2>\d+)").unwrap();
    let mut res: Vec<Line> = vec![];
    for value in input {
        let data = re.captures(value.as_str()).unwrap();
        res.push(Line { start: Point { x: data["x1"].parse::<i32>().unwrap(), y:data["y1"].parse::<i32>().unwrap() }, 
        end: Point { x: data["x2"].parse::<i32>().unwrap(), y:data["y2"].parse::<i32>().unwrap() } });
    }

    return res;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}