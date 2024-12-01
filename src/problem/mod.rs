use crate::generic_problem;

pub mod day1;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 1] = [day1::init];
