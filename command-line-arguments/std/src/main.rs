use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default values
    let mut verbose = false;
    let mut jobs = None;
    let mut name = String::from("myname");

    // Parse command-line arguments
    for i in 1..args.len() {
        match args[i].as_str() {
            "-v" => verbose = true,
            "--jobs" => {
                if let Some(value) = args.get(i + 1) {
                    if let Ok(num) = value.parse::<u32>() {
                        jobs = Some(num);
                    }
                }
            }
            "--name" => {
                if let Some(value) = args.get(i + 1) {
                    name = value.clone();
                }
            }
            _ => {}
        }
    }

    // Print the parsed values
    println!("Verbose: {}", verbose);
    if let Some(jobs) = jobs {
        println!("Jobs: {}", jobs);
    }
    println!("Name: {}", name);
}
