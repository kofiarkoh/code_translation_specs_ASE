use std::collections::HashMap;

fn process_invitations(test_cases: Vec<(usize, usize, Vec<String>, Vec<Vec<String>>)>) -> Vec<String> {
    let mut results = Vec::new();

    for (num_names, num_groups, names, groups) in test_cases {
        // Check preconditions
        if num_names == 0 || num_groups == 0 || names.len() != num_names || groups.len() != num_groups {
            results.push("Impossible".to_string());
            continue;
        }

        // Create a hash table to store unique names
        let mut name_map: HashMap<String, usize> = HashMap::new();
        for (i, name) in names.iter().enumerate() {
            name_map.insert(name.clone(), i);
        }

        // Check if groups can be formed
        let mut distinct_groups = Vec::new();

        for group in groups {
            let mut valid_group = true;
            let mut group_members = Vec::new();

            for member in &group { // Borrow member to match expected type
                if let Some(&id) = name_map.get(member) {
                    group_members.push(member.clone());
                } else {
                    valid_group = false;
                    break;
                }
            }

            if valid_group {
                distinct_groups.push(group_members);
            }
        }

        // Determine output based on the groups formed
        if distinct_groups.is_empty() {
            results.push("Impossible".to_string());
        } else {
            let max_groups = distinct_groups.len();
            results.push(format!("{}: {:?}", max_groups, distinct_groups));
        }
    }

    results
}

fn main() {
    // Example usage
    let test_cases = vec![
        (3, 2, vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()], 
         vec![vec!["Alice".to_string(), "Bob".to_string()], 
              vec!["Charlie".to_string()]]),
    ];
    
    let results = process_invitations(test_cases);
    for result in results {
        println!("{}", result);
    }
}
