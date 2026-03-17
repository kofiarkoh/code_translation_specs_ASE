use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line containing N and M
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    // Edge case: If N or M is zero, output 0
    if n == 0 || m == 0 {
        println!("0");
        return;
    }

    // Read the first set of strings
    let mut frequency_map: HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        let string = lines.next().unwrap().unwrap();
        *frequency_map.entry(string).or_insert(0) += 1;
    }

    // Read the second set of strings and store them in a HashSet for quick lookup
    let mut exclusion_set: HashMap<String, ()> = HashMap::new();
    for _ in 0..m {
        let string = lines.next().unwrap().unwrap();
        exclusion_set.insert(string, ());
    }

    // Calculate the maximum frequency of strings from the first set that are not in the second set
    let max_frequency = frequency_map.iter()
        .filter(|(key, _)| !exclusion_set.contains_key(*key))
        .map(|(_, &value)| value)
        .max()
        .unwrap_or(0); // If no valid strings, max frequency is 0

    // Output the result
    println!("{}", max_frequency);
}
