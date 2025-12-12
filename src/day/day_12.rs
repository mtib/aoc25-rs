use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{day::Solution, example_println, util::number::parse_u8_slice_to_i64};

use super::Day;

struct Day12;

type PieceId = u8;

#[derive(Debug)]
struct PieceDefinition {
    pattern: Vec<Vec<bool>>,
    id: PieceId,
}

impl PieceDefinition {
    fn from_lines(lines: &[&[u8]]) -> Self {
        let mut iter = lines.iter();
        let id = iter.next().unwrap()[0] - b'0';
        let pattern = iter
            .map(|line| line.iter().map(|&c| c == b'#').collect())
            .collect();
        PieceDefinition { pattern, id }
    }

    fn rotated(&self, right: u8) -> Self {
        let mut new_pattern = self.pattern.clone();
        for _ in 0..right {
            let height = new_pattern.len();
            let width = new_pattern[0].len();
            let mut rotated = vec![vec![false; height]; width];
            for r in 0..height {
                for c in 0..width {
                    rotated[c][height - 1 - r] = new_pattern[r][c];
                }
            }
            new_pattern = rotated;
        }
        PieceDefinition {
            pattern: new_pattern,
            id: self.id,
        }
    }

    fn flipped(&self) -> Self {
        let height = self.pattern.len();
        let width = self.pattern[0].len();
        let mut flipped = vec![vec![false; width]; height];
        for r in 0..height {
            for c in 0..width {
                flipped[r][width - 1 - c] = self.pattern[r][c];
            }
        }
        PieceDefinition {
            pattern: flipped,
            id: self.id,
        }
    }
}

#[derive(Debug)]
struct BoardDefinition {
    width: usize,
    height: usize,
    requested_pieces: HashMap<PieceId, i64>,
}

impl BoardDefinition {
    fn from_line(line: &[u8]) -> Self {
        let parts: Vec<&[u8]> = line.split(|&c| c == b':').collect();
        let size_part = parts[0];
        let pieces_part = parts[1];

        let size_parts: Vec<&[u8]> = size_part.split(|&c| c == b'x').collect();
        let width = parse_u8_slice_to_i64(size_parts[0]) as usize;
        let height = parse_u8_slice_to_i64(size_parts[1]) as usize;

        let requested_pieces = pieces_part
            .split(|&c| c == b' ')
            .filter(|s| !s.is_empty())
            .map(|s| parse_u8_slice_to_i64(s))
            .enumerate()
            .map(|(id, count)| (id as PieceId, count))
            .collect();

        BoardDefinition {
            width,
            height,
            requested_pieces,
        }
    }
}

impl Day12 {
    fn parse_input(input: &[u8]) -> (Vec<PieceDefinition>, Vec<BoardDefinition>) {
        let mut chunks = Vec::new();
        for line in input.split(|&c| c == b'\n') {
            if line.is_empty() {
                chunks.push(Vec::new());
                continue;
            }
            if line.contains(&b'x') {
                // This is a board definition line
                chunks.push(vec![line]);
                continue;
            }
            if chunks.is_empty() {
                chunks.push(Vec::new());
            }
            chunks.last_mut().unwrap().push(line);
        }

        let mut piece_definitions = Vec::new();
        let mut board_definitions = Vec::new();

        for chunk in chunks.iter().filter(|c| !c.is_empty()) {
            example_println!("Processing chunk with {} lines", chunk.len());
            if chunk.len() == 1 {
                board_definitions.push(BoardDefinition::from_line(chunk[0]));
            } else {
                piece_definitions.push(PieceDefinition::from_lines(&chunk));
            }
        }

        (piece_definitions, board_definitions)
    }

    fn dfs_fit(board: &BoardDefinition, pieces: &Vec<PieceDefinition>) -> bool {
        struct PiecePlacement {
            piece_id: PieceId,
            rotation: u8,
            flipped: bool,
            x: usize,
            y: usize,
        }

        fn dfs(
            board: &BoardDefinition,
            pieces: &Vec<PieceDefinition>,
            placements: &mut Vec<PiecePlacement>,
            requested_pieces: &mut HashMap<PieceId, i64>,
        ) -> bool {
            if requested_pieces.values().all(|&count| count == 0) {
                return true;
            }

            let mut current_board = vec![vec![false; board.width]; board.height];

            for placement in placements.iter() {
                let piece = pieces.iter().find(|p| p.id == placement.piece_id).unwrap();
                let transformed_piece = if placement.flipped {
                    piece.flipped().rotated(placement.rotation)
                } else {
                    piece.rotated(placement.rotation)
                };

                for (py, row) in transformed_piece.pattern.iter().enumerate() {
                    for (px, &cell) in row.iter().enumerate() {
                        if cell {
                            current_board[placement.y + py][placement.x + px] = true;
                        }
                    }
                }
            }

            for piece in pieces {
                let count = requested_pieces.get(&piece.id).cloned().unwrap_or(0);
                if count <= 0 {
                    continue;
                }

                for &flipped in &[false, true] {
                    for rotation in 0..4 {
                        let transformed_piece = if flipped {
                            piece.flipped().rotated(rotation)
                        } else {
                            piece.rotated(rotation)
                        };

                        for x in 0..=board.width - transformed_piece.pattern[0].len() {
                            for y in 0..=board.height - transformed_piece.pattern.len() {
                                // Check if the piece can be placed at (x, y)
                                let mut can_place = true;
                                'outer: for (py, row) in
                                    transformed_piece.pattern.iter().enumerate()
                                {
                                    for (px, &cell) in row.iter().enumerate() {
                                        if cell && current_board[y + py][x + px] {
                                            can_place = false;
                                            break 'outer;
                                        }
                                    }
                                }

                                if can_place {
                                    placements.push(PiecePlacement {
                                        piece_id: piece.id,
                                        rotation,
                                        flipped,
                                        x,
                                        y,
                                    });
                                    *requested_pieces.get_mut(&piece.id).unwrap() -= 1;

                                    if dfs(board, pieces, placements, requested_pieces) {
                                        return true;
                                    }

                                    placements.pop();
                                    *requested_pieces.get_mut(&piece.id).unwrap() += 1;
                                }
                            }
                        }
                    }
                }
            }

            false
        }
        let mut placements = Vec::new();
        let mut requested_pieces = board.requested_pieces.clone();
        dfs(board, pieces, &mut placements, &mut requested_pieces)
    }
}

impl Solution for Day12 {
    fn number(&self) -> u8 {
        12
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let (piece_definitions, board_definitions) = Self::parse_input(input);

        let fits = board_definitions
            .par_iter()
            .filter(|board| Self::dfs_fit(board, &piece_definitions))
            .count();

        Ok(fits as i64)
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day12)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotate() {
        let piece = PieceDefinition::from_lines(&[b"0:", b"###", b"##.", b"##."]);
        let rotated_once = piece.rotated(1);
        assert_eq!(
            rotated_once.pattern,
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![false, false, true],
            ]
        );

        let rotated_twice = piece.rotated(2);
        assert_eq!(
            rotated_twice.pattern,
            vec![
                vec![false, true, true],
                vec![false, true, true],
                vec![true, true, true],
            ]
        );
    }

    #[test]
    fn flipped() {
        let piece = PieceDefinition::from_lines(&[b"0:", b"###", b"##.", b"##."]);
        let flipped = piece.flipped();
        assert_eq!(
            flipped.pattern,
            vec![
                vec![true, true, true],
                vec![false, true, true],
                vec![false, true, true],
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, todo!());
    }
}
