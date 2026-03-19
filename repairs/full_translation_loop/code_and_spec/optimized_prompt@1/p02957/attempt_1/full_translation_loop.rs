use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as long integers
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.len() != 2 {
        // If there are not exactly two inputs, we cannot proceed (though not specified in the spec)
        println!("IMPOSSIBLE");
        return;
    }

    let a: i64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("IMPOSSIBLE");
            return;
        }
    };

    let b: i64 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("IMPOSSIBLE");
            return;
        }
    };

    // Check if the sum is even
    if (a + b) % 2 != 0 {
        println!("IMPOSSIBLE");
    } else {
        // Calculate and print the average
        println!("{}", (a + b) / 2);
    }
}
