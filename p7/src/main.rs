use std::{fs::read_to_string, collections::btree_map::Values};
use regex::Regex;

fn main() {
    let hands = parse_input_pt1(read_lines("data.txt"));
    sort_cards(hands);
}

fn sort_cards(hands: Vec<(String, i32)>) {
    let mut summorized: Vec<(String, i32, i32, i32)> = Vec::new();
    for (hand, value) in hands {
        let mut temp = summorize_card(hand, value);
        temp.2 = get_hand_value(&temp.0);
        summorized.push(temp);
    }

    summorized.sort_by_key(|i| (i.1, i.2));
    let mut res = 0;
    for (i, val) in summorized.iter().enumerate() {
        res += (i as i32 + 1) * val.3; 
    }
    println!("{:?}", res);
}

fn get_hand_value(hand: &String) -> i32 {
    let mut res = "".to_string();
    for i in hand.chars() {
        res.push(match i {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            other => other,
        })
    }
    return i32::from_str_radix(res.as_str(), 16).unwrap();
}

fn summorize_card(hand: String, val: i32) -> (String, i32, i32, i32) {
    let mut vec: Vec<(char, i32)> = Vec::new();
    'hand: for card in hand.chars() {
        for (card_name, card_val) in vec.iter_mut() {
            if *card_name == card {
                *card_val += 1;
                continue 'hand;
            }
        }
        vec.push((card, 1));
    }

    let mut state = 0;
    for (_, i, ) in &vec {
        match i {
            2 => {if state == 1 {state = 2} else if state == 3 {state = 4} else {state = 1}}
            3 => {if state == 1 {state = 4} else {state = 3}}
            4 => {state = 5}
            5 => {state = 6}
            _ => {}
        }
    }

    return (hand, state, 0, val);
}

fn parse_input_pt1(input: Vec<String>) -> Vec<(String, i32)> {
    let re: Regex = Regex::new(r".{5}| [0-9]*").unwrap();
    let mut res: Vec<(String, i32)> = Vec::new();
    for line in input {
        let data = re.find_iter(line.as_str()).map(|i| i.as_str()).collect::<Vec<_>>();
        res.push((String::from(data[0]), data[1].trim().parse::<i32>().unwrap()));
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