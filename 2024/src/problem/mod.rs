use crate::generic_problem;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 22] = [
    day1::init,
    day2::init,
    day3::init,
    day4::init,
    day5::init,
    day6::init,
    day7::init,
    day8::init,
    day9::init,
    day10::init,
    day11::init,
    day12::init,
    day13::init,
    day14::init,
    day15::init,
    day16::init,
    day17::init,
    day18::init,
    day19::init,
    day20::init,
    day21::init,
    day22::init,
];
