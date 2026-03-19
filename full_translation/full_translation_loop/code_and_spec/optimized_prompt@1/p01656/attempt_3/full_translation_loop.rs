use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable

    // Read N and Q
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: i32 = parts.next().unwrap().parse().unwrap();

    // Initialize names array with a default name
    let mut names: Vec<String> = vec!["kogakubu10gokan".to_string()];
    let mut answer = names[0].clone();

    // Read N pairs of (year, name)
    for _ in 0..n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let year: i32 = parts.next().unwrap().parse().unwrap();
        let name: String = parts.collect::<Vec<&str>>().join(" ");

        // Store the name
        names.push(name);

        // Update answer if the year is less than or equal to Q
        if year <= q {
            answer = names.last().unwrap().clone();
        }
    }

    // Output the answer
    println!("{}", answer);
}
