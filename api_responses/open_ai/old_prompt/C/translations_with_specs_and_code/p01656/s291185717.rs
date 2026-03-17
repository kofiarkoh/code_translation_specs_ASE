use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read N and Q from input
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let q: i32 = iter.next().unwrap().parse().expect("Invalid input for Q");

    // Initialize names array
    let mut names: Vec<String> = Vec::with_capacity(n);
    names.push("kogakubu10gokan".to_string()); // Initial name

    // Read years and names
    let mut answer = &names[0]; // Start with the first name
    for _ in 1..n {
        let mut input = String::new();
        handle.read_line(&mut input).expect("Failed to read line");
        let mut parts = input.split_whitespace();
        let year: i32 = parts.next().unwrap().parse().expect("Invalid input for year");
        let name = parts.next().unwrap().to_string();
        names.push(name);

        // Update answer if the year is less than or equal to Q
        if year <= q {
            answer = names.last().unwrap(); // Update answer to the latest valid name
        }
    }

    // Output the answer
    println!("{}", answer);
}
