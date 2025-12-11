use std::{
    collections::{HashMap, HashSet},
    str::from_utf8_unchecked,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    day::{Solution, get_input_mode},
    example_println,
    util::input::PuzzleInputType,
};

use super::Day;

struct Day11;

static PART_2_PATHS: &'static [&[(&[u8], &[u8], Option<&[&[u8]]>)]] = &[
    &[
        (b"svr", b"dac", None),
        (b"dac", b"fft", Some(&[b"svr"])),
        (b"fft", b"out", Some(&[b"dac", b"svr"])),
    ],
    &[
        (b"svr", b"fft", None),
        (b"fft", b"dac", Some(&[b"svr"])),
        (b"dac", b"out", Some(&[b"fft", b"svr"])),
    ],
];

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

    fn can_reach<'a, 'b>(
        from: &'a [u8],
        to: &'a [u8],
        graph: &'a HashMap<&[u8], Vec<&[u8]>>,
        path: &'b mut Vec<&'a [u8]>,
        avoid: &HashSet<&'a [u8]>,
    ) -> bool {
        if from == to {
            return true;
        }
        if avoid.contains(&from) {
            return false;
        }
        path.push(from);

        let neighbors = match graph.get(from) {
            Some(n) => n,
            None => return false,
        };

        for &neighbor in neighbors.iter() {
            if Self::can_reach(neighbor, to, graph, path, avoid) {
                return true;
            }
        }

        path.pop();
        false
    }

    fn reachable_from<'a, 'b>(
        from: &'a [u8],
        graph: &'a HashMap<&[u8], Vec<&[u8]>>,
        path: &'b mut Vec<&'a [u8]>,
        reachable: &mut HashSet<&'a [u8]>,
    ) {
        path.push(from);

        let neighbors = match graph.get(from) {
            Some(n) => n,
            None => return,
        };

        for &neighbor in neighbors.iter() {
            if !reachable.insert(neighbor) {
                // Will not discover new children
                continue;
            }
            Self::reachable_from(neighbor, graph, path, reachable);
        }

        path.pop();
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
        PART_2_PATHS
            .par_iter()
            .filter(|path| {
                let middle = path[1];
                let mut avoid_end = HashSet::new();
                Self::reachable_from(middle.1, graph, &mut Vec::new(), &mut avoid_end);
                if !Self::can_reach(middle.0, middle.1, graph, &mut Vec::new(), &avoid_end) {
                    unsafe {
                        example_println!(
                            "Cannot reach {:?} to {:?}",
                            from_utf8_unchecked(middle.0),
                            from_utf8_unchecked(middle.1)
                        );
                    }
                    return false;
                } else {
                    unsafe {
                        example_println!(
                            "Can reach {:?} to {:?}",
                            from_utf8_unchecked(middle.0),
                            from_utf8_unchecked(middle.1)
                        );
                    }
                }
                true
            })
            .map(|path| {
                path.par_iter()
                    .map(|&(from, to, must_not)| {
                        let mut reachable = HashSet::new();
                        Self::reachable_from(to, graph, &mut Vec::new(), &mut reachable);
                        if let Some(must_not) = must_not {
                            must_not.iter().for_each(|&negative_node| {
                                reachable.insert(negative_node);
                            });
                        }
                        Self::count_paths(
                            from,
                            to,
                            graph,
                            Some(reachable.iter().cloned().collect::<Vec<_>>().as_slice()),
                        )
                    })
                    .product::<i64>()
            })
            .sum()
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
