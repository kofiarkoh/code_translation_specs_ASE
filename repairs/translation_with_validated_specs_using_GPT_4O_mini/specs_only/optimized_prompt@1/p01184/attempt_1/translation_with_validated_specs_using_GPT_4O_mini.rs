use std::collections::HashSet;

fn main() {
    // Example input for testing
    let test_cases = vec![
        (3, 2, vec!["Alice", "Bob", "Charlie"], vec![vec!["Alice", "Bob"], vec!["Bob", "Charlie"]]),
        (3, 2, vec!["Alice", "Bob", "Charlie"], vec![vec!["Alice", "Bob"], vec!["Alice", "Charlie"]]),
    ];

    for (n, m, names, groups) in test_cases {
        match find_max_distinct_groups(n, m, names, groups) {
            Some(result) => {
                println!("Max distinct groups: {}", result.len());
                for group in result {
                    println!("{:?}", group);
                }
            }
            None => println!("Impossible to form groups"),
        }
    }
}

fn find_max_distinct_groups(n: usize, m: usize, names: Vec<&str>, groups: Vec<Vec<&str>>) -> Option<Vec<Vec<&str>>> {
    // Check for valid input
    if n == 0 || m == 0 || names.is_empty() || groups.is_empty() {
        return None;
    }

    let mut name_set: HashSet<&str> = HashSet::new();
    for name in &names {
        name_set.insert(name);
    }

    let mut max_groups = Vec::new();
    let mut used_names = HashSet::new();

    // Iterate through all combinations of groups
    for i in 0..(1 << m) {
        let mut current_groups = Vec::new();
        used_names.clear();
        let mut valid_combination = true;

        for j in 0..m {
            if (i & (1 << j)) != 0 { // Check if the j-th group is included
                let group = &groups[j];
                let mut group_set: HashSet<&str> = HashSet::new();
                for name in group {
                    if used_names.contains(name) || !name_set.contains(name) {
                        valid_combination = false;
                        break;
                    }
                    group_set.insert(name);
                }
                if valid_combination {
                    current_groups.push(group.clone());
                    used_names.extend(group_set);
                }
            }
            if !valid_combination {
                break;
            }
        }

        if valid_combination && current_groups.len() > max_groups.len() {
            max_groups = current_groups;
        }
    }

    if max_groups.is_empty() {
        None
    } else {
        Some(max_groups)
    }
}
