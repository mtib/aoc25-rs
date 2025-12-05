use crate::day::{Day, Solution};
use rayon::prelude::*;

struct Day02;

impl Day02 {
    fn simple_invalid_ids(&self, from: i64, to: i64) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        for id in from..=to {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            let (front, back) = id_str.split_at(id_str.len() / 2);
            if front == back {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }
    fn repeated_split_check(&self, s: &str, divisor: usize) -> bool {
        let part1 = &s[0..divisor];
        for i in 1..(s.len() / divisor) {
            if &s[i * divisor..(i + 1) * divisor] != part1 {
                return false;
            }
        }
        return true;
    }
    fn repeated_invalid_ids(&self, from: i64, to: i64) -> Vec<i64> {
        let mut invalid_ids = Vec::new();
        for id in from..=to {
            let id_str = id.to_string();
            let divisors: Vec<usize> = (1..=id_str.len() / 2)
                .filter(|d| id_str.len() % d == 0)
                .collect();
            for divisor in divisors {
                if self.repeated_split_check(&id_str, divisor) {
                    invalid_ids.push(id);
                    break;
                }
            }
        }
        invalid_ids
    }
}

impl Solution for Day02 {
    fn number(&self) -> u8 {
        2
    }
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .trim()
            .split(',')
            .par_bridge()
            .map(|range| {
                let parsed_range = range
                    .split('-')
                    .map(|r| r.parse().unwrap())
                    .collect::<Vec<i64>>();
                self.simple_invalid_ids(parsed_range[0], parsed_range[1])
                    .iter()
                    .sum::<i64>()
            })
            .sum();
        Ok(sum)
    }

    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .trim()
            .split(',')
            .par_bridge()
            .map(|range| {
                let parsed_range = range
                    .split('-')
                    .map(|r| r.parse().unwrap())
                    .collect::<Vec<i64>>();
                self.repeated_invalid_ids(parsed_range[0], parsed_range[1])
                    .iter()
                    .sum::<i64>()
            })
            .sum();
        Ok(sum)
    }
    fn get_example(&self) -> Option<&str> {
        Some(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day02)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input).unwrap();
        assert_eq!(result, 4174379265);
    }
}
