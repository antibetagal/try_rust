// This line tells Rust that we're using a library called `std::env`.
use std::env;

fn main() {
    // This line gets the first argument passed to our program, which represents its name.
    let app_name = env::args().nth(0);

    // This line gets the second argument passed to our program. In Rust, arguments are zero-indexed,
    // so `nth(1)` gets the second argument.
    let arg = env::args().nth(1);

    // This line prints out the name of our program using `app_name`. The `.unwrap_or_default()` method
    // is used to handle the case where `app_name` is `None`, in which case it returns an empty string.
    println!("Program name: {}", app_name.unwrap_or_default());

    // This match statement checks whether we have any arguments passed to our program. If we do, it's
    // stored in the `arg` variable. The `Some(ref arg)` arm of the match is used to pattern-match on
    // the value of `arg`. The `ref` keyword is used to indicate that we want to borrow the value,
    // rather than taking ownership of it.
    match arg {
        // If `arg` is `Some`, this code will run. It prints out a message indicating that we passed an argument,
        // and includes the value of the argument in GB (using `B` as a unit suffix).
        Some(ref arg) => println!("You passed: {}B", arg),

        // If `arg` is `None`, this arm of the match will run. It prints out an error message
        // indicating that we need to pass a number, and then exits our program using
        // `std::process::exit(1)`. The `(1)` at the end indicates that we're exiting with a status code
        // of 1, which typically means "error".
        None => { println!("Provide number (B) of params."); 
            std::process::exit(1);  
        }
    }

    // This line attempts to parse the value of `arg` into a floating-point number. The `.expect("No argument.")`
    // method is used to handle any errors that might occur during parsing, and will exit our program with an error
    // message if parsing fails.
    let num: f32 = arg.expect("No argument.").parse::<f32>().unwrap_or_default();

    // This line calculates the amount of GPU memory we would have available for a given number of parameters.
    // The calculation is based on the formula `(num * 4.0) / (32.0 / 16.0) * 1.2`. It's not entirely clear
    // what this formula represents, but it appears to be calculating some kind of memory allocation based on
    // the number of parameters.
    let gpu_mem = (num * 4.0) / (32.0 / 16.0) * 1.2;

    // This line prints out the result of our calculation in GB (using `GB` as a unit suffix).
    println!("GPU Memory: {} GB", gpu_mem);
}
