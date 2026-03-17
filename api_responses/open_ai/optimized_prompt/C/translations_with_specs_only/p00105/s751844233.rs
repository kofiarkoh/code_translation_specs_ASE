use std::collections::HashMap;
use std::io::{self, BufRead};

const MAX_STRINGS: usize = 100; // Maximum number of unique strings
const MAX_INTEGERS: usize = 1001; // Maximum number of integers per string
const MAX_STRING_LENGTH: usize = 100; // Maximum length of input strings

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    // Read input until EOF
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Ensure we have exactly 2 parts: a string and an integer
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let input_string = parts[0];
        let input_integer: i32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid integers
        };

        // Check string length constraint
        if input_string.len() > MAX_STRING_LENGTH {
            continue; // Skip strings that are too long
        }

        // Insert or update the entry in the map
        if let Some(integers) = map.get_mut(input_string) {
            // Only add the integer if we haven't exceeded the maximum
            if integers.len() < MAX_INTEGERS {
                integers.push(input_integer);
            }
        } else {
            // Create a new entry if we haven't exceeded the maximum unique strings
            if map.len() < MAX_STRINGS {
                map.insert(input_string.to_string(), vec![input_integer]);
            }
        }
    }

    // Collect and sort the unique strings
    let mut unique_strings: Vec<String> = map.keys().cloned().collect();
    unique_strings.sort();

    // Output the results
    for unique_string in unique_strings {
        if let Some(integers) = map.get(&unique_string) {
            // Print the string followed by its associated integers
            let integers_str: Vec<String> = integers.iter().map(|i| i.to_string()).collect();
            println!("{} {}", unique_string, integers_str.join(" "));
        }
    }
}
