// Import the `env` module from the Rust standard library.
// The `env` module provides functions for accessing and manipulating environment variables.
use std::env;

fn main() {
    // Get the first argument passed to the program (the command name).
    let app_name = env::args().nth(0);

    // Get the second argument passed to the program (the user-provided parameter).
    let arg = env::args().nth(1);

    // Print the name of the application.
    // The `unwrap_or_default` method is used in case there's no first command-line argument,
    // and it will return an empty string instead of panicking.
    println!("Program name: {}", app_name.unwrap_or_default());

    // Use a match statement to handle the second argument passed to the program.
    match arg {
        // If the user provided a parameter, print a message indicating that.
        Some(ref arg) => println!("You passed: {}B", arg),


        // If there's no parameter provided, exit the process with an error code.
        None => {
            // Print an error message asking the user to provide a number of parameters.
            println!("Provide number (B) of params.");

            // Use the `exit` function from the `std::process` module to terminate the program
            // immediately, returning an exit status of 1 to indicate an error condition.
            std::process::exit(1);  
        }
    }

    // Attempt to parse the user-provided parameter as a floating-point number (f32).
    let num: f32 = arg.expect("No argument.").parse::<f32>().unwrap_or_default();

    // Calculate the amount of GPU memory required based on the provided parameter.
    // The formula is: `(num * 4.0) / (32.0 / 16.0) * 1.2`
    let gpu_mem = num * 4.0 / (32.0 / 16.0) * 1.2;

    // Print the calculated GPU memory amount in gigabytes.
    println!("GPU Memory: {} GB", gpu_mem);
}
