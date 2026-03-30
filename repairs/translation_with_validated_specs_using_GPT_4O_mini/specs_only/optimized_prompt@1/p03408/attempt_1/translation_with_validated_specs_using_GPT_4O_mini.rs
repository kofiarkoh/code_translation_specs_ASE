use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of strings in the first set
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the first set of strings and count occurrences
    let mut count_map: HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let string = input.trim().to_string();
        *count_map.entry(string).or_insert(0) += 1;
    }

    // Read the number of strings in the second set
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    // Subtract occurrences of matching strings from the second set
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let string = input.trim();
        if let Some(count) = count_map.get_mut(string) {
            if *count > 0 {
                *count = count.saturating_sub(1); // Ensure non-negative count
            }
        }
    }

    // Find the maximum adjusted count
    let max_count = count_map.values().copied().max().unwrap_or(0);

    // Output the result
    println!("{}", max_count);
}
