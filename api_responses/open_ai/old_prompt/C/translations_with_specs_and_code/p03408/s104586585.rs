use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of strings in the first set
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Read the strings in the first set
    let mut s = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        s.push(input.trim().to_string());
    }

    // Read the number of strings in the second set
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().expect("Invalid input for M");

    // Read the strings in the second set
    let mut t = HashSet::new();
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        t.insert(input.trim().to_string());
    }

    // Calculate the maximum adjusted frequency
    let mut max_frequency = 0;

    for string in &s {
        let frequency = s.iter().filter(|&&ref s_str| s_str == *string).count() as i32;
        let adjusted_frequency = frequency - if t.contains(string) { 1 } else { 0 };

        if adjusted_frequency > max_frequency {
            max_frequency = adjusted_frequency;
        }
    }

    // Output the result
    println!("{}", max_frequency);
}
