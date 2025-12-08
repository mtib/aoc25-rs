use std::collections::{HashMap, HashSet};

use crate::day::Solution;

use super::Day;

struct Day07;

struct Particle {
    position: usize,
    multitude: i64,
}

impl Solution for Day07 {
    fn number(&self) -> u8 {
        7
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let mut energized = HashSet::new();

        let map = input.split(|&c| c == b'\n').collect::<Box<_>>();

        for (index, &char) in map[0].iter().enumerate() {
            if char == b'S' {
                energized.insert(index);
            }
        }

        let mut split = 0;
        map.iter().skip(2).for_each(|&row| {
            let mut next_energized = HashSet::new();
            for (index, &char) in row.iter().enumerate() {
                if char == b'^' && energized.contains(&index) {
                    energized.remove(&index);
                    next_energized.insert(index - 1);
                    next_energized.insert(index + 1);
                    split += 1;
                }
            }
            energized = next_energized.union(&energized).copied().collect();
        });

        Ok(split)
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let map = input
            .trim_ascii_end()
            .split(|&c| c == b'\n')
            .collect::<Box<_>>();
        let mut particles: Vec<Particle> = Vec::new();

        for (index, &char) in map[0].iter().enumerate() {
            if char == b'S' {
                particles.push(Particle {
                    position: index,
                    multitude: 1,
                });
            }
        }

        for &row in map.iter().skip(2) {
            let mut next_particles: HashMap<usize, i64> = HashMap::new();
            for particle in particles.into_iter() {
                if row[particle.position] == b'^' {
                    for delta in [-1, 1] {
                        let new_index = (particle.position as isize + delta) as usize;
                        *next_particles.entry(new_index).or_insert(0) += particle.multitude;
                    }
                } else {
                    *next_particles.entry(particle.position).or_insert(0) += particle.multitude;
                }
            }
            particles = next_particles
                .into_iter()
                .map(|(position, multitude)| Particle {
                    position,
                    multitude,
                })
                .collect();
        }

        Ok(particles.iter().map(|p| p.multitude).sum())
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day07)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 40);
    }
}
