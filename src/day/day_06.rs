use std::fmt;

use crate::{day::Solution, util::number::parse_u8_slice_to_i64};

use super::Day;

struct Day06;

struct VerticalIter<'a, A: Copy + fmt::Debug> {
    data: &'a [Box<[A]>],
    col: usize,
    row: usize,
}

impl<A: Copy + fmt::Debug> VerticalIter<'_, A> {
    fn new<'a>(data: &'a [Box<[A]>], col: usize) -> VerticalIter<'a, A> {
        VerticalIter { data, col, row: 0 }
    }
}

impl<'a, A: Copy + fmt::Debug> Iterator for VerticalIter<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.data.len() {
            return None;
        }
        if self.col >= self.data[self.row].len() {
            return None;
        }
        let value = self.data[self.row][self.col];
        self.row += 1;
        Some(value)
    }
}

impl Solution for Day06 {
    fn number(&self) -> u8 {
        6
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let mut rows = Vec::new();
        for line in input.split(|&c| c == b'\n') {
            let line = std::str::from_utf8(line)?.trim();
            if line.is_empty() {
                continue;
            }
            let nums = line.split_whitespace().collect::<Box<[&str]>>();
            rows.push(nums);
        }
        let ops = rows.pop().unwrap();
        Ok((0..ops.len()).fold(0i64, |acc, col| {
            let column_iter = VerticalIter::new(&rows, col);
            let mut column_values = column_iter.map(|s| s.parse::<i64>().unwrap());
            let first_value = column_values.next().unwrap();
            let result = match ops[col] {
                "*" => column_values.fold(first_value, |a, b| a * b),
                "+" => column_values.fold(first_value, |a, b| a + b),
                _ => panic!("Unknown operation"),
            };
            acc + result
        }))
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let chars = input
            .split(|&c| c == b'\n')
            .map(|line| line.iter().copied().collect::<Box<[u8]>>())
            .collect::<Box<[Box<[u8]>]>>();
        let ops = chars.last().ok_or("No input lines")?;
        let left_edges: Box<_> = ops
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| {
                if c == b'+' || c == b'*' {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let ops = left_edges.iter().map(|&i| ops[i]).collect::<Box<[u8]>>();

        Ok(left_edges
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let op = ops[i];
                let column_end = if i + 1 < left_edges.len() {
                    left_edges[i + 1] - 2
                } else {
                    chars[0].len() - 1
                };
                (p..=column_end)
                    .map(|c| VerticalIter::new(&chars, c))
                    .map(|col_iter| {
                        let mut num_chars = Vec::new();
                        for ch in col_iter {
                            if ch.is_ascii_digit() {
                                num_chars.push(ch);
                            }
                        }
                        parse_u8_slice_to_i64(&num_chars)
                    })
                    .reduce(|acc, next| match op {
                        b'*' => acc * next,
                        b'+' => acc + next,
                        _ => panic!("Unknown operation"),
                    })
                    .unwrap()
            })
            .sum())
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day06)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 3263827);
    }
}
