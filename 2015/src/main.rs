use std::env;
use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let day = &args[1][3..];
        run_day(day)?;
    } else {
        run_all_days()?;
    }

    Ok(())
}

fn run_day(day: &str) -> io::Result<()> {
    match day {
        "1" => {
            println!("Day 1:");
            println!("Part 1: {}", day1::part1::solve()?);
            println!("Part 2: {}", day1::part2::solve()?);
        }
        "2" => {
            println!("Day 2:");
            println!("Part 1: {}", day2::part1::solve()?);
            println!("Part 2: {}", day2::part2::solve()?);
        }
        "3" => {
            println!("Day 3:");
            println!("Part 1: {}", day3::part1::solve()?);
            println!("Part 2: {}", day3::part2::solve()?);
        }
        "4" => {
            println!("Day 4:");
            println!("Part 1: {}", day4::part1::solve()?);
            println!("Part 2: {}", day4::part2::solve()?);
        }
        "5" => {
            println!("Day 5:");
            println!("Part 1: {}", day5::part1::solve()?);
            println!("Part 2: {}", day5::part2::solve()?);
        }
        _ => println!("Day not implemented"),
    }
    Ok(())
}

fn run_all_days() -> io::Result<()> {
    for day in 1..=25 {
        if let Err(e) = run_day(&day.to_string()) {
            eprintln!("Error running day {}: {}", day, e);
        }
        println!();
    }
    Ok(())
}
