use aoc_lib::benchmark::{benchmark, print_benchmark, store_benchmark, SAMPLE_SIZE};
use clap::Parser;
use colored::*;
use std::process::Command;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "cargo-time")]
struct Cli {
    /// The subcommand name (ignored)
    #[arg(hide = true)]
    _subcommand: String,

    /// The day to benchmark (e.g., day2 or just 2)
    day_arg: String,

    /// Run both parts
    #[arg(long)]
    all: bool,

    /// Store the benchmark results
    #[arg(long)]
    store: bool,
}

fn parse_day(day_arg: &str) -> Result<u32, String> {
    if day_arg.starts_with("day") {
        day_arg[3..].parse().map_err(|_| format!("Invalid day format: {}", day_arg))
    } else {
        day_arg.parse().map_err(|_| format!("Invalid day number: {}", day_arg))
    }
}

fn extract_part_result(output: &str, part: u8) -> String {
    let prefix = format!("Part {}: ", part);
    output
        .lines()
        .find(|line| line.contains(&prefix))
        .and_then(|line| line.split(&prefix).nth(1))
        .unwrap_or("Error: Could not parse output")
        .to_string()
}

fn run_solution(year: &str, day: u32, part: u8) -> String {
    let year_dir = format!("{}", year);
    let day_arg = format!("day{}", day);
    
    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", "main", &day_arg])
        .current_dir(&year_dir)
        .output();

    match cmd {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Error running day {} part {} in year {}:\n{}",
                    day,
                    part,
                    year,
                    String::from_utf8_lossy(&output.stderr)
                );
                std::process::exit(1);
            }
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            extract_part_result(&output_str, part)
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let Cli { _subcommand, day_arg, all, store } = Cli::parse();

    let day = parse_day(&day_arg).unwrap();
    let year = &_subcommand;

    println!("\n{}", format!("Day {:02}", day).bright_green().bold());
    println!("{}", "-".repeat(6).bright_black());

    let total_start = Instant::now();

    // First run to verify the solution works
    println!("Verifying solution...");
    let initial_result = run_solution(year, day, 1);
    println!("Initial run successful, result: {}", initial_result);
    println!("Starting benchmark with {} samples...", SAMPLE_SIZE);

    // Run Part 1 benchmark
    let (result, duration) = benchmark(|| run_solution(year, day, 1), SAMPLE_SIZE);
    print_benchmark(day, 1, &result, duration);
    if store {
        store_benchmark(day, 1, result, duration);
    }

    // Run Part 2 if --all is specified
    if all {
        println!("\nStarting Part 2...");
        let initial_result = run_solution(year, day, 2);
        println!("Initial Part 2 run successful, result: {}", initial_result);
        println!("Starting Part 2 benchmark with {} samples...", SAMPLE_SIZE);
        
        let (result, duration) = benchmark(|| run_solution(year, day, 2), SAMPLE_SIZE);
        print_benchmark(day, 2, &result, duration);
        if store {
            store_benchmark(day, 2, result, duration);
        }
    }

    let total_duration = total_start.elapsed();
    println!(
        "\nTotal (Run): {}\n",
        format!("{:.2?}", total_duration).bright_blue()
    );

    if store {
        println!("{}", "Stored updated benchmarks.".bright_green());
    }
} 