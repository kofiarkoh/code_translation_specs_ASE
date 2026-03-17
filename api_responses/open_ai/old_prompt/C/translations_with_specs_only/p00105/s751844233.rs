use std::collections::HashMap;
use std::io::{self, BufRead};

const WDMAX: usize = 31; // Maximum string length
const PGMAX: usize = 1001; // Maximum page number
const MAX: usize = 100; // Maximum unique strings

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut idx: HashMap<String, Vec<usize>> = HashMap::new();

    // Read input until EOF
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Ensure we have a string and a page number
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        let string = parts[0];
        let page: usize = match parts[1].parse() {
            Ok(num) if num < PGMAX => num,
            _ => continue, // Skip invalid page numbers
        };

        // Check string length constraint
        if string.len() > WDMAX {
            continue; // Skip strings that are too long
        }

        // Insert or update the entry in the HashMap
        idx.entry(string.to_string())
            .or_insert_with(Vec::new)
            .push(page);
    }

    // Collect and sort the unique strings
    let mut unique_strings: Vec<String> = idx.keys().cloned().collect();
    unique_strings.sort(); // Sort alphabetically

    // Output the results
    for string in unique_strings {
        if let Some(pages) = idx.get(&string) {
            // Print the string followed by its associated page numbers
            print!("{}", string);
            for page in pages {
                print!(" {}", page);
            }
            println!(); // New line after each entry
        }
    }
}
