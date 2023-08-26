use std::fs::read_to_string;

fn main() {
    let mut data_o = read_lines("p3.txt");
    let mut data_co2 = data_o.clone();

    let len: i32 = data_o[0].len().try_into().unwrap();

    for i in 0..len {
        let mut common = 0;
        let datalen: i32 = data_o.len().try_into().unwrap();
        for line in &data_o {
            let num: Vec<char> = line.chars().collect();
            common += num[i as usize].to_string().parse::<i32>().unwrap();
        }

        let target: char; 
        if f64::from(common) >= f64::from(datalen) / 2f64 {
            target = '1';
        } else {
            target = '0';
        }

        data_o.retain(|x| x.chars().collect::<Vec<char>>()[i as usize] == target);
    }

    for i in 0..len {
        let mut common = 0;
        let datalen: i32 = data_co2.len().try_into().unwrap();
        if datalen == 1 {break;}
        for line in &data_co2 {
            let num: Vec<char> = line.chars().collect();
            common += num[i as usize].to_string().parse::<i32>().unwrap();
        }

        let target: char; 
        if f64::from(common) >= f64::from(datalen) / 2f64 {
            target = '0';
        } else {
            target = '1';
        }

        data_co2.retain(|x| x.chars().collect::<Vec<char>>()[i as usize] == target);
    }

    println!("{}", string_convert(&data_co2[0]) * string_convert(&data_o[0]));
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