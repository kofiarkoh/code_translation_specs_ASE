fn main() {
    use std::io::{self, BufRead};
    use std::cmp::Ordering;

    const MOD: i64 = 998244353;

    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("N must be a positive integer");

    // Initialize a vector to hold the pairs
    let mut pairs: Vec<(i64, i64)> = Vec::with_capacity(n);

    // Read the pairs of integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let key: i64 = iter.next().unwrap().parse().expect("Key must be a valid integer");
        let identifier: i64 = iter.next().unwrap().parse().expect("Identifier must be a valid integer");
        pairs.push((key, identifier));
    }

    // Sort the pairs based on the keys
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Compute the result based on sorted pairs
    let mut result: i64 = 0;

    for (key, identifier) in pairs {
        // Transform the identifier based on the key
        // Here we assume a simple transformation: result += key * identifier
        // This is a placeholder for the actual transformation logic
        result = (result + key * identifier) % MOD;
    }

    // Output the result
    println!("{}", result);
}
