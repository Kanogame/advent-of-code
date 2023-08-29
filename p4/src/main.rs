use regex::Regex;
use std::fs::read_to_string;

struct Item {
    value: i32,
    marked: bool,
}
fn main() {
    //parsing
    let input = read_lines("p4.txt");
    let numbers = get_numbers(&input[0]);
    let mut cards: Vec<Vec<Vec<Item>>> = vec![];

    for i in 0..(input.len() - 1) / 6 {
        cards.push(parse_card(&input[i * 6 + 2..(i + 1) * 6 + 1]));
    }
    //solving
    let mut datalen = cards.len();
    'main_loop: for num_val in numbers {
        for value in cards.iter_mut().filter(|x| !slow_bingo(x)) {
            if mark_card(value, num_val) {datalen -= 1};
            if datalen == 0 {
                println!("{}", calculate_unmarked(value) * num_val);
                return;
            }
        }
    }
}

fn calculate_unmarked(data: & Vec<Vec<Item>>) -> i32 {
    let mut res: i32 = 0;
    for i in data {
        for j in i {
            if !j.marked {
                res += j.value;
            }
        }
    }

    return res;
}
 
fn print_card(data: & Vec<Vec<Item>>) {
    for i in data {
        for j in i {
            print!("{} {} ", j.value, j.marked);
        }
        println!();
    }
}

fn get_numbers(data: &String) -> Vec<i32> { 
    let mut res: Vec<i32> = vec![];
    let re: Regex = Regex::new(r"(?<value>\d+)").unwrap();

    for (_, [value]) in re.captures_iter(data).map(|c| c.extract()) {
        res.push(value.parse::<i32>().unwrap());
    }

    return res;
}

fn slow_bingo(data: &Vec<Vec<Item>>) -> bool {
    for row in 0..5 {
        if check_row(data, row) {
            return true;
        }
    }
    for column in 0..5 {
        if check_column(data, column) {
            return true;
        }
    }
    return false;
}

fn parse_card(data: &[String]) -> Vec<Vec<Item>> {
    let re: Regex = Regex::new(r"(?<value>\d+)").unwrap();
    let mut res: Vec<Vec<Item>>  = vec![];
    for (i, value) in data.into_iter().enumerate() {
        res.push(vec![]);
        for (_, [data]) in re.captures_iter(&value).map(|c| c.extract()) {
            res[i].push(Item {value: data.parse::<i32>().unwrap(), marked: false});
        }
    }
    return res;
}

fn mark_card(card: &mut Vec<Vec<Item>>, value: i32) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if card[i][j].value == value {
                card[i][j].marked = true;
                return bingo(card, i, j);
            }
        }
    }

    return false;
}

fn bingo(card: &mut Vec<Vec<Item>>, row: usize, column: usize) -> bool {
    return check_row(card, row) || check_column(card, column);
}

fn check_row(card: &Vec<Vec<Item>>, row: usize) -> bool {
    for column_f in 0..5 {
        if card[row][column_f].marked != true {
            return false;
        }
    }
    return true;
}

fn check_column(card: &Vec<Vec<Item>>, column: usize) -> bool {
    for row_f in 0..5 {
        if card[row_f][column].marked != true {
            return false;
        }
    }
    return true;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}