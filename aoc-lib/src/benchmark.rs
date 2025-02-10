use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

// how many samples to run
pub const SAMPLE_SIZE: usize = 1000;

const BENCH_FILE: &str = "benchmarks.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchResult {
    duration_ns: f64,
    samples: usize,
    result: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DayBenchmarks {
    part1: Option<BenchResult>,
    part2: Option<BenchResult>,
}

pub fn benchmark<F>(f: F, samples: usize) -> (String, f64)
where
    F: Fn() -> String,
{
    let mut total_duration = Duration::new(0, 0);
    let result = f();

    for i in 0..samples {
        if i % 10 == 0 {
            print!("\rRunning sample {}/{}", i + 1, samples);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        let start = Instant::now();
        f();
        total_duration += start.elapsed();
    }
    println!("\rCompleted {} samples", samples);

    let avg_duration = total_duration.as_nanos() as f64 / samples as f64;
    (result, avg_duration)
}

pub fn store_benchmark(day: u32, part: u8, result: String, duration_ns: f64) {
    let mut benchmarks: HashMap<u32, DayBenchmarks> = fs::read_to_string(BENCH_FILE)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default();

    let day_bench = benchmarks.entry(day).or_default();
    let bench_result = BenchResult {
        duration_ns,
        samples: SAMPLE_SIZE,
        result,
    };

    match part {
        1 => day_bench.part1 = Some(bench_result),
        2 => day_bench.part2 = Some(bench_result),
        _ => return,
    }

    if let Ok(json) = serde_json::to_string_pretty(&benchmarks) {
        let _ = fs::write(BENCH_FILE, json);
    }
}

pub fn format_duration(ns: f64) -> String {
    if ns < 1000.0 {
        format!("{:.1}ns", ns)
    } else if ns < 1_000_000.0 {
        format!("{:.1}µs", ns / 1000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.1}ms", ns / 1_000_000.0)
    } else {
        format!("{:.1}s", ns / 1_000_000_000.0)
    }
}

pub fn print_benchmark(_day: u32, part: u8, result: &str, duration_ns: f64) {
    let duration_str = format_duration(duration_ns);
    println!(
        "Part {}: {} ({} @ {} samples)",
        part,
        result.bright_yellow(),
        duration_str.bright_blue(),
        SAMPLE_SIZE
    );
} 