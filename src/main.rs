use std::env;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Validate the input arguments
    let (c_program_path, c_program_args) = validate_args(&args);

    // Prompt user for the number of executions
    let num_executions = get_execution_count();

    // Measure and store the execution durations
    let durations = execute_c_program(c_program_path, c_program_args, num_executions);

    // Calculate and display the average execution time
    display_average_execution_time(&durations);
}

/// Validates command line arguments and returns the C program path and its arguments.
///
/// # Arguments
///
/// * `args` - A reference to the vector of command line arguments.
///
/// # Returns
///
/// A tuple containing the C program path and its arguments.
fn validate_args(args: &[String]) -> (&String, &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_c_program> [args...]", args[0]);
        std::process::exit(1);
    }
    (&args[1], &args[2..])
}

/// Prompts the user for the number of times to execute the C program and validates the input.
///
/// # Returns
///
/// The number of times to execute the C program as a positive integer.
fn get_execution_count() -> usize {
    println!("Enter the number of times to execute the C program:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input into a positive integer
    match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Please enter a positive integer.");
            std::process::exit(1);
        }
    }
}

/// Executes the C program the specified number of times and collects the durations of each execution.
///
/// # Arguments
///
/// * `c_program_path` - Path to the C program to execute.
/// * `c_program_args` - Additional arguments to pass to the C program.
/// * `num_executions` - Number of times to execute the C program.
///
/// # Returns
///
/// A vector containing the duration of each execution.
fn execute_c_program(
    c_program_path: &String,
    c_program_args: &[String],
    num_executions: usize,
) -> Vec<Duration> {
    let mut durations: Vec<Duration> = Vec::with_capacity(num_executions);

    for i in 0..num_executions {
        let start = Instant::now();

        let output = Command::new(c_program_path)
            .args(c_program_args)
            .output()
            .expect("Failed to execute C program");

        let duration = start.elapsed();
        durations.push(duration);

        // Check if the execution was successful and print the result
        if output.status.success() {
            println!("C program executed successfully.");
            println!("Execution number: {}", i + 1);
            println!("Duration: {:.4} seconds", duration.as_secs_f32());
        } else {
            println!("C program failed to execute.");
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error: {}", stderr);
        }
    }

    durations
}

/// Calculates and displays the average execution time from the collected durations.
///
/// # Arguments
///
/// * `durations` - A reference to a vector of durations from the executed programs.
fn display_average_execution_time(durations: &[Duration]) {
    let total_duration: Duration = durations.iter().sum();
    let mean_duration = total_duration / durations.len() as u32;

    println!("\n\nAverage execution time: {:?}", mean_duration);
}
