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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => match self.y.cmp(&other.y) {
                std::cmp::Ordering::Equal => self.z.cmp(&other.z),
                other => other,
            },
            other => other,
        }
    }
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

struct Connection {
    point_a: Point,
    point_b: Point,
}

impl Connection {
    fn dist(&self) -> i64 {
        self.point_a.dist(&self.point_b)
    }

    fn new(point_a: &Point, point_b: &Point) -> Self {
        debug_assert!(point_a <= point_b);
        Connection {
            point_a: point_a.clone(),
            point_b: point_b.clone(),
        }
    }
}

impl Day08 {}

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

        let mut connections: Vec<Connection> = input
            .iter()
            .flat_map(|p1| {
                input
                    .iter()
                    .cloned()
                    .filter(move |p2| p1 < p2)
                    .map(|p2| Connection::new(p1, &p2))
            })
            .collect();
        connections.sort_by_key(|c| c.dist());

        let mut component: HashMap<&Point, usize> = HashMap::new();

        for connection in connections.iter().take(if is_example { 10 } else { 1000 }) {
            let component_id_a = component.get(&connection.point_a);
            let component_id_b = component.get(&connection.point_b);

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
                    component.insert(&connection.point_b, id);
                }
                (None, Some(&id)) => {
                    component.insert(&connection.point_a, id);
                }
                (None, None) => {
                    let new_id = component.values().max().unwrap_or(&0) + 1;
                    component.insert(&connection.point_a, new_id);
                    component.insert(&connection.point_b, new_id);
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

        let mut connections: Vec<Connection> = input
            .iter()
            .flat_map(|p1| {
                input
                    .iter()
                    .cloned()
                    .filter(move |p2| p1 < p2)
                    .map(|p2| Connection::new(p1, &p2))
            })
            .collect();
        connections.sort_by_key(|c| c.dist());

        let mut component: HashMap<&Point, usize> = HashMap::new();
        let mut last_added_connection = None;

        let mut connection_iter = connections.iter();
        while component.values().collect::<HashSet<_>>().len() != 1 || component.len() < input.len()
        {
            let connection = connection_iter.next().unwrap();
            let component_id_a = component.get(&connection.point_a);
            let component_id_b = component.get(&connection.point_b);

            last_added_connection = Some(connection);

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
                    component.insert(&connection.point_b, id);
                }
                (None, Some(&id)) => {
                    component.insert(&connection.point_a, id);
                }
                (None, None) => {
                    let new_id = component.values().max().unwrap_or(&0) + 1;
                    component.insert(&connection.point_a, new_id);
                    component.insert(&connection.point_b, new_id);
                }
            }
        }

        Ok(last_added_connection
            .map(|c| c.point_a.x * c.point_b.x)
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
