use std::collections::HashMap;

fn process_test_cases(test_cases: Vec<(usize, usize, Vec<String>, Vec<Vec<usize>>)>) {
    for (num_names, num_patterns, names, patterns) in test_cases {
        // Ensure the number of names and patterns are positive integers
        if num_names == 0 || num_patterns == 0 {
            println!("Impossible");
            continue;
        }

        // Create a hash table to map names to their identifiers (powers of two)
        let mut name_to_id: HashMap<String, usize> = HashMap::new();
        for i in 0..num_names {
            // Each identifier is a power of two (1 << i)
            name_to_id.insert(names[i].clone(), 1 << i);
        }

        // Process each pattern
        for pattern in patterns {
            let mut combined_id = 0;
            let mut valid = true;

            for &index in &pattern {
                // Check if the index is valid
                if index == 0 || index > num_names {
                    valid = false;
                    break;
                }
                // Combine identifiers using bitwise OR
                combined_id |= name_to_id[&names[index - 1]];
            }

            if valid {
                // Collect the names corresponding to the combined identifier
                let mut result_names: Vec<String> = Vec::new();
                for (name, id) in &name_to_id {
                    if combined_id & id != 0 {
                        result_names.push(name.clone());
                    }
                }

                // Output the result
                println!("{} {:?}", result_names.len(), result_names);
            } else {
                println!("Impossible");
            }
        }
    }
}

// Example usage
fn main() {
    let test_cases = vec![
        (3, 2, vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()], vec![vec![1, 2], vec![2, 3]]),
    ];
    process_test_cases(test_cases);
}
