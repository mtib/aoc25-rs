use crate::days::{Day, ExampleGetter, NumberedDay, PuzzleDayRunner};

struct Day02;

impl NumberedDay for Day02 {
    fn number(&self) -> u8 {
        2
    }
}

impl PuzzleDayRunner for Day02 {
    fn run_part1(
        &self,
        puzzle_getter: &dyn super::PuzzleGetter,
        benchmarker: &dyn super::Benchmarker,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        fn invalid_ids(from: i64, to: i64) -> Vec<i64> {
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
        let input = puzzle_getter.get_input()?;

        benchmarker.start_benchmark();
        let sum = input
            .trim()
            .split(',')
            .map(|range| {
                let parsed_range = range
                    .split('-')
                    .map(|r| r.parse().unwrap())
                    .collect::<Vec<i64>>();
                invalid_ids(parsed_range[0], parsed_range[1])
            })
            .flatten()
            .sum();
        benchmarker.end_benchmark();

        Ok(sum)
    }

    fn run_part2(
        &self,
        puzzle_getter: &dyn super::PuzzleGetter,
        benchmarker: &dyn super::Benchmarker,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        fn repeated_split_chech(s: &str, divisor: usize) -> bool {
            let part1 = &s[0..divisor];
            for i in 1..(s.len() / divisor) {
                if &s[i * divisor..(i + 1) * divisor] != part1 {
                    return false;
                }
            }
            return true;
        }
        fn invalid_ids(from: i64, to: i64) -> Vec<i64> {
            let mut invalid_ids = Vec::new();
            for id in from..=to {
                let id_str = id.to_string();
                let divisors: Vec<usize> = (1..=id_str.len() / 2)
                    .filter(|d| id_str.len() % d == 0)
                    .collect();
                for divisor in divisors {
                    if repeated_split_chech(&id_str, divisor) {
                        invalid_ids.push(id);
                        break;
                    }
                }
            }
            invalid_ids
        }
        let input = puzzle_getter.get_input()?;

        benchmarker.start_benchmark();
        let sum = input
            .trim()
            .split(',')
            .map(|range| {
                let parsed_range = range
                    .split('-')
                    .map(|r| r.parse().unwrap())
                    .collect::<Vec<i64>>();
                invalid_ids(parsed_range[0], parsed_range[1])
            })
            .flatten()
            .sum();
        benchmarker.end_benchmark();

        Ok(sum)
    }
}

impl ExampleGetter for Day02 {
    fn get_example(&self) -> String {
        r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#
            .into()
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day02)
}
