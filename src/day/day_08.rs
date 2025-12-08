use std::collections::{HashMap, HashSet};

use crate::{
    day::{Solution, get_input_mode},
    util::{input::PuzzleInputType, number::parse_u8_slice_to_i64},
};

use super::Day;

struct Day08;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Day08 {
    fn binary_search(&self, target: &Point, points: &[Point], axis: char) -> usize {
        let mut low = 0;
        let mut high = points.len();

        while low < high {
            let mid = (low + high) / 2;
            let value = match axis {
                'x' => points[mid].x,
                'y' => points[mid].y,
                'z' => points[mid].z,
                _ => panic!("Invalid axis"),
            };

            if value
                < match axis {
                    'x' => target.x,
                    'y' => target.y,
                    'z' => target.z,
                    _ => panic!("Invalid axis"),
                }
            {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }

    fn find_closest(
        &self,
        a: &Point,
        px: &[Point],
        py: &[Point],
        pz: &[Point],
        min_dist: i64,
    ) -> Point {
        let mut closest = None;
        let mut min_distance = i64::MAX;

        for axis in ['x', 'y', 'z'] {
            let points = match axis {
                'x' => px,
                'y' => py,
                'z' => pz,
                _ => unreachable!(),
            };

            let approximate_index = self.binary_search(a, points, axis);
            let mut to_consider = (points.len() / 10).max(6) as i64;
            let mut di: i64 = 0;

            while to_consider > 0 {
                for m in [di, -di] {
                    let neighbor_index = (approximate_index as i64) + m;
                    if neighbor_index < 0 || neighbor_index >= points.len() as i64 {
                        to_consider -= 1;
                        continue;
                    }
                    let neighbor = &points[neighbor_index as usize];
                    if neighbor == a {
                        continue;
                    }
                    let distance = a.dist(neighbor);
                    if distance <= min_dist {
                        continue;
                    }
                    to_consider -= 1;
                    if distance < min_distance {
                        min_distance = distance;
                        closest = Some(neighbor.clone());
                    }
                }
                di += 1;
            }
        }

        closest.unwrap()
    }
}

impl Solution for Day08 {
    fn number(&self) -> u8 {
        8
    }

    fn run_part_1(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let is_example = get_input_mode() == PuzzleInputType::Example;

        let input: Vec<Point> = input
            .trim_ascii()
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut coords = line
                    .split(|&c| c == b',')
                    .map(|num| parse_u8_slice_to_i64(num));
                Point {
                    x: coords.next().unwrap(),
                    y: coords.next().unwrap(),
                    z: coords.next().unwrap(),
                }
            })
            .collect();

        let (px, py, pz) = {
            let mut px = input.to_vec();
            let mut py = input.to_vec();
            let mut pz = input.to_vec();

            px.sort_by_key(|p| p.x);
            py.sort_by_key(|p| p.y);
            pz.sort_by_key(|p| p.z);

            (px, py, pz)
        };

        let mut component: HashMap<Point, usize> = HashMap::new();
        let mut min_dist = 0;

        for _ in 0..(if is_example { 10 } else { 1000 }) {
            let point_a = input
                .iter()
                .min_by_key(|&p| p.dist(&self.find_closest(p, &px, &py, &pz, min_dist)))
                .unwrap();
            let point_b = self.find_closest(point_a, &px, &py, &pz, min_dist);
            min_dist = point_a.dist(&point_b);

            let component_id_a = component.get(&point_a);
            let component_id_b = component.get(&point_b);

            match (component_id_a, component_id_b) {
                (Some(&id_a), Some(&id_b)) => {
                    if id_a == id_b {
                        continue;
                    }
                    let new_id = id_a.min(id_b);
                    let old_id = id_a.max(id_b);

                    for (_point, comp_id) in component.iter_mut() {
                        if *comp_id == old_id {
                            *comp_id = new_id;
                        }
                    }
                }
                (Some(&id), None) => {
                    component.insert(point_b, id);
                }
                (None, Some(&id)) => {
                    component.insert(point_a.clone(), id);
                }
                (None, None) => {
                    let new_id = component.values().max().unwrap_or(&0) + 1;
                    component.insert(point_a.clone(), new_id);
                    component.insert(point_b, new_id);
                }
            }
        }

        let mut by_component: HashMap<usize, Vec<&Point>> = HashMap::new();
        for (point, &comp_id) in component.iter() {
            by_component.entry(comp_id).or_default().push(point);
        }

        let mut component_sizes: Vec<i64> =
            by_component.iter().map(|(_, v)| v.len() as i64).collect();
        component_sizes.sort();

        Ok(component_sizes
            .into_iter()
            .rev()
            .take(3)
            .reduce(|acc, next| acc * next)
            .unwrap())
    }

    fn run_part_2(&self, input: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
        let input: Vec<Point> = input
            .trim_ascii()
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut coords = line
                    .split(|&c| c == b',')
                    .map(|num| parse_u8_slice_to_i64(num));
                Point {
                    x: coords.next().unwrap(),
                    y: coords.next().unwrap(),
                    z: coords.next().unwrap(),
                }
            })
            .collect();

        let (px, py, pz) = {
            let mut px = input.to_vec();
            let mut py = input.to_vec();
            let mut pz = input.to_vec();

            px.sort_by_key(|p| p.x);
            py.sort_by_key(|p| p.y);
            pz.sort_by_key(|p| p.z);

            (px, py, pz)
        };

        let mut component: HashMap<Point, usize> = HashMap::new();
        let mut min_dist = 0;

        let mut last_added_connection = None;

        while component.values().collect::<HashSet<_>>().len() != 1 || component.len() < input.len()
        {
            let point_a = input
                .iter()
                .min_by_key(|&p| p.dist(&self.find_closest(p, &px, &py, &pz, min_dist)))
                .unwrap();
            let point_b = self.find_closest(point_a, &px, &py, &pz, min_dist);
            min_dist = point_a.dist(&point_b);

            let component_id_a = component.get(&point_a);
            let component_id_b = component.get(&point_b);

            last_added_connection = Some((point_a.clone(), point_b.clone()));

            match (component_id_a, component_id_b) {
                (Some(&id_a), Some(&id_b)) => {
                    if id_a == id_b {
                        continue;
                    }
                    let new_id = id_a.min(id_b);
                    let old_id = id_a.max(id_b);

                    for (_point, comp_id) in component.iter_mut() {
                        if *comp_id == old_id {
                            *comp_id = new_id;
                        }
                    }
                }
                (Some(&id), None) => {
                    component.insert(point_b, id);
                }
                (None, Some(&id)) => {
                    component.insert(point_a.clone(), id);
                }
                (None, None) => {
                    let new_id = component.values().max().unwrap_or(&0) + 1;
                    component.insert(point_a.clone(), new_id);
                    component.insert(point_b, new_id);
                }
            }
        }

        Ok(last_added_connection.map(|(a, b)| a.x * b.x).unwrap())
    }

    fn get_example(&self) -> Option<&str> {
        Some(
            r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#,
        )
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(Day08)
}

#[cfg(test)]
mod test {
    use crate::day::set_input_mode;

    use super::*;

    #[test]
    fn part_1_example() {
        set_input_mode(PuzzleInputType::Example);
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, 25272);
    }
}
