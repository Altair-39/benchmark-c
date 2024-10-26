use std::env;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_c_program> [args...]", args[0]);
        std::process::exit(1);
    }

    let c_program_path = &args[1];
    let c_program_args = &args[2..];

    println!("Enter the number of times to execute the C program:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num_executions: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Please enter a positive integer.");
            std::process::exit(1);
        }
    };

    let mut durations: Vec<Duration> = Vec::with_capacity(num_executions);

    for i in 0..num_executions {
        let start = Instant::now();

        let output = Command::new(c_program_path)
            .args(c_program_args)
            .output()
            .expect("Failed to execute C program");

        let duration = start.elapsed();
        durations.push(duration);

        if output.status.success() {
            println!("C program executed successfully.");
            println!("Execution number: {}", i + 1);
            println!("Duration: {}", duration.as_secs_f32());
        } else {
            println!("C program failed to execute.");
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error: {}", stderr);
        }
    }

    let total_duration: Duration = durations.iter().sum();
    let mean_duration = total_duration / num_executions as u32;

    println!("\n\nAverage execution time: {:?}", mean_duration);
}
