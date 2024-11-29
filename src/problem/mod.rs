use crate::generic_problem;

pub mod day0;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 1] = [day0::init];
