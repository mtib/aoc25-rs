use std::{cmp::{max, min}, collections::HashSet};

use crate::{day::Solution, example_println, util::number::parse_u8_slice_to_i64};

use super::Day;

struct Day09;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    x: i64,
    y: i64,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Solution for Day09 {
    fn number(&self) -> u8 {
        9
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let tiles: Vec<_> = input.trim_ascii_end().split(|&c| c == b'\n').map(
            |line| {
                let mut parts = line.split(|&c| c == b',');
                (parse_u8_slice_to_i64(parts.next().unwrap()), parse_u8_slice_to_i64(parts.next().unwrap()))
            },
        ).collect();

        let tile_pairs = tiles.iter().flat_map(|&(x1, y1)| {
            tiles.iter().filter_map(move |&(x2, y2)| {
                if (x1, y1) >= (x2, y2) {
                    return None;
                }

                Some(((x1, y1), (x2, y2)))
            })
        });

        Ok(tile_pairs.map(|((x1, y1), (x2,y2))| ((y2-y1).abs() + 1) * ((x2-x1).abs() + 1)).max().unwrap())
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let mut allowable: HashSet<Tile> = HashSet::new();
        let tiles: Vec<_> = input.trim_ascii_end().split(|&c| c == b'\n').map(
            |line| {
                let mut parts = line.split(|&c| c == b',');
                (parse_u8_slice_to_i64(parts.next().unwrap()), parse_u8_slice_to_i64(parts.next().unwrap()))
            },
        ).collect();

        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for i in 0..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[(i + 1) % tiles.len()];

            if x1 < min_x {
                min_x = x1;
            }
            if x1 > max_x {
                max_x = x1;
            }
            if y1 < min_y {
                min_y = y1;
            }
            if y1 > max_y {
                max_y = y1;
            }
            
            for y in min(y1, y2)..=max(y1, y2) {
                for x in min(x1, x2)..=max(x1, x2) {
                    allowable.insert(Tile { x, y });
                }
            }
        }

        let tile_pairs = tiles.iter().flat_map(|&(x1, y1)| {
            let walls = &allowable;
            tiles.iter().filter_map(move |&(x2, y2)| {
                if (x1, y1) >= (x2, y2) {
                    return None;
                }

                for wall in walls {
                    if (wall.x > min(x1, x2) && wall.x < max(x1, x2)) &&
                       (wall.y > min(y1, y2) && wall.y < max(y1, y2)) {
                        return None;
                    }
                }

                Some(((x1, y1), (x2, y2)))
            })
        });

        Ok(tile_pairs.map(|((x1, y1), (x2,y2))| ((y2-y1).abs() + 1) * ((x2-x1).abs() + 1)).max().unwrap())
    }

    fn get_example(&self) -> Option<&str> {
        Some(r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#)
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day09)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 50);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 24);
    }
}
