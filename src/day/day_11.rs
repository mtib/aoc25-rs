use std::{collections::HashMap, mem::swap};

use matrixmultiply::sgemm;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    day::{Solution, get_input_mode},
    util::input::PuzzleInputType,
};

use super::Day;

struct Day11;

impl Day11 {
    fn parse_input(input: &[u8]) -> HashMap<&[u8], Vec<&[u8]>> {
        let mut graph = HashMap::new();
        for line in input.split(|&c| c == b'\n').filter(|l| !l.is_empty()) {
            let parts: Vec<&[u8]> = line.split(|&c| c == b':').collect();
            let node = parts[0];
            let edges = if parts.len() > 1 {
                parts[1]
                    .split(|&c| c == b' ')
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                Vec::new()
            };
            graph.insert(node, edges);
        }
        graph
    }

    fn count_paths_adj<'a>(
        from: &'a [u8],
        to: &'a [u8],
        graph: &'a HashMap<&[u8], Vec<&[u8]>>,
    ) -> i64 {
        let mut adjacency_list: Vec<Vec<f32>> = Vec::new();
        let ordering = {
            let mut ordering = graph.keys().cloned().collect::<Vec<_>>();
            ordering.push(b"out");
            ordering
        };

        for &node in ordering.iter() {
            let mut edges = Vec::new();
            for &other_node in ordering.iter() {
                if let Some(neighbors) = graph.get(node) {
                    if neighbors.contains(&other_node) {
                        edges.push(1.0);
                    } else {
                        edges.push(0.0);
                    }
                } else {
                    edges.push(0.0);
                }
            }
            adjacency_list.push(edges);
        }

        let from_index = ordering.iter().position(|&n| n == from).unwrap();
        let to_index = ordering.iter().position(|&n| n == to).unwrap();

        let n = ordering.len();
        let mut count = 0.0;
        let adjacency_matrix = adjacency_list.concat();
        let mut adjacency_matrix_squared = adjacency_matrix.clone();
        let mut temp = vec![0.0; n * n];
        loop {
            if (adjacency_matrix_squared[from_index * n + to_index]) > 0.0 {
                count += adjacency_matrix_squared[from_index * n + to_index];
            }

            if adjacency_matrix_squared.iter().all(|&x| x == 0.0) {
                break;
            }

            unsafe {
                sgemm(
                    n,
                    n,
                    n,
                    1.0,
                    adjacency_matrix_squared.as_ptr(),
                    n as isize,
                    1,
                    adjacency_matrix.as_ptr(),
                    n as isize,
                    1,
                    0.0,
                    temp.as_mut_ptr(),
                    n as isize,
                    1,
                );
            }
            swap(&mut adjacency_matrix_squared, &mut temp);
        }

        count as i64
    }

    fn count_paths(
        from: &[u8],
        to: &[u8],
        graph: &HashMap<&[u8], Vec<&[u8]>>,
        must_not: Option<&[&[u8]]>,
    ) -> i64 {
        let mut path = vec![from];

        fn explore<'a>(
            path: &mut Vec<&'a [u8]>,
            graph: &HashMap<&[u8], Vec<&'a [u8]>>,
            end: &[u8],
            must_not: Option<&[&'a [u8]]>,
        ) -> i64 {
            let last = path.last().unwrap();
            if last == &end {
                return 1;
            }

            let neighbors = match graph.get(last) {
                Some(n) => n,
                None => return 0,
            };

            let mut count = 0;
            for &neighbor in neighbors.iter() {
                if path.contains(&neighbor) {
                    continue;
                }
                if let Some(must_not_list) = must_not {
                    if must_not_list.contains(&neighbor) {
                        continue;
                    }
                }

                path.push(neighbor);
                count += explore(path, graph, end, must_not);
                path.pop();
            }

            count
        }

        explore(&mut path, graph, to, must_not)
    }

    fn count_you_to_out(graph: &HashMap<&[u8], Vec<&[u8]>>) -> i64 {
        Self::count_paths(b"you", b"out", graph, None)
    }

    fn count_srv_to_out_over_dac_and_fft(graph: &HashMap<&[u8], Vec<&[u8]>>) -> i64 {
        [(b"svr", b"fft"), (b"fft", b"dac"), (b"dac", b"out")]
            .par_iter()
            .map(|&(from, to)| Self::count_paths_adj(from, to, graph))
            .product()
    }

    fn get_exmaple_part_2(&self) -> &'static str {
        "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
    }
}

impl Solution for Day11 {
    fn number(&self) -> u8 {
        11
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let graph = Self::parse_input(input);
        let result = Self::count_you_to_out(&graph);
        Ok(result)
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let graph = if get_input_mode() == PuzzleInputType::Example {
            let example_input = self.get_exmaple_part_2();
            Self::parse_input(example_input.as_bytes())
        } else {
            Self::parse_input(input)
        };
        let result = Self::count_srv_to_out_over_dac_and_fft(&graph);
        Ok(result)
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day11)
}

#[cfg(test)]
mod test {
    use crate::day::set_input_mode;

    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_example() {
        let day = Day11;
        set_input_mode(PuzzleInputType::Example);
        let result = day.run_part_2(day.get_exmaple_part_2().as_bytes()).unwrap();
        assert_eq!(result, 2);
    }
}
