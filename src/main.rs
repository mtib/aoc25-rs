use crate::days::{Benchmarker, DayCookiePuzzleInputGetter, PuzzleGetter, SimpleBenchmarker};

mod days;

fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = std::env::args().skip(1).collect();

    let all_days = days::get_days();
    let run_days = if args.is_empty() {
        vec![all_days.last().unwrap()]
    } else {
        let day_nums: Vec<u8> = args
            .iter()
            .map(|a| a.parse().expect("Input day invalid"))
            .collect();
        all_days
            .iter()
            .filter(|d| day_nums.contains(&d.number()))
            .collect()
    };

    if run_days.is_empty() {
        println!("No valid days to run.");
        return;
    }

    for day in run_days {
        for part in [days::Part::One, days::Part::Two] {
            let cookie_getter = DayCookiePuzzleInputGetter::new(2025, day.number());
            let getters: Vec<&dyn PuzzleGetter> = vec![day.as_ref(), &cookie_getter];
            for puzzle_getter in getters {
                let benchmarker = SimpleBenchmarker::new();
                let result = day.run(part, puzzle_getter, &benchmarker);
                match result {
                    Ok(output) => println!(
                        "Day {} Part {} {}: {} in {}ms",
                        day.number(),
                        part,
                        puzzle_getter.get_type(),
                        output,
                        benchmarker.elapsed_ms().unwrap_or(-1.0)
                    ),
                    Err(e) => println!(
                        "Day {} Part {} {} Error: {} in {}ms",
                        day.number(),
                        part,
                        puzzle_getter.get_type(),
                        e,
                        benchmarker.elapsed_ms().unwrap_or(-1.0)
                    ),
                }
            }
        }
    }
}
