use crate::generic_problem::{self, Day};

pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day9"),
        day_id: 9,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}

pub fn part_one(input: generic_problem::ProblemInput) {
    let values: Vec<i64> = input.lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();
    let mut reverse: Vec<i64> = values
        .clone()
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .collect();

    let mut res: i64 = 0;
    let mut actual_position: i64 = -1;
    let mut reverse_position: i64 = 0;

    let max_len: i64 = (reverse.iter().sum::<i64>()) - 1;

    for (id, block_size) in values.into_iter().enumerate() {
        if id % 2 == 0 {
            //data

            for _ in 0..block_size {
                actual_position += 1;
                res += (id as i64 / 2) * actual_position;
                if actual_position == max_len {
                    break;
                }
            }
        } else {
            //free space
            for _ in 0..block_size {
                actual_position += 1;
                res += (reverse.len() as i64 - reverse_position - 1) * actual_position;
                reverse[reverse_position as usize] -= 1;
                if reverse[reverse_position as usize] == 0 {
                    reverse_position += 1;
                }
                if actual_position == max_len {
                    break;
                }
            }
        }

        if actual_position == max_len {
            break;
        }
    }

    println!("{}", res);
}

pub fn part_two(input: generic_problem::ProblemInput) {
    let values: Vec<usize> = input.lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut disk: Vec<usize> = Vec::new();

    let mut empty_cache: Vec<(usize, usize)> = Vec::new();
    let mut full_cache: Vec<(usize, usize)> = Vec::new();

    for (id, block_size) in values.iter().enumerate() {
        let file_id = id / 2;
        if id % 2 == 1 {
            empty_cache.push((disk.len(), *block_size));
        } else {
            full_cache.push((disk.len(), *block_size));
        }
        for _ in 0..*block_size {
            if id % 2 == 0 {
                disk.push(file_id);
            } else {
                disk.push(0);
            }
        }
    }

    for (full_pos, full_size) in full_cache.iter().rev() {
        for (pos, size) in empty_cache.iter_mut() {
            if *full_size <= *size && *pos < *full_pos {
                for k in *pos..(*pos + *full_size) {
                    disk[k] = disk[*full_pos];
                }

                for k in *full_pos..(*full_pos + *full_size) {
                    disk[k] = 0;
                }

                *pos += *full_size;
                *size -= *full_size;
                break;
            }
        }
    }

    let mut cksum = 0;
    for (i, val) in disk.into_iter().enumerate() {
        cksum += (i as i64) * (val as i64);
    }

    println!("{:?}", cksum);
}
