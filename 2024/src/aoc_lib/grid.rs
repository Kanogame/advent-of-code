#[allow(dead_code)]
pub mod grid {
    use std::collections::{BTreeMap, HashMap};

    pub fn g_add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, a.1 + b.1)
    }

    pub fn g_sub(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 - b.0, a.1 - b.1)
    }

    pub fn print_grid(value: &HashMap<(i32, i32), char>) {
        let b_tree: BTreeMap<_, _> = value.clone().into_iter().collect();
        let mut s = 0;
        for ((y, _), v) in b_tree {
            if y != s {
                println!();
                s = y;
            }
            print!("{}", v);
        }
        println!()
    }

    pub const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
}
