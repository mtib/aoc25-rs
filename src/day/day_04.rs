use std::sync::Mutex;

use crate::{
    day::{Day, Solution},
    example_println,
};
use rayon::prelude::*;

struct Day04;

impl Day04 {
    fn count_adjacent(&self, lines: &[&str], width: usize, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && ny >= 0
                    && (nx as usize) < width
                    && (ny as usize) < lines.len()
                    && lines[ny as usize].chars().nth(nx as usize).unwrap() == '@'
                {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Solution for Day04 {
    fn number(&self) -> u8 {
        4
    }
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let lines: Box<[&str]> = input.lines().collect();
        let width = lines[0].len();

        let count = lines
            .par_iter()
            .enumerate()
            .map(|(y, &line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '@')
                    .map(|(x, _)| self.count_adjacent(&lines, width, x, y))
                    .filter(|&count| count < 4)
                    .count() as i64
            })
            .sum::<i64>();

        Ok(count)
    }
    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let lines: Mutex<Vec<String>> = Mutex::new(input.lines().map(|s| s.to_string()).collect());
        let width = lines.lock().unwrap()[0].len();

        let count = Mutex::new(0);

        loop {
            let pre_count = count.lock().unwrap().clone();
            let current_map_string: Box<[String]> = {
                let lines = lines.lock().unwrap();
                lines.iter().cloned().collect()
            };
            let current_map: Box<[&str]> = current_map_string.iter().map(|s| s.as_str()).collect();
            for line in current_map.iter() {
                example_println!("{}", line);
            }
            example_println!("current count = {}\n", count.lock().unwrap());
            current_map
                .par_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '@')
                        .filter(|(x, _)| self.count_adjacent(&current_map, width, *x, y) < 4)
                        .map(|(x, _)| (x, y))
                        .collect::<Vec<(usize, usize)>>()
                })
                .for_each(|(x, y)| {
                    {
                        let mut count = count.lock().unwrap();
                        *count += 1;
                    }
                    let mut lines = lines.lock().unwrap();
                    let line = &mut lines[y];
                    let mut chars: Vec<char> = line.chars().collect();
                    chars[x] = '.';
                    *line = chars.into_iter().collect();
                });

            if *count.lock().unwrap() == pre_count {
                break;
            }
        }
        Ok(count.lock().unwrap().clone())
    }
    fn get_example(&self) -> Option<&str> {
        Some(
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day04)
}
