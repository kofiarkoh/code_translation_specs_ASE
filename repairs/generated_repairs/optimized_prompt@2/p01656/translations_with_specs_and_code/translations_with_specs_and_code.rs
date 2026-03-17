use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Changed to mutable

    // Read N and Q
    let mut first_line = String::new();
    handle.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: i32 = parts.next().unwrap().parse().unwrap();

    // Initialize names array with a default name
    let mut names: Vec<String> = vec!["kogakubu10gokan".to_string()];
    let mut answer = names[0].clone();

    // Read N pairs of (year, name)
    for _ in 0..n {
        let mut line = String::new();
        handle.read_line(&mut line).unwrap();
        let mut name_parts = line.split_whitespace();
        let year: i32 = name_parts.next().unwrap().parse().unwrap();
        let name = name_parts.next().unwrap().to_string();

        // Update the answer if the year is less than or equal to Q
        if year <= q {
            answer = name.clone();
        }
        names.push(name);
    }

    // Output the result
    println!("{}", answer);
}
