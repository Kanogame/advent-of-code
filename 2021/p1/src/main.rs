use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "data.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut data: [i32 ; 2000] = [0; 2000]; //fill the array

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        data[index] = line.parse().unwrap();
    }

    let mut count: usize = 0;

    for i in 0..(data.len() - 3) {
        let first_window = data[i] + data[i + 1] + data[i + 2];
        let second_window = data[i + 1] + data[i + 2] + data[i + 3];
        if second_window > first_window {
            count += 1;
        }
    }

    println!("{count}");
}
