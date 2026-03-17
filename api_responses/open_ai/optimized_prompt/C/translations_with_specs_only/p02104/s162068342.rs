fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashSet;

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of identifiers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize a HashSet to store unique identifiers
    let mut identifiers = HashSet::new();

    // Read the next n lines and collect identifiers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let id: i32 = line.parse().unwrap(); // Assuming valid input as per preconditions
            identifiers.insert(id); // Store unique identifiers
        }
    }

    // Output each unique identifier on a new line
    for id in identifiers {
        println!("{}", id);
    }
}
