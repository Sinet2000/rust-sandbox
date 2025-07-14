fn main() {
    println!("Hello, world!");
    let (a, b) = (1, 2);
    let num = if a > b { a } else { b };

    let msg = if num == 5 {
        "The number is five."
    } else if num == 3 {
        "The number is three."
    } else {
        "The number is something else."
    };
    println!("{}", msg);

    // even/odd
    for x in [1, 2, 3, 4, 5, 6].iter() {
        if x % 2 == 0 {
            println!("{} is even", x);
        } else {
            println!("{} is odd", x);
        };
    }

    // tuple comparisons
    let array = [(1, 2), (3, 4), (5, 6)];
    for (x, y) in array.iter() {
        if x > y {
            println!("{} is greater than {}", x, y);
        } else if x < y {
            println!("{} is less than {}", x, y);
        } else {
            println!("{} is equal to {}", x, y);
        }
    }

    // prime check up to 1000
    for x in 0..=1000 {
        if x < 2 {
            continue;
        }
        let mut is_prime = true;
        for i in 2..=((x as f64).sqrt() as u32) {
            if x % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{} is a prime number", x);
        } else {
            println!("{} is not a prime number", x);
        };
    }

    // str vs String basics
    let s: &str = "Hello, Rust!";
    println!("\nString slice: {}", s);

    // 1) length
    println!("Length: {}", s.len());

    // 2) uppercase / lowercase
    println!("Uppercase: {}", s.to_uppercase());
    println!("Lowercase: {}", s.to_lowercase());

    // 3) contains / starts_with / ends_with
    println!("Contains \"Rust\"? {}", s.contains("Rust"));
    println!("Starts with \"Hello\"? {}", s.starts_with("Hello"));
    println!("Ends with \"!\"? {}", s.ends_with('!'));

    // 4) replace
    let replaced = s.replace("Rust", "World");
    println!("Replaced: {}", replaced);

    // 5) split and iterate
    println!("Words:");
    for word in s.split_whitespace() {
        println!("  - {}", word);
    }

    // 6) trim (leading/trailing whitespace)
    let padded = "   lots of space   ";
    println!("Before trim: {:?}", padded);
    println!("After trim: {:?}", padded.trim());

    // 7) chars, reversing a string
    let rev: String = s.chars().rev().collect();
    println!("Reversed: {}", rev);

    // 8) owned String methods
    let mut owned = String::from("Hello");
    owned.push(',');
    owned.push_str(" Rustaceans!");
    println!("Owned String: {}", owned);

    // 9) formatting and interpolation
    let name = "Ferris";
    let info = format!("{} the crab says hi!", name);
    println!("{}", info);
}
