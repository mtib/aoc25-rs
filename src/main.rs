use crate::{
    day::Day,
    error::{ExampleInputNotAvailableError, PuzzleNotImplementedError},
    util::{
        benchmark::{Benchmarker, SimpleBenchmarker},
        input::{DayCookiePuzzleInputGetter, PuzzleGetter, PuzzleInputType},
    },
};

mod day;
mod error;
mod util;

fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let all_days = day::get_days();
    let run_targets = {
        let arg_targets = determine_run_targets(&args);
        if arg_targets.is_empty() {
            all_days
                .last()
                .map(|d| d.as_run_targets())
                .unwrap_or_default()
        } else {
            arg_targets
        }
    };

    if run_targets.is_empty() {
        println!("No valid days to run.");
        return;
    }

    for run in run_targets {
        let day = all_days
            .iter()
            .find(|d| d.number() == run.day)
            .expect("Can't find day");
        let cookie_getter = DayCookiePuzzleInputGetter::new(2025, run.day);
        let getter: &dyn PuzzleGetter = match run.input_type {
            PuzzleInputType::Example => day.as_ref(),
            PuzzleInputType::Actual => &cookie_getter,
        };
        let benchmarker = SimpleBenchmarker::new();

        day::set_current_mode(run.input_type);
        let result = day.run(run.part, getter, &benchmarker);
        print_result(&run, result, &benchmarker);
    }
}

struct RunTarget {
    day: u8,
    part: day::Part,
    input_type: PuzzleInputType,
}

impl dyn Day {
    fn as_run_targets(&self) -> Vec<RunTarget> {
        let mut targets = Vec::new();
        for part in [day::Part::One, day::Part::Two] {
            for input_type in [PuzzleInputType::Example, PuzzleInputType::Actual] {
                targets.push(RunTarget {
                    day: self.number(),
                    part,
                    input_type,
                });
            }
        }
        targets
    }
}

fn determine_run_targets(args: &[String]) -> Vec<RunTarget> {
    let mut targets = Vec::new();
    let regex = regex::Regex::new(r"(\d)+(\.\d)?([ae])?").unwrap();
    for arg in args {
        let result = if let Some(m) = regex.captures(&arg) {
            m
        } else {
            continue;
        };
        let day = result.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let parts = if let Some(part_match) = result.get(2) {
            match part_match.as_str() {
                ".1" => vec![day::Part::One],
                ".2" => vec![day::Part::Two],
                _ => panic!("Invalid part specifier"),
            }
        } else {
            vec![day::Part::One, day::Part::Two]
        };
        let input_types = if let Some(input_match) = result.get(3) {
            match input_match.as_str() {
                "a" => vec![PuzzleInputType::Actual],
                "e" => vec![PuzzleInputType::Example],
                _ => panic!("Invalid input type specifier"),
            }
        } else {
            vec![PuzzleInputType::Example, PuzzleInputType::Actual]
        };

        for part in parts {
            for input_type in &input_types {
                targets.push(RunTarget {
                    day,
                    part,
                    input_type: *input_type,
                });
            }
        }
    }
    targets
}

fn print_result(
    run: &RunTarget,
    result: Result<i64, Box<dyn std::error::Error>>,
    benchmarker: &dyn Benchmarker,
) {
    let identifier = format!(
        "\x1b[37m[{:2}.{}{}]\x1b[0m",
        run.day,
        run.part.to_number(),
        match run.input_type {
            PuzzleInputType::Example => "e",
            PuzzleInputType::Actual => "a",
        }
    );
    match result {
        Ok(value) => {
            println!(
                "{} \x1b[33;1m{}\x1b[0;37m in {:.3}ms\x1b[0m",
                identifier,
                value,
                benchmarker.elapsed_ms().unwrap()
            );
        }
        Err(e) => {
            match e {
                _ if e.is::<ExampleInputNotAvailableError>() => {
                    println!("{} \x1b[31m<no example available>\x1b[0m", identifier);
                    return;
                }
                _ if e.is::<PuzzleNotImplementedError>() => {
                    println!("{} \x1b[31m<not implemented>\x1b[0m", identifier);
                    return;
                }
                _ => {}
            }
            println!(
                "{} \x1b[31mError running puzzle: \x1b[1m{}\x1b[0m",
                identifier, e
            );
        }
    }
}
