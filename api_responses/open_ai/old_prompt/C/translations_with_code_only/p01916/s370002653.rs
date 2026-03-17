use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    let mut a = [0; 30]; // Array to track occurrences of characters
    let mut sum = 0;

    // Iterate over each character in the input string
    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            let index = (ch as usize) - ('a' as usize);
            a[index] = 1 - a[index]; // Toggle the value (0 to 1 or 1 to 0)
        }
    }

    // Sum the toggled values
    for &value in &a {
        sum += value;
    }

    // Print the result divided by 2
    println!("{}", sum / 2);
}
