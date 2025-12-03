use std::{error::Error, fmt::Display};

use crate::{
    error::{ExampleInputNotAvailableError, PuzzleNotImplementedError},
    util::{
        benchmark::Benchmarker,
        input::{PuzzleGetter, PuzzleInputType},
    },
};

mod day_01;
mod day_02;
mod day_03;

pub fn get_days() -> Vec<Box<dyn Day>> {
    vec![day_01::day(), day_02::day(), day_03::day()]
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

impl Part {
    pub fn to_number(&self) -> u8 {
        match self {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

static CURRENT_MODE: std::sync::Mutex<Option<PuzzleInputType>> = std::sync::Mutex::new(None);
pub fn set_current_mode(mode: PuzzleInputType) {
    let mut guard = CURRENT_MODE.lock().unwrap();
    *guard = Some(mode);
}

#[allow(unused)]
pub fn get_current_mode() -> PuzzleInputType {
    let guard = CURRENT_MODE.lock().unwrap();
    guard.unwrap()
}

#[macro_export]
macro_rules! example_println {
    ($($arg:tt)*) => {
        if $crate::day::get_current_mode() == $crate::util::input::PuzzleInputType::Example {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! actual_println {
    ($($arg:tt)*) => {
        if $crate::day::get_current_mode() == $crate::util::input::PuzzleInputType::Actual {
            println!($($arg)*);
        }
    };
}

pub trait Solution {
    fn number(&self) -> u8;
    fn get_example(&self) -> Option<&str> {
        None
    }
    #[allow(unused_variables)]
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        Err(PuzzleNotImplementedError.into())
    }
    #[allow(unused_variables)]
    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        Err(PuzzleNotImplementedError.into())
    }
}

impl<T: Solution> Day for T {}

pub trait Day: Solution + PuzzleGetter {}

impl dyn Day {
    pub fn run(
        &self,
        part: Part,
        puzzle_getter: &dyn PuzzleGetter,
        benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn Error>> {
        let input = puzzle_getter.get_input()?;
        benchmarker.start_benchmark();
        let result = match part {
            Part::One => self.run_part_1(&input),
            Part::Two => self.run_part_2(&input),
        };
        benchmarker.end_benchmark();
        result
    }
}

impl<T: Solution> PuzzleGetter for T {
    fn get_input(&self) -> Result<String, Box<dyn Error>> {
        match self.get_example() {
            Some(example) => Ok(example.to_string()),
            None => Err(ExampleInputNotAvailableError.into()),
        }
    }

    fn get_type(&self) -> PuzzleInputType {
        PuzzleInputType::Example
    }
}
