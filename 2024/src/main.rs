use std::env;
use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let day = &args[1][3..];
        run_day(day)?;
    } else {
        run_all_days()?;
    }

    Ok(())
}

fn run_day(day: &str) -> Result<(), Box<dyn Error>> {
    match day {
        "1" => {
            println!("Day 1:");
            println!("Part 1: {}", day1::part1::solve()?);
        }
        "2" => {
            println!("Day 2:");
            println!("Part 1: {}", day2::part1::solve()?);
        }
        "3" => {
            println!("Day 3:");
            println!("Part 1: {}", day3::part1::solve()?);
        }
        "4" => {
            println!("Day 4:");
            println!("Part 1: {}", day4::part1::solve()?);
        }
        "5" => {
            println!("Day 5:");
            println!("Part 1: {}", day5::part1::solve()?);
        }
        "6" => {
            println!("Day 6:");
            println!("Part 1: {}", day6::part1::solve()?);
        }
        "7" => {
            println!("Day 7:");
            println!("Part 1: {}", day7::part1::solve()?);
            println!("Part 2: {}", day7::part2::solve()?);
        }
        _ => println!("Day not implemented"),
    }
    Ok(())
}

fn run_all_days() -> Result<(), Box<dyn Error>> {
    for day in 1..=7 {
        if let Err(e) = run_day(&day.to_string()) {
            eprintln!("Error running day {}: {}", day, e);
        }
        println!();
    }
    Ok(())
}
