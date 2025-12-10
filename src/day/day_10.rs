use std::collections::VecDeque;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use z3::{Optimize, ast::Int};

use crate::{day::Solution, util::number::parse_u8_slice_to_i64};

use super::Day;

struct Day10;

impl Day10 {}

struct Machine {
    target_lights: Box<[bool]>,
    buttons: Box<[Box<[usize]>]>,
    target_joltage: Box<[usize]>,
}

impl Machine {
    fn from_input(line: &[u8]) -> Self {
        let parts = line.split(|&c| c == b' ');
        let mut target_state = None;
        let mut buttons = Vec::new();
        let mut joltages = None;
        for part in parts {
            if part.starts_with(b"[") && part.ends_with(b"]") {
                let state = &part[1..part.len() - 1];
                target_state = Some(
                    state
                        .iter()
                        .map(|&c| match c {
                            b'#' => true,
                            b'.' => false,
                            _ => panic!("Invalid state character"),
                        })
                        .collect::<Box<[bool]>>(),
                );
            } else if part.starts_with(b"(") && part.ends_with(b")") {
                let button = &part[1..part.len() - 1];
                let button_read = button
                    .split(|&c| c == b',')
                    .map(|idx| parse_u8_slice_to_i64(idx) as usize)
                    .collect::<Box<[usize]>>();
                buttons.push(button_read);
            } else if part.starts_with(b"{") && part.ends_with(b"}") {
                let joltage = &part[1..part.len() - 1];
                let joltage_read = joltage
                    .split(|&c| c == b',')
                    .map(|idx| parse_u8_slice_to_i64(idx) as usize)
                    .collect::<Box<[usize]>>();
                joltages = Some(joltage_read);
            }
        }
        Machine {
            target_lights: target_state.unwrap(),
            buttons: buttons.into_boxed_slice(),
            target_joltage: joltages.unwrap(),
        }
    }

    fn initial_lights_state(&self) -> Box<[bool]> {
        vec![false; self.target_lights.len()].into_boxed_slice()
    }

    fn bfs_least_buttons_lights(&self) -> usize {
        let mut stack = VecDeque::new();
        stack.push_back((self.initial_lights_state(), 0));

        if self.initial_lights_state().as_ref() == self.target_lights.as_ref() {
            return 0;
        }

        while let Some((start_state, d)) = stack.pop_front() {
            for btn in 0..self.buttons.len() {
                let mut next_state = start_state.clone();
                for &idx in &self.buttons[btn] {
                    next_state[idx] = !next_state[idx];
                }
                if next_state.as_ref() == self.target_lights.as_ref() {
                    return d + 1;
                }
                stack.push_back((next_state, d + 1));
            }
        }

        unreachable!()
    }

    fn bfs_least_buttons_joltage(&self) -> usize {
        let optimize = Optimize::new();
        let btn_presses: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::fresh_const(format!("btn_{}", i).as_str()))
            .collect();

        btn_presses.iter().for_each(|btn| {
            optimize.assert(&btn.ge(0));
        });

        for target_idx in 0..self.target_joltage.len() {
            let mut sum_expr = Int::from_i64(0);
            for (btn_idx, btn) in self.buttons.iter().enumerate() {
                if btn.contains(&target_idx) {
                    sum_expr = sum_expr + &btn_presses[btn_idx]
                }
            }
            optimize.assert(&sum_expr.eq(self.target_joltage[target_idx] as i64));
        }

        let total_presses = btn_presses
            .iter()
            .fold(Int::from_i64(0), |acc, btn| acc + btn);

        optimize.minimize(&total_presses);

        optimize.check(&[]);

        if let Some(model) = optimize.get_model() {
            let total: i64 = model.eval(&total_presses, true).unwrap().as_i64().unwrap();
            return total as usize;
        }

        unreachable!()
    }
}

impl Solution for Day10 {
    fn number(&self) -> u8 {
        10
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let machines: Vec<Machine> = input
            .trim_ascii_end()
            .split(|&c| c == b'\n')
            .map(|line| Machine::from_input(line))
            .collect();

        Ok(machines
            .par_iter()
            .map(|m| m.bfs_least_buttons_lights() as i64)
            .sum())
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let machines: Vec<Machine> = input
            .trim_ascii_end()
            .split(|&c| c == b'\n')
            .map(|line| Machine::from_input(line))
            .collect();

        Ok(machines
            .par_iter()
            .map(|m| m.bfs_least_buttons_joltage() as i64)
            .sum())
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 33);
    }
}
