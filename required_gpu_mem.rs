use std::env;

fn main() {
    let app_name = env::args().nth(0);

    let arg = env::args().nth(1);

    println!("Program name: {}", app_name.unwrap_or_default());

    match arg {
        Some(ref arg) => println!("You passed: {}B", arg),

        None => { println!("Provide number (B) of params."); 
            std::process::exit(1);  
        }
    }

    let num: f32 = arg.expect("No argument.").parse::<f32>().unwrap_or_default();

    let gpu_mem = (num * 4.0) / (32.0 / 16.0) * 1.2;

    println!("GPU Memory: {} GB", gpu_mem);
}
