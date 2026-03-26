use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable

    // Read the first line containing N and Q
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: i32 = parts.next().unwrap().parse().unwrap();

    // Initialize variables to store names and years
    let mut names = Vec::with_capacity(n);
    let mut years = Vec::with_capacity(n);

    // Read the next N lines containing year and name pairs
    for _ in 0..n {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut name_parts = line.split_whitespace();
        let year: i32 = name_parts.next().unwrap().parse().unwrap();
        let name: &str = name_parts.next().unwrap();

        years.push(year);
        names.push(name.to_string());
    }

    // Variable to hold the answer
    let mut answer = &names[0]; // Default to the first name

    // Find the most recent name with a year <= Q
    for i in 0..n {
        if years[i] <= q {
            answer = &names[i]; // Update answer if the year is valid
        }
    }

    // Output the result
    println!("{}", answer);
}
