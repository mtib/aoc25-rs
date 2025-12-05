use crate::day::{Day, Solution};
use rayon::prelude::*;

struct Day05;

impl Day05 {
    fn merge_overlaps(&self, ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        ranges.sort_by_key(|r| r.0);

        let mut merged: Vec<(i64, i64)> = Vec::new();
        for range in ranges.iter() {
            if let Some(last) = merged.last_mut() {
                if range.0 <= last.1 {
                    last.1 = last.1.max(range.1);
                    continue;
                }
            }
            merged.push(*range);
        }
        merged
    }
}

impl Solution for Day05 {
    fn number(&self) -> u8 {
        5
    }
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let (ranges, ingredients) = {
            let mut ranges = Vec::new();
            let mut ingredients = Vec::new();
            let mut parsing_ranges = true;
            for line in input.lines() {
                if line.trim().is_empty() {
                    parsing_ranges = false;
                    continue;
                }
                if parsing_ranges {
                    let parts: Vec<&str> = line.split('-').collect();
                    let start: i64 = parts[0].parse()?;
                    let end: i64 = parts[1].parse()?;
                    ranges.push((start, end));
                } else {
                    let ingredient: i64 = line.trim().parse()?;
                    ingredients.push(ingredient);
                }
            }
            (ranges, ingredients)
        };

        Ok(ingredients
            .par_iter()
            .filter(|&&i| ranges.iter().any(|r| r.0 <= i && i <= r.1))
            .count() as i64)
    }
    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let ranges = {
            let mut ranges = Vec::new();
            for line in input.lines() {
                if line.trim().is_empty() {
                    break;
                }
                let parts: Vec<&str> = line.split('-').collect();
                let start: i64 = parts[0].parse()?;
                let end: i64 = parts[1].parse()?;
                ranges.push((start, end));
            }
            self.merge_overlaps(&mut ranges)
        };
        Ok(ranges.iter().map(|r| r.1 - r.0 + 1).sum())
    }
    fn get_example(&self) -> Option<&str> {
        Some(
            r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day05)
}
