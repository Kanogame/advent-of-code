use std::str::FromStr;

use generic_problem::Problem;

mod problem;

fn main() {
    let problems = vec![
        generic_problem::Day {
            name: String::from("this"),
            data: vec![String::from("value")],
            runner: Box::new(problem::day::run),
        },
        generic_problem::Day {
            name: String::from("that"),
            data: vec![String::from("value2")],
            runner: Box::new(problem::daysec::run),
        },
    ];

    for problem in problems.iter() {
        problem.run();
    }
}

mod generic_problem {

    pub trait Problem {
        fn run(&self);
    }

    pub struct DayContext {
        pub name: String,
        pub data: Vec<String>,
    }

    impl Day {
        fn as_day_context(&self) -> DayContext {
            return DayContext {
                name: self.name.clone(),
                data: self.data.clone(),
            };
        }
    }

    pub struct Day {
        pub name: String,
        pub data: Vec<String>,
        pub runner: Box<dyn Fn(DayContext)>,
    }

    impl Problem for Day {
        fn run(&self) {
            (self.runner.as_ref())(self.as_day_context())
        }
    }
}
