# This is the Advent of Code template in Rust.

## To add new day:

- Create new file in src/problem/dayX.rs
- Write init function

```rust
pub fn init() -> generic_problem::Day {
    return Day {
        name: String::from("day0"),
        day_id: 0,
        part_one: Box::new(part_one),
        part_two: Box::new(part_two),
    };
}
```

- Add module to src/problem/mod.rs
- Add init function to MODULE_LIST

## To add new input file:

- Create inputs folder
- Insert text into dayX.txt
