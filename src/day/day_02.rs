use crate::{
    day::{Day, Solution},
    util::number::parse_u8_slice_to_i64,
};
use rayon::prelude::*;

struct Day02;

impl Day02 {
    fn sum_invalid_ids(&self, from: i64, to: i64) -> i64 {
        let mut invalid_id_sum = 0;
        for id in from..=to {
            let decimal_len = ((id as f64).log10().floor() as u32) + 1;
            if decimal_len % 2 != 0 {
                continue;
            }
            let power = 10_i64.pow(decimal_len / 2);
            let front = id / power;
            let back = id % power;
            if front == back {
                invalid_id_sum += id
            }
        }
        invalid_id_sum
    }
    fn repeated_split_check(&self, number: i64, part_len: u32, decimal_len: u32) -> bool {
        let part1 = number / 10_i64.pow(decimal_len - part_len);
        for i in 1..(decimal_len / part_len) {
            let partn =
                number / 10_i64.pow(decimal_len - (i + 1) * part_len) % 10_i64.pow(part_len);
            if partn != part1 {
                return false;
            }
        }
        return true;
    }
    fn sum_repeated_invalid_ids(&self, from: i64, to: i64) -> i64 {
        let mut invalid_id_sum = 0;
        for id in from..=to {
            let decimal_len = ((id as f64).log10().floor() as u32) + 1;
            let part_len_options: Vec<u32> = (1..=decimal_len / 2)
                .filter(|d| decimal_len % d == 0)
                .collect();
            for part_len in part_len_options {
                if self.repeated_split_check(id, part_len, decimal_len) {
                    invalid_id_sum += id;
                    break;
                }
            }
        }
        invalid_id_sum
    }
}

impl Solution for Day02 {
    fn number(&self) -> u8 {
        2
    }
    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .trim_ascii_end()
            .split(|&c| c == b',')
            .par_bridge()
            .map(|range| {
                let dash = range.iter().position(|&c| c == b'-').unwrap();
                let start: i64 = parse_u8_slice_to_i64(&range[..dash]);
                let end: i64 = parse_u8_slice_to_i64(&range[dash + 1..]);
                self.sum_invalid_ids(start, end)
            })
            .sum();
        Ok(sum)
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .trim_ascii_end()
            .split(|&c| c == b',')
            .par_bridge()
            .map(|range| {
                let dash = range.iter().position(|&c| c == b'-').unwrap();
                let start: i64 = parse_u8_slice_to_i64(&range[..dash]);
                let end: i64 = parse_u8_slice_to_i64(&range[dash + 1..]);
                self.sum_repeated_invalid_ids(start, end)
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
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 4174379265);
    }
}
