use std::env; // Importing the `env` module from the standard library.
fn main() {
    let app_name = env::args().nth(0); // Getting the first argument (program name) using `nth(0)` on the iterator returned by `env::args()`.

    let arg = env::args().nth(1); // Getting the second argument (user provided number) using `nth(1)` on the same iterator as above.
    println!("Program name: {}", app_name.unwrap_or_default());
    // Printing out the program name. If it is `None`, the `unwrap_or_default()` method returns an empty string.

    match arg {
        Some(ref arg) => println!("You passed: {}B", arg), // If `arg` has a value, print "You passed: {value}B".

        None => {
            println!("Provide number (B) of params."); // If `arg` is `None`, print an error message and exit the program with status code 1.
            std::process::exit(1);  
        }
    }

    let num: f32 = arg.expect("No argument.").parse::<f32>().unwrap_or_default();
    // Parse the string value of `arg` into a floating-point number (`f32`). If parsing fails, return a default value of 0.0.
    // The `expect()` method is used to panic with an error message if `arg` is `None`.

    let gpu_mem = (num * 4.0) / (32.0 / 16.0) * 1.2;
    // Calculate the GPU memory using the formula `(num * 4.0) / (32.0 / 16.0) * 1.2`.

    println!("GPU Memory: {} GB", gpu_mem);
    // Print out the calculated GPU memory.
}
