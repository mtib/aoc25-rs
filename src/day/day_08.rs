use std::collections::HashMap;

use crate::{day::Solution, example_println, util::number::parse_u8_slice_to_i64};

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

    fn find_closest(&self, a: &Point, px: &[Point], py: &[Point], pz: &[Point]) -> Point {
        let mut closest = None;
        let mut min_distance = i64::MAX;

        for axis in ['x', 'y', 'z'] {
            let points = match axis {
                'x' => px,
                'y' => py,
                'z' => pz,
                _ => unreachable!(),
            };

            let index = self.binary_search(a, points, axis);

            for neighbor in points
                .get((index.saturating_sub(20)).max(0)..=(index + 20).min(points.len() - 1))
                .unwrap()
            {
                if neighbor == a {
                    continue;
                }
                let distance = a.dist(neighbor);

                if distance < min_distance && distance != 0 {
                    min_distance = distance;
                    closest = Some(neighbor.clone());
                }
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

        let start = input
            .iter()
            .min_by_key(|&p| p.dist(&self.find_closest(p, &px, &py, &pz)))
            .unwrap();

        let mut component: HashMap<Point, usize> = HashMap::new();

        component.insert(start.clone(), 0);
        component.insert(self.find_closest(start, &px, &py, &pz), 0);

        example_println!("start {:?}", component);

        while component.len() < input.len() {
            // not in component
            let next_point = input
                .iter()
                .filter(|&p| !component.contains_key(p))
                .min_by_key(|&p| p.dist(&self.find_closest(p, &px, &py, &pz)))
                .unwrap();

            // maybe in component
            let closest = self.find_closest(next_point, &px, &py, &pz);

            example_println!("next pair {:?} and {:?}", next_point, closest);

            let component_id = component.get(&closest);

            if let Some(&id) = component_id {
                component.insert(next_point.clone(), id);
            } else {
                let new_id = component.values().max().unwrap() + 1;
                component.insert(next_point.clone(), new_id);
                component.insert(closest, new_id);
            }
        }

        let mut by_component: HashMap<usize, Vec<&Point>> = HashMap::new();
        for (point, &comp_id) in component.iter() {
            by_component.entry(comp_id).or_default().push(point);
        }

        for (k, v) in by_component.iter() {
            example_println!("component {} (len={}): {:?}", k, v.len(), v);
        }

        let mut component_sizes: Vec<i64> =
            by_component.iter().map(|(_, v)| v.len() as i64).collect();
        component_sizes.sort();

        example_println!("component_sizes {:?}", component_sizes);

        Ok(component_sizes
            .into_iter()
            .rev()
            .take(3)
            .reduce(|acc, next| acc * next)
            .unwrap())
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
    use super::*;

    #[test]
    fn part_1_example() {
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
        assert_eq!(result, todo!());
    }
}
