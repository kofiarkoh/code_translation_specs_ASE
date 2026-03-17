use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    i: i32, // Identifier
    u: i32, // Unique attribute
    a: i32, // Attribute A
    p: i32, // Attribute P
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut items: Vec<Item> = Vec::new();
    
    // Read input
    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let n: usize = line.trim().parse().expect("Invalid input");
        
        if n == 0 {
            break; // Exit if n is 0
        }
        
        for _ in 0..n {
            let line = handle.lines().next().expect("Failed to read line").expect("Invalid input");
            let attributes: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid input"))
                .collect();
            
            if attributes.len() == 4 {
                items.push(Item {
                    i: attributes[0],
                    u: attributes[1],
                    a: attributes[2],
                    p: attributes[3],
                });
            }
        }
    }

    // Sort items based on the criteria (simple bubble sort for O(n^2) complexity)
    let mut selected_items: Vec<i32> = Vec::new();
    let mut uu = vec![0; 1001]; // Count occurrences of unique U values

    for item in &items {
        if uu[item.u as usize] == 0 {
            selected_items.push(item.i);
            uu[item.u as usize] += 1; // Increment count for this unique U
        }
        if selected_items.len() >= 26 {
            break; // Limit to 26 unique items
        }
    }

    // Output selected item identifiers
    for id in selected_items {
        println!("{}", id);
    }
}
