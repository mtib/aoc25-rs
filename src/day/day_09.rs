use std::cmp::{max, min};

use crate::{day::Solution, util::number::parse_u8_slice_to_i64};

use super::Day;

struct Day09;

struct Edge<'a> {
    start: &'a (i64, i64),
    end: &'a (i64, i64),
}

impl<'a> Edge<'a> {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

struct Rect<'a> {
    point_1: &'a (i64, i64),
    point_2: &'a (i64, i64),
}

impl<'a> Rect<'a> {
    fn interior_intersects(&self, edge: &Edge) -> bool {
        let rect_x1 = min(self.point_1.0, self.point_2.0) + 1;
        let rect_x2 = max(self.point_1.0, self.point_2.0) - 1;
        let rect_y1 = min(self.point_1.1, self.point_2.1) + 1;
        let rect_y2 = max(self.point_1.1, self.point_2.1) - 1;
        if edge.is_vertical() {
            let x = edge.start.0;
            let y1 = min(edge.start.1, edge.end.1);
            let y2 = max(edge.start.1, edge.end.1);

            x >= rect_x1 && x <= rect_x2 && !(y1 > rect_y2 || y2 < rect_y1)
        } else {
            let y = edge.start.1;
            let x1 = min(edge.start.0, edge.end.0);
            let x2 = max(edge.start.0, edge.end.0);

            y >= rect_y1 && y <= rect_y2 && !(x1 > rect_x2 || x2 < rect_x1)
        }
    }
}

impl Solution for Day09 {
    fn number(&self) -> u8 {
        9
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let tiles: Vec<_> = input
            .trim_ascii_end()
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut parts = line.split(|&c| c == b',');
                (
                    parse_u8_slice_to_i64(parts.next().unwrap()),
                    parse_u8_slice_to_i64(parts.next().unwrap()),
                )
            })
            .collect();

        let tile_pairs = tiles.iter().flat_map(|t1| {
            tiles.iter().filter_map(move |t2| {
                if t1 >= t2 {
                    return None;
                }

                Some((t1, t2))
            })
        });

        Ok(tile_pairs
            .map(|((x1, y1), (x2, y2))| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
            .max()
            .unwrap())
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let tiles: Vec<_> = input
            .trim_ascii_end()
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut parts = line.split(|&c| c == b',');
                (
                    parse_u8_slice_to_i64(parts.next().unwrap()),
                    parse_u8_slice_to_i64(parts.next().unwrap()),
                )
            })
            .collect();

        let mut edges = Vec::with_capacity(tiles.len() + 1);
        for i in 0..tiles.len() {
            edges.push(Edge {
                start: unsafe { tiles.get_unchecked(i) },
                end: unsafe { tiles.get_unchecked((i + 1) % tiles.len()) },
            });
        }

        let tile_pairs = tiles.iter().flat_map(|t1| {
            let edges = &edges;
            tiles.iter().filter_map(move |t2| {
                if t1 >= t2 {
                    return None;
                }

                let rect = Rect {
                    point_1: t1,
                    point_2: t2,
                };
                for edge in edges {
                    if rect.interior_intersects(edge) {
                        return None;
                    }
                }

                Some((t1, t2))
            })
        });

        Ok(tile_pairs
            .map(|((x1, y1), (x2, y2))| ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1))
            .max()
            .unwrap())
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#,
        )
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
