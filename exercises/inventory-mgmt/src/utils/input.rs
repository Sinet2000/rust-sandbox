use std::io::{self, Write};

pub fn read_u32(message: &str) -> u32 {
    loop {
        print!("{message}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

pub fn read_string(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if !input.is_empty() {
            return input.to_string();
        }

        println!("Input cannot be empty.\n");
    }
}

pub fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.replace(',', ".").parse::<f64>() {
            Ok(value) if value >= 0.0 => return value,
            Ok(_) => println!("Value cannot be negative.\n"),
            Err(_) => println!("Please enter a valid number.\n"),
        }
    }
}

pub fn pause() {
    println!("\nPress Enter to continue...");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
