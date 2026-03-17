use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N and Q
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: i32 = parts.next().unwrap().parse().unwrap();

    // Initialize names array with a predefined limit
    let mut names: Vec<String> = vec!["kogakubu10gokan".to_string(); 50];
    let mut answer = names[0].clone();

    // Read N pairs of (year, name)
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let year: i32 = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().to_string();
        names[i] = name;

        // Update answer if the year is less than or equal to Q
        if year <= q {
            answer = names[i].clone();
        }
    }

    // Output the answer
    println!("{}", answer);
}
