use std::env;
use std::io;
use std::process::Command;
use std::time::Instant;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that the user has provided the path and at least one argument for the C program
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_c_program> [args...]", args[0]);
        std::process::exit(1);
    }

    // The path to the C executable
    let c_program_path = &args[1];

    // The remaining arguments are for the C program
    let c_program_args = &args[2..];

    // Prompt the user for the number of executions
    println!("Enter the number of times to execute the C program:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input to a number
    let num_executions: usize = match input.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Please enter a positive integer.");
            std::process::exit(1);
        }
    };

    let mut total_duration = std::time::Duration::new(0, 0);

    for i in 0..num_executions {
        // Start the timer for each execution
        let start = Instant::now();

        // Execute the C program with the provided arguments
        let output = Command::new(c_program_path)
            .args(c_program_args) // Pass the additional arguments
            .output()
            .expect("Failed to execute C program");

        // Stop the timer
        let duration = start.elapsed();
        total_duration += duration;

        // Check if the C program executed successfully
        if output.status.success() {
            println!("C program executed successfully.");
            println!("Execution number: {}", i + 1);
        } else {
            println!("C program failed to execute.");
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error: {}", stderr);
        }
    }

    let mean_duration = total_duration / num_executions as u32;
    println!("Total execution time: {:?}", total_duration);
    println!("Average execution time: {:?}", mean_duration);
}
