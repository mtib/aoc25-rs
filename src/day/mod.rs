use std::{error::Error, fmt::Display};

use crate::{
    error::{ExampleInputNotAvailableError, PuzzleNotImplementedError},
    util::{
        benchmark::Benchmarker,
        input::{PuzzleGetter, PuzzleInputType},
    },
};

include!(concat!(env!("OUT_DIR"), "/days.rs"));

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

static CURRENT_INPUT: std::sync::Mutex<Option<PuzzleInputType>> = std::sync::Mutex::new(None);
pub fn set_input_mode(mode: PuzzleInputType) {
    let mut guard = CURRENT_INPUT.lock().unwrap();
    *guard = Some(mode);
}

#[allow(unused)]
pub fn get_input_mode() -> PuzzleInputType {
    let guard = CURRENT_INPUT.lock().unwrap();
    guard.unwrap()
}

static IS_BENCHMARKING: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
pub fn set_benchmarking(is_benchmarking: bool) {
    let mut guard = IS_BENCHMARKING.lock().unwrap();
    *guard = is_benchmarking;
}

#[allow(unused)]
pub fn is_benchmarking() -> bool {
    let guard = IS_BENCHMARKING.lock().unwrap();
    *guard
}

#[macro_export]
macro_rules! example_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            if (!$crate::day::is_benchmarking() && $crate::day::get_input_mode() == $crate::util::input::PuzzleInputType::Example) {
                println!($($arg)*);
            }
        }
    };
}

#[macro_export]
macro_rules! actual_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            if (!$crate::day::is_benchmarking() && $crate::day::get_input_mode() == $crate::util::input::PuzzleInputType::Actual) {
                println!($($arg)*);
            }
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
        benchmarker: &mut dyn Benchmarker,
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
