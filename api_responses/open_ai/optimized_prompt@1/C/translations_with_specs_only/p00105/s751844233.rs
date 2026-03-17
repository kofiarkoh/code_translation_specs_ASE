use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX_UNIQUE_STRINGS: usize = 100; // Define a maximum limit for unique strings

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut occurrences: HashMap<String, Vec<i32>> = HashMap::new();

    // Read input until EOF
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Ensure we have a string and an integer
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let string_part = parts[0].to_string();
        let int_part: i32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip if the integer is invalid
        };

        // Check if we exceed the maximum unique strings
        if occurrences.len() >= MAX_UNIQUE_STRINGS {
            break; // Stop processing if the limit is reached
        }

        // Insert or update the occurrences
        occurrences.entry(string_part).or_insert_with(Vec::new).push(int_part);
    }

    // Sort the unique strings lexicographically
    let mut unique_strings: Vec<String> = occurrences.keys().cloned().collect();
    unique_strings.sort();

    // Output the results
    for string in unique_strings {
        if let Some(integers) = occurrences.get(&string) {
            // Print the string followed by its associated integers
            let integers_str = integers.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("{} {}", string, integers_str);
        }
    }
}
