use std::io::{self, BufRead};

fn roman_to_integer(roman: &str) -> i32 {
    // Mapping of Roman numeral characters to their integer values
    let roman_map = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    let mut total = 0;
    let mut prev_value = 0;

    for ch in roman.chars() {
        // Get the value from the map, defaulting to 1000 for invalid characters
        let value = *roman_map.get(&ch).unwrap_or(&1000);
        
        // If the current value is greater than the previous value, we have a subtraction case
        if value > prev_value {
            total += value - 2 * prev_value; // Adjust for the previously added value
        } else {
            total += value;
        }
        
        prev_value = value;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(roman) = line {
            // Ensure the input string does not exceed 100 characters
            if roman.len() <= 100 {
                let value = roman_to_integer(&roman);
                println!("{}", value);
            }
        }
    }
}
