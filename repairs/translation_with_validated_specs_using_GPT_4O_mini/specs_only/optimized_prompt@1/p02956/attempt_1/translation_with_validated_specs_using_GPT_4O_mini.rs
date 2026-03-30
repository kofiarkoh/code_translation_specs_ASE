use std::collections::HashMap;

const MODULO: i64 = 998244353;

fn process_data_pairs(n: usize, data_pairs: &[(i64, i64)]) -> i64 {
    // Step 1: Sort the data pairs by their keys
    let mut sorted_pairs: Vec<(i64, i64)> = data_pairs.to_vec();
    sorted_pairs.sort_by_key(|pair| pair.0);

    // Step 2: Group by keys and update identifiers
    let mut grouped_identifiers: HashMap<i64, i64> = HashMap::new();
    let mut current_identifier = 0;

    for (key, _) in &sorted_pairs {
        if !grouped_identifiers.contains_key(key) {
            current_identifier += 1; // Increment identifier for new key
            grouped_identifiers.insert(*key, current_identifier);
        }
    }

    // Step 3: Calculate contributions based on updated identifiers
    let mut result: i64 = 0;

    for (key, _) in &sorted_pairs {
        if let Some(&identifier) = grouped_identifiers.get(key) {
            // Contribution is key * identifier, modulo MODULO
            result = (result + key * identifier) % MODULO;
        }
    }

    // Step 4: Return the final result
    result
}

// Example usage
fn main() {
    let n = 5; // Example number of pairs
    let data_pairs = vec![(3, 1), (1, 2), (2, 3), (3, 4), (1, 5)];
    let result = process_data_pairs(n, &data_pairs);
    println!("{}", result); // Output the result
}
