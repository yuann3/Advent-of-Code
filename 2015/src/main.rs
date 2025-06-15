use anyhow::Result;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let day = &args[1][3..];
        run_day(day)?;
    } else {
        run_all_days()?;
    }

    Ok(())
}

fn run_day(day: &str) -> Result<()> {
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
        "6" => {
            println!("Day 6:");
            println!("Part 1: {}", day6::part1::solve()?);
            println!("Part 2: {}", day6::part2::solve()?);
        }
        "7" => {
            println!("Day 7:");
            println!("Part 1: {}", day7::part1::solve()?);
            println!("Part 2: {}", day7::part2::solve()?);
        }
        "8" => {
            println!("Day 8:");
            println!("Part 1: {}", day8::part1::solve()?);
            println!("Part 2: {}", day8::part2::solve()?);
        }
        "9" => {
            println!("Day 9:");
            println!("Part 1: {}", day9::part1::solve()?);
            println!("Part 2: {}", day9::part2::solve()?);
        }
        "10" => {
            println!("Day 10:");
            println!("Part 1: {}", day10::part1::solve()?);
            println!("Part 2: {}", day10::part2::solve()?);
        }
        "11" => {
            println!("Day 11:");
            println!("Part 1: {}", day11::part1::solve()?);
            println!("Part 2: {}", day11::part2::solve()?);
        }
        "12" => {
            println!("Day 12:");
            println!("Part 1: {}", day12::part1::solve()?);
            println!("Part 2: {}", day12::part2::solve()?);
        }
        "13" => {
            println!("Day 13:");
            println!("Part 1: {}", day13::part1::solve()?);
            println!("Part 2: {}", day13::part2::solve()?);
        }
        "14" => {
            println!("Day 14:");
            println!("Part 1: {}", day14::part1::solve()?);
            println!("Part 2: {}", day14::part2::solve()?);
        }
        "15" => {
            println!("Day 15:");
            println!("Part 1: {}", day15::part1::solve()?);
            println!("Part 2: {}", day15::part2::solve()?);
        }
        "16" => {
            println!("Day 16:");
            println!("Part 1: {}", day16::part1::solve()?);
            println!("Part 2: {}", day16::part2::solve()?);
        }
        "17" => {
            println!("Day 17:");
            println!("Part 1: {}", day17::part1::solve()?);
            println!("Part 2: {}", day17::part2::solve()?);
        }
        "18" => {
            println!("Day 18:");
            println!("Part 1: {}", day18::part1::solve()?);
            println!("Part 2: {}", day18::part2::solve()?);
        }
        "19" => {
            println!("Day 19:");
            println!("Part 1: {}", day19::part1::solve()?);
            println!("Part 2: {}", day19::part2::solve()?);
        }
        "20" => {
            println!("Day 20:");
            println!("Part 1: {}", day20::part1::solve()?);
            println!("Part 2: {}", day20::part2::solve()?);
        }
        "21" => {
            println!("Day 21:");
            println!("Part 1: {}", day21::part1::solve()?);
            println!("Part 2: {}", day21::part2::solve()?);
        }
        "22" => {
            println!("Day 22:");
            println!("Part 1: {}", day22::part1::solve()?);
            println!("Part 2: {}", day22::part2::solve()?);
        }
        "23" => {
            println!("Day 23:");
            println!("Part 1: {}", day23::part1::solve()?);
            println!("Part 2: {}", day23::part2::solve()?);
        }
        "24" => {
            println!("Day 24:");
            println!("Part 1: {}", day24::part1::solve()?);
            println!("Part 2: {}", day24::part2::solve()?);
        }
        _ => println!("Day not implemented"),
    }
    Ok(())
}

fn run_all_days() -> Result<()> {
    for day in 1..=25 {
        if let Err(e) = run_day(&day.to_string()) {
            eprintln!("Error running day {}: {}", day, e);
        }
        println!();
    }
    Ok(())
}
