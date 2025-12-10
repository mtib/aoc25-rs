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

static BENCHMARK_FLAGS: &[&str] = &["-b", "--benchmark"];

fn main() {
    dotenv::dotenv().ok();
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let flags = take_flags(&mut args);
    let args = args;
    let all_days = day::get_days();
    let run_targets = {
        let arg_targets = determine_run_targets(&args, &all_days);
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
        let mut benchmarker = SimpleBenchmarker::new();

        day::set_input_mode(run.input_type);
        let is_benchmarking = has_flag(&flags, BENCHMARK_FLAGS);
        if is_benchmarking {
            day::set_benchmarking(true);
        } else {
            day::set_benchmarking(false);
        }
        let start = std::time::Instant::now();
        let result = day.run(run.part, getter, &mut benchmarker);
        while is_benchmarking
            && start.elapsed().as_millis() < 2000
            && benchmarker.n() < 1000
            && result.is_ok()
        {
            day::set_benchmarking(true);
            let _ = day.run(run.part, getter, &mut benchmarker);
            day::set_benchmarking(false);
        }
        print_result(&run, result, &benchmarker, &flags);
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

fn determine_run_targets(args: &[String], available_days: &[Box<dyn Day>]) -> Vec<RunTarget> {
    let mut targets = Vec::new();
    let regex = regex::Regex::new(r"(\d+)(\.\d)?([ae])?").unwrap();
    for arg in args {
        if arg == "a" {
            for day in available_days {
                targets.extend({
                    let mut all = day.as_run_targets();
                    all.retain(|t| t.input_type == PuzzleInputType::Actual);
                    all
                });
            }
            continue;
        }
        if arg == "e" {
            for day in available_days {
                targets.extend({
                    let mut all = day.as_run_targets();
                    all.retain(|t| t.input_type == PuzzleInputType::Example);
                    all
                });
            }
            continue;
        }
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
    flags: &Vec<String>,
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
            let mut message = String::new();
            message.push_str(&identifier);
            message.push_str(" \x1b[33;1m");
            message.push_str(&if has_flag(flags, BENCHMARK_FLAGS) {
                "<hidden>".to_owned()
            } else {
                value.to_string()
            });
            message.push_str(&format!(
                "\x1b[0;37m in {:.3}ms",
                benchmarker.elapsed_ms().unwrap()
            ));
            if benchmarker.n() > 1 {
                message.push_str(&format!(" (n={})", benchmarker.n()));
            }
            println!("{}", message);
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

fn take_flags(args: &mut Vec<String>) -> Vec<String> {
    let flags = args
        .iter()
        .filter(|arg| arg.starts_with("-"))
        .cloned()
        .collect();
    args.retain(|arg| !arg.starts_with("-"));
    flags
}

fn has_flag(flags: &[String], variants: &[&str]) -> bool {
    flags.iter().any(|f| variants.contains(&f.as_str()))
}
