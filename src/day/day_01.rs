use std::error::Error;

use crate::day::Solution;

use super::Day;

struct Day01;

impl Solution for Day01 {
    fn number(&self) -> u8 {
        1
    }

    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn Error>> {
        let mut position = 50;
        let mut zero_hit = 0;
        for line in input.lines().collect::<Vec<_>>() {
            let mut chars = line.chars();
            let direction = match chars.next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => return Err("Invalid direction".into()),
            };
            let steps: i32 = chars.collect::<String>().parse()?;
            position += direction * steps;
            position = ((position % 100) + 100) % 100;
            if position == 0 {
                zero_hit += 1;
            }
        }
        Ok(zero_hit)
    }

    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let mut position = 50;
        let mut zero_pass = 0;
        for line in input.lines().collect::<Vec<_>>() {
            let mut chars = line.chars();
            let direction = match chars.next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => return Err("Invalid direction".into()),
            };
            let steps: i32 = chars.collect::<String>().parse()?;
            position = {
                let mut pos = position;

                let diff_to_zero = match pos {
                    0 => 100,
                    _ => match direction {
                        1 => 100 - pos,
                        -1 => pos,
                        _ => return Err("Invalid direction".into()),
                    },
                };

                if steps >= diff_to_zero {
                    zero_pass += 1;
                }

                let spin = (steps - diff_to_zero) / 100;
                zero_pass += spin;

                pos += direction * steps;

                ((pos % 100) + 100) % 100
            };
        }
        Ok(zero_pass.into())
    }
    fn get_example(&self) -> Option<&str> {
        Some(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day01)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input).unwrap();
        assert_eq!(result, 6);
    }
}
