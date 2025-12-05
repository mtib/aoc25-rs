use crate::day::{Day, Solution};
use rayon::prelude::*;

struct Day04;

struct Map {
    lines: Box<[Box<[CellState]>]>,
    width: usize,
    height: usize,
    mask: Vec<Vec<bool>>,
}

impl Map {
    fn from_input<'a>(input: &str) -> Map {
        let lines = input.lines().collect::<Box<[&str]>>();
        let height = lines.len();
        let width = lines[0].len();
        let lines = lines
            .iter()
            .map(|line| {
                let cells: Box<[CellState]> = line
                    .chars()
                    .map(|c| {
                        if c == '@' {
                            CellState::Filled
                        } else {
                            CellState::Empty
                        }
                    })
                    .collect::<Box<[CellState]>>();
                cells
            })
            .collect::<Box<[Box<[CellState]>]>>();

        Map {
            lines,
            width,
            height,
            mask: vec![vec![false; width]; height],
        }
    }
}

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Empty,
    Filled,
}

impl Day04 {
    fn is_blocked(&self, map: &Map, x: usize, y: usize) -> bool {
        let bx = x as isize;
        let by = y as isize;
        let mut count = 0;
        for (dx, dy) in DIRECTIONS {
            let nx = bx + dx;
            let ny = by + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < map.width
                && (ny as usize) < map.height
                && map.lines[ny as usize][nx as usize] == CellState::Filled
                && !map.mask[ny as usize][nx as usize]
            {
                count += 1;
            }

            if count >= 4 {
                return true;
            }
        }
        return false;
    }
}

impl Solution for Day04 {
    fn number(&self) -> u8 {
        4
    }
    fn run_part_1(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let map = Map::from_input(input);

        let count = map
            .lines
            .par_iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == CellState::Filled)
                    .filter(|(x, _)| !self.is_blocked(&map, *x, y))
                    .count() as i64
            })
            .sum::<i64>();

        Ok(count)
    }
    fn run_part_2(&self, input: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let mut map = Map::from_input(input);
        let mut total = 0;
        loop {
            let moved = map
                .lines
                .par_iter()
                .enumerate()
                .flat_map_iter(|(y, line)| {
                    let map = &map;
                    line.iter()
                        .enumerate()
                        .filter(move |(x, c)| **c == CellState::Filled && !map.mask[y][*x])
                        .filter(move |(x, _)| !self.is_blocked(&map, *x, y))
                        .map(move |(x, _)| (x, y))
                })
                .collect::<Box<_>>();

            if moved.len() == 0 {
                return Ok(total);
            }
            total += moved.len() as i64;
            for (x, y) in moved {
                map.mask[y][x] = true;
            }
        }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day_04_part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input).unwrap();
        assert_eq!(result, 13);
    }
    #[test]
    fn test_day_04_part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input).unwrap();
        assert_eq!(result, 43);
    }
}
