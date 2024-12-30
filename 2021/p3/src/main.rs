use std::fs::read_to_string;

fn main() {
    let data = read_lines("p3.txt");

    let len: i32 = data[0].len().try_into().unwrap();

    println!("{}", string_convert(&solve(len, data.clone(), false)) * string_convert(&solve(len, data, true)));
}

fn solve(len: i32, mut data: Vec<String>, is_co2:bool) -> String {
    for i in 0..len {
        let mut common = 0;
        let datalen: i32 = data.len().try_into().unwrap();
        if datalen == 1 {break;}
        for line in &data {
            let num: Vec<char> = line.chars().collect();
            common += num[i as usize].to_string().parse::<i32>().unwrap();
        }

        let target: char; 
        if f64::from(common) >= f64::from(datalen) / 2f64 {
            target = if is_co2 { '1' } else { '0' };
        } else {
            target = if is_co2 { '0' } else { '1' };
        }

        data.retain(|x| x.chars().collect::<Vec<char>>()[i as usize] == target);
    }

    return data[0].clone();
}

fn string_convert(data: &String) -> i32 {
    let res = i32::from_str_radix(&data, 2).unwrap();
    res
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)
        .collect() 
}