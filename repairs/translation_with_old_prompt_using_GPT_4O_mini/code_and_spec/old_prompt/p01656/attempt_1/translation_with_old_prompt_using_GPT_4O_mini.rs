use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N and Q
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: i32 = parts.next().unwrap().parse().unwrap();

    // Initialize names array
    let mut names: Vec<String> = vec![String::new(); n];
    let mut answer = &names[0];

    // Read years and names
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut year_name = line.split_whitespace();
        let year: i32 = year_name.next().unwrap().parse().unwrap();
        let name = year_name.next().unwrap().to_string();
        names[i] = name;

        // Update answer if the year is less than or equal to Q
        if year <= q {
            answer = &names[i];
        }
    }

    // Output the answer
    println!("{}", answer);
}
