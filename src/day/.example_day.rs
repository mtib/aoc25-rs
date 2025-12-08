use crate::day::Solution;

use super::Day;

struct DayXX;

impl Solution for DayXX {
    fn number(&self) -> u8 {
        XX
    }

    fn get_example(&self) -> Option<&str> {
        Some(r#"XXX"#)
    }
}

pub fn day() -> Box<dyn Day> {
    Box::new(DayXX)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_1(example_input.as_bytes()).unwrap();
        assert_eq!(result, todo!());
    }

    #[test]
    fn part_2_example() {
        let day = day();
        let example_input = day.get_example().unwrap();
        let result = day.run_part_2(example_input.as_bytes()).unwrap();
        assert_eq!(result, todo!());
    }
}
