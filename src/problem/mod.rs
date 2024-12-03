use crate::generic_problem;

pub mod day1;
pub mod day2;
pub mod day3;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 3] = [day1::init, day2::init, day3::init];
