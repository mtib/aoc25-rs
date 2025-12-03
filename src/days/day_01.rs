use crate::days::{Benchmarker, ExampleGetter, NumberedDay, PuzzleDayRunner, PuzzleGetter};

use super::Day;

struct Day01;

impl NumberedDay for Day01 {
    fn number(&self) -> u8 {
        1
    }
}

impl PuzzleDayRunner for Day01 {
    fn run_part1(
        &self,
        puzzle_getter: &dyn PuzzleGetter,
        benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let input = puzzle_getter.get_input()?;
        benchmarker.start_benchmark();
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
        benchmarker.end_benchmark();
        Ok(zero_hit)
    }

    fn run_part2(
        &self,
        puzzle_getter: &dyn PuzzleGetter,
        benchmarker: &dyn Benchmarker,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let input = puzzle_getter.get_input()?;
        benchmarker.start_benchmark();
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
        benchmarker.end_benchmark();
        Ok(zero_pass.into())
    }
}

impl ExampleGetter for Day01 {
    fn get_example(&self) -> String {
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
"#
        .to_string()
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day01)
}
