use std::{
    cell::RefCell,
    error::Error,
    fmt::Display,
    hash::{self, Hash, Hasher},
};

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
    #[allow(unused)]
    fn get_type(&self) -> PuzzleInputType;
}

struct CookiePuzzleInputGetter {
    cache: RefCell<Option<String>>,
}

impl CookiePuzzleInputGetter {
    fn get_input(&self, year: u16, day: u8) -> Result<String, Box<dyn Error>> {
        if let Some(cached) = &*self.cache.borrow() {
            return Ok(cached.clone());
        }
        let session_cookie =
            std::env::var("AOC_COOKIE").expect("AOC_COOKIE environment variable not set");

        let cache_path = format!("inputs/{}_{}_{}.txt", year, day, {
            let mut hasher = hash::DefaultHasher::new();
            session_cookie.hash(&mut hasher);
            hasher.finish()
        });
        if let Ok(cached) = std::fs::read_to_string(&cache_path) {
            let mut cache = self.cache.borrow_mut();
            cache.replace(cached.clone());
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

    fn new() -> Self {
        Self {
            cache: RefCell::new(None),
        }
    }
}

pub struct DayCookiePuzzleInputGetter {
    year: u16,
    day: u8,
    inner: CookiePuzzleInputGetter,
}

impl DayCookiePuzzleInputGetter {
    pub fn new(year: u16, day: u8) -> Self {
        Self {
            year,
            day,
            inner: CookiePuzzleInputGetter::new(),
        }
    }
}

impl PuzzleGetter for DayCookiePuzzleInputGetter {
    fn get_input(&self) -> Result<String, Box<dyn Error>> {
        self.inner.get_input(self.year, self.day)
    }

    fn get_type(&self) -> PuzzleInputType {
        PuzzleInputType::Actual
    }
}
