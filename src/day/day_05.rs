use crate::{
    day::{Day, Solution},
    util::number::parse_u8_slice_to_i64,
};
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
    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let (ranges, ingredients) = {
            let mut ranges = Vec::new();
            let mut ingredients = Vec::new();
            let mut parsing_ranges = true;
            for line in input.split(|&c| c == b'\n') {
                if line.len() == 0 {
                    parsing_ranges = false;
                    continue;
                }
                if parsing_ranges {
                    let dash_position = line.iter().position(|&c| c == b'-').unwrap();
                    let start: i64 = parse_u8_slice_to_i64(&line[..dash_position]);
                    let end: i64 = parse_u8_slice_to_i64(&line[dash_position + 1..]);
                    ranges.push((start, end));
                } else {
                    let ingredient: i64 = parse_u8_slice_to_i64(line);
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
    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let ranges = {
            let mut ranges = Vec::new();
            for line in input.split(|&c| c == b'\n') {
                if line.len() == 0 {
                    break;
                }
                let dash_position = line.iter().position(|&c| c == b'-').unwrap();
                let start: i64 = parse_u8_slice_to_i64(&line[..dash_position]);
                let end: i64 = parse_u8_slice_to_i64(&line[dash_position + 1..]);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 14);
    }
}
