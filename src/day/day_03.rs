use crate::day::{Day, Solution};

struct Day03;

impl Day03 {
    fn optimize_coverage_index(
        &self,
        batteries: &Vec<u32>,
        coverage: &mut Vec<usize>,
        index: usize,
    ) {
        let current_position = coverage[index];
        let current_value = batteries[current_position];
        let min_position = match index {
            0 => 0,
            _ => coverage[index - 1] + 1,
        };
        for hope in (current_value..=9).rev() {
            let best = batteries[min_position..current_position]
                .iter()
                .copied()
                .position(|d| d == hope);
            if let Some(p) = best {
                coverage[index] = p + min_position;
                return;
            }
        }
    }
    fn optimize_coverage(&self, batteries: &Vec<u32>, coverage: &mut Vec<usize>) {
        for i in 0..coverage.len() {
            self.optimize_coverage_index(batteries, coverage, i);
        }
    }
}

impl Solution for Day03 {
    fn number(&self) -> u8 {
        3
    }
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .lines()
            .map(|bank| {
                let batteries: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
                for left_digit in (0..=9).rev() {
                    let mut bank_iter = batteries.iter();
                    let left_digit_position =
                        if let Some(p) = bank_iter.position(|d| d == &left_digit) {
                            p
                        } else {
                            continue;
                        };
                    let base_iter: Vec<u32> = bank_iter.copied().collect();
                    for right_digit in (0..=9).rev() {
                        let _right_digit_position =
                            if let Some(p) = base_iter.iter().position(|d| d == &right_digit) {
                                p + left_digit_position + 1
                            } else {
                                continue;
                            };
                        return (left_digit * 10 + right_digit) as i64;
                    }
                }
                0
            })
            .sum();
        Ok(sum)
    }
    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let sum = input
            .lines()
            .map(|bank| {
                let batteries: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
                let mut coverage = vec![];
                for i in 0..12 {
                    coverage.push(batteries.len() - 12 + i)
                }
                self.optimize_coverage(&batteries, &mut coverage);
                coverage.iter().copied().fold(0, |acc, index| -> i64 {
                    acc * 10 + batteries[index] as i64
                })
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
