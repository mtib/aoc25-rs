use std::cmp::max;

use crate::day::{Day, Solution};
use rayon::prelude::*;

struct Day03;

impl Day03 {
    fn cascade(&self, selection: &mut Vec<u8>, new_digit: u8) {
        let mut free_digit = new_digit;
        for i in 0..selection.len() {
            if free_digit >= selection[i] {
                let old = selection[i];
                selection[i] = free_digit;
                free_digit = old;
            } else {
                break;
            }
        }
    }
}

impl Solution for Day03 {
    fn number(&self) -> u8 {
        3
    }
    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .split(|&c| c == b'\n')
            .par_bridge()
            .map(|bank| {
                let mut pair = (0, 0);
                let mut iter = bank.iter().map(|c| c - b'0').rev();
                (&mut iter).take(2).for_each(|d| {
                    pair.1 = pair.0;
                    pair.0 = d;
                });
                iter.for_each(|d| {
                    if d >= pair.0 {
                        let old = pair.0;
                        pair.0 = d;
                        pair.1 = max(old, pair.1);
                    }
                });

                (pair.0 * 10 + pair.1) as i64
            })
            .sum::<i64>();
        Ok(sum)
    }
    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .split(|&c| c == b'\n')
            .par_bridge()
            .map(|bank| {
                let mut iter = bank.iter().map(|c| c - b'0').rev();
                let mut selection = (&mut iter).take(12).collect::<Vec<u8>>();
                selection.reverse();
                iter.for_each(|d| self.cascade(&mut selection, d));
                selection
                    .iter()
                    .take(12)
                    .fold(0u64, |acc, &d| acc * 10 + d as u64) as i64
            })
            .sum();
        Ok(sum)
    }
    fn get_example(&self) -> Option<&str> {
        Some(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day03)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 357);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 3121910778619);
    }
}
