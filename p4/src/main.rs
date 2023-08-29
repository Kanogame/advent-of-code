use regex::Regex;

struct Item {
    value: i32,
    marked: bool,
}

fn main() {

    let re: Regex = Regex::new(r"(?<value>\d+)").unwrap();

    let data  = [
        String::from("60 79 46  9 58"),
        String::from("97 81  6 94 84"),
        String::from("38 40 17 61 29"),
        String::from("11 28  0 91 15"),
        String::from("24 77 34 59 36"),
    ];
    let mut card = parse_card(data);

    mark_card(&mut card, 60);
    mark_card(&mut card, 79);
    mark_card(&mut card, 46);
    mark_card(&mut card, 9);
    mark_card(&mut card, 58);
    println!("{}", mark_card(&mut card, 58));
    
    //let mut results = vec![];
    //for (_, [data]) in re.captures_iter(hay).map(|c| c.extract()) {
    //    results.push(data);
    //}

    for i in card {
        for j in i {
            print!("{} {} ", j.value, if j.marked {"t"} else {"f"});
        }
        println!();
    }
}

fn parse_card(data: [String; 5]) -> Vec<Vec<Item>> {
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
