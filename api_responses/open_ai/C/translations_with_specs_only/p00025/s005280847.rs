use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly 8 integers (4 for each set)
        if numbers.len() != 8 {
            continue; // According to the specification, we assume valid input, so we skip invalid lines.
        }

        let (set1, set2) = numbers.split_at(4);
        let (hits, blows) = count_hits_and_blows(set1, set2);

        println!("{} {}", hits, blows);
    }
}

fn count_hits_and_blows(set1: &[i32], set2: &[i32]) -> (usize, usize) {
    let mut hits = 0;
    let mut blows = 0;
    let mut set2_used = vec![false; 4]; // Track used elements in set2 for blow counting

    // Count hits
    for i in 0..4 {
        if set1[i] == set2[i] {
            hits += 1;
            set2_used[i] = true; // Mark this position as used
        }
    }

    // Count blows
    for i in 0..4 {
        if !set2_used[i] { // Only check unused elements in set2
            for j in 0..4 {
                if set1[i] == set2[j] && !set2_used[j] {
                    blows += 1;
                    set2_used[j] = true; // Mark this position as used
                    break; // Move to the next element in set1
                }
            }
        }
    }

    (hits, blows)
}
