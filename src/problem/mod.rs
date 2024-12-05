use crate::generic_problem;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 5] =
    [day1::init, day2::init, day3::init, day4::init, day5::init];
