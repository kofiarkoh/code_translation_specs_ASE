use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().take(24).collect::<Result<Vec<_>, _>>().unwrap();

    // Parse the input into pairs of (i32, f64)
    let mut pairs: Vec<(i32, f64)> = Vec::with_capacity(24);
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let integer: i32 = parts[0].parse().unwrap();
        let double: f64 = parts[1].parse().unwrap();
        pairs.push((integer, double));
    }

    // Split pairs into three sets
    let set1 = &pairs[0..8];
    let set2 = &pairs[8..16];
    let set3 = &pairs[16..24];

    // Sort each set based on the double values
    let mut sorted_sets: Vec<Vec<(i32, f64)>> = vec![
        sort_pairs(set1),
        sort_pairs(set2),
        sort_pairs(set3),
    ];

    // Output the first two pairs from each sorted set
    for sorted_set in sorted_sets.iter() {
        for pair in &sorted_set[0..2] {
            println!("{} {:.2}", pair.0, pair.1);
        }
    }

    // Collect all pairs to find the two smallest double values across all sets
    let mut all_pairs: Vec<(i32, f64)> = sorted_sets.iter().flat_map(|s| s.iter()).cloned().collect();
    all_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Output the two pairs with the smallest double values
    for pair in &all_pairs[0..2] {
        println!("{} {:.2}", pair.0, pair.1);
    }
}

// Helper function to sort pairs based on the double values
fn sort_pairs(pairs: &[(i32, f64)]) -> Vec<(i32, f64)> {
    let mut sorted_pairs: Vec<(i32, f64)> = pairs.to_vec();
    sorted_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    sorted_pairs
}
