use std::collections::HashMap;

fn process_integers(input_sets: Vec<(i32, i32, i32, i32)>) -> Vec<i32> {
    let mut results = Vec::new();

    for (n, s, w, q) in input_sets {
        // Preconditions check: n must be greater than zero
        if n <= 0 {
            continue; // Skip invalid input sets
        }

        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;

        // Simulate processing of n integers based on s, w, and q
        for i in 0..n {
            // Here we derive a value based on s, w, and q
            let value = s + w * i + q; // Example computation, adjust as needed

            // Update the frequency map
            let entry = frequency_map.entry(value).or_insert(0);
            *entry += 1;

            // Count specific conditions (example condition)
            if *entry > 1 { // Example condition: count if frequency is greater than 1
                count += 1;
            }
        }

        // Store the result for this set of inputs
        results.push(count);
    }

    results
}

// Example usage
fn main() {
    let input_sets = vec![(3, 1, 2, 1), (4, 2, 3, 1)];
    let output = process_integers(input_sets);
    for count in output {
        println!("{}", count);
    }
}
