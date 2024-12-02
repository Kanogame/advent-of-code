use crate::generic_problem;

pub mod day1;
pub mod day2;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 2] = [day1::init, day2::init];
