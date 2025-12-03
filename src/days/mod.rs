use std::{
    cell::RefCell,
    error::Error,
    fmt::Display,
    hash::{self, Hash, Hasher},
};

mod day_01;

pub fn get_days() -> Vec<Box<dyn Day>> {
    vec![day_01::day()]
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

pub trait NumberedDay {
    fn number(&self) -> u8;
}

pub trait DayRunner {
    fn run(
        &self,
        part: Part,
        _puzzle_getter: &dyn PuzzleGetter,
        _benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn Error>>;
}

pub trait PuzzleDayRunner {
    fn run_part1(
        &self,
        _puzzle_getter: &dyn PuzzleGetter,
        _benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn Error>> {
        Err("Part One not implemented".into())
    }
    fn run_part2(
        &self,
        _puzzle_getter: &dyn PuzzleGetter,
        _benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn Error>> {
        Err("Part Two not implemented".into())
    }
}

#[macro_export]
macro_rules! example_println {
    ($puzzle_getter:expr, $($arg:tt)*) => {
        if $puzzle_getter.get_type() == $crate::days::PuzzleInputType::Example {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! actual_println {
    ($puzzle_getter:expr, $($arg:tt)*) => {
        if $puzzle_getter.get_type() == $crate::days::PuzzleInputType::Actual {
            println!($($arg)*);
        }
    };
}

pub trait ExampleGetter {
    fn get_example(&self) -> String;
}

pub trait Day: NumberedDay + DayRunner + PuzzleDayRunner + ExampleGetter + PuzzleGetter {}

impl<T: NumberedDay + PuzzleDayRunner + ExampleGetter> Day for T {}

impl<T: PuzzleDayRunner> DayRunner for T {
    fn run(
        &self,
        part: Part,
        puzzle_getter: &dyn PuzzleGetter,
        benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn Error>> {
        match part {
            Part::One => self.run_part1(puzzle_getter, benchmarker),
            Part::Two => self.run_part2(puzzle_getter, benchmarker),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PuzzleInputType {
    Example,
    Actual,
}

impl Display for PuzzleInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleInputType::Example => write!(f, "Example"),
            PuzzleInputType::Actual => write!(f, "Actual"),
        }
    }
}

pub trait PuzzleGetter {
    fn get_input(&self) -> Result<String, Box<dyn Error>>;
    fn get_type(&self) -> PuzzleInputType;
}

impl<T: ExampleGetter> PuzzleGetter for T {
    fn get_input(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.get_example())
    }

    fn get_type(&self) -> PuzzleInputType {
        PuzzleInputType::Example
    }
}

pub trait Benchmarker {
    fn start_benchmark(&self);
    fn end_benchmark(&self);
    /// Returns the benchmark time in milliseconds
    fn elapsed_ms(&self) -> Option<f64>;
}

pub struct SimpleBenchmarker {
    start_time: RefCell<Option<std::time::Instant>>,
    end_time: RefCell<Option<std::time::Instant>>,
}

impl SimpleBenchmarker {
    pub fn new() -> Self {
        Self {
            start_time: RefCell::new(None),
            end_time: RefCell::new(None),
        }
    }
}

impl Benchmarker for SimpleBenchmarker {
    fn start_benchmark(&self) {
        self.start_time
            .borrow_mut()
            .replace(std::time::Instant::now());
    }

    fn end_benchmark(&self) {
        self.end_time
            .borrow_mut()
            .replace(std::time::Instant::now());
    }

    fn elapsed_ms(&self) -> Option<f64> {
        let start = self.start_time.borrow();
        let end = self.end_time.borrow();
        if let (Some(s), Some(e)) = (*start, *end) {
            let duration = e.duration_since(s);
            Some(duration.as_secs_f64() * 1000.0)
        } else {
            None
        }
    }
}

struct CookiePuzzleInputGetter;

impl CookiePuzzleInputGetter {
    fn get_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
        let session_cookie =
            std::env::var("AOC_COOKIE").expect("AOC_COOKIE environment variable not set");

        let cache_path = format!("inputs/{}_{}_{}.txt", year, day, {
            let mut hasher = hash::DefaultHasher::new();
            session_cookie.hash(&mut hasher);
            hasher.finish()
        });
        if let Ok(cached) = std::fs::read_to_string(&cache_path) {
            return Ok(cached);
        }

        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("Cookie", format!("session={}", session_cookie))
            .send()?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to fetch input for day {}: HTTP {}",
                day,
                response.status()
            )
            .into());
        }

        let input = response.text()?;
        std::fs::create_dir_all("inputs")?;
        std::fs::write(&cache_path, &input)?;
        Ok(input)
    }
}

pub struct DayCookiePuzzleInputGetter {
    year: u16,
    day: u8,
}

impl DayCookiePuzzleInputGetter {
    pub fn new(year: u16, day: u8) -> Self {
        Self { year, day }
    }
}

impl PuzzleGetter for DayCookiePuzzleInputGetter {
    fn get_input(&self) -> Result<String, Box<dyn Error>> {
        CookiePuzzleInputGetter::get_input(self.year, self.day)
    }

    fn get_type(&self) -> PuzzleInputType {
        PuzzleInputType::Actual
    }
}
