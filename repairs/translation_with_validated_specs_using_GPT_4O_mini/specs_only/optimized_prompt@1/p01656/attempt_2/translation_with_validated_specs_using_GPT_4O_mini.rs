use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Changed to mutable

    // Read the number of names (N)
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Preconditions check
    if n > 50 {
        panic!("Input for N exceeds the limit of 50");
    }

    // Read the year threshold (Q)
    input.clear();
    reader.read_line(&mut input).unwrap();
    let q: i32 = input.trim().parse().expect("Invalid input for Q");

    // Preconditions check
    if q < 0 {
        panic!("Input for Q must be non-negative");
    }

    let mut names = Vec::new();
    let mut years = Vec::new();

    // Read N pairs of (year, name)
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let year: i32 = parts[0].parse().expect("Invalid year input");
        let name = parts[1].to_string();

        years.push(year);
        names.push(name);
    }

    let mut most_recent_name = names[0].clone(); // Default to the first name
    let mut most_recent_year = i32::MIN; // Initialize to the smallest possible year

    // Find the most recent name corresponding to a year <= Q
    for (i, &year) in years.iter().enumerate() {
        if year <= q && year > most_recent_year {
            most_recent_year = year;
            most_recent_name = names[i].clone();
        }
    }

    // Output the result
    println!("{}", most_recent_name);
}
