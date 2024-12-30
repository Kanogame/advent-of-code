fn main() {
    let data = read_lines("p3.txt");

    let len = data[0].len();
    let datalen: i32 = data.len().try_into().unwrap();

    let mut common = vec![0; len];

    for (line) in data {
        for (i, num) in line.chars().enumerate() {
            common[i] += num.to_string().parse::<i32>().unwrap();
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for val in common {
        if val > datalen / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            epsilon += "1";
            gamma += "0";
        }
    }

    println!("{}", string_convert(gamma) * string_convert(epsilon));
}
