use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for n (number of participants) and m (number of preference groups)
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Preconditions: n and m must be greater than zero
    if n == 0 || m == 0 {
        println!("Impossible");
        return;
    }

    // Initialize a vector to store participant names
    let mut names = Vec::new();
    let mut preferences: HashMap<String, Vec<String>> = HashMap::new();

    // Read participant names
    for _ in 0..n {
        let mut name = String::new();
        reader.read_line(&mut name).unwrap();
        let name = name.trim().to_string(); // Remove newline and trim spaces
        names.push(name.clone());
        preferences.insert(name, Vec::new());
    }

    // Read preferences for each participant
    for name in names.iter() {
        let mut pref_line = String::new();
        reader.read_line(&mut pref_line).unwrap();
        let prefs: Vec<String> = pref_line.trim().split_whitespace().map(|s| s.to_string()).collect();
        if let Some(prefs_vec) = preferences.get_mut(name) {
            *prefs_vec = prefs;
        }
    }

    // Process preferences to determine maximum groups
    let mut groups = Vec::new();
    let mut used = vec![false; n];

    for i in 0..n {
        if used[i] {
            continue; // Skip already used participants
        }
        let mut group = vec![names[i].clone()];
        used[i] = true;

        // Attempt to form a group based on preferences
        for pref in &preferences[&names[i]] {
            if let Some(pos) = names.iter().position(|x| x == pref) {
                if !used[pos] {
                    group.push(names[pos].clone());
                    used[pos] = true;
                }
            }
        }
        groups.push(group);
    }

    // Output the results
    if groups.is_empty() {
        println!("Impossible");
    } else {
        println!("{}", groups.len());
        for group in groups {
            println!("{}", group.join(" "));
        }
    }
}
