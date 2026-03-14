use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of sequences
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let mut a = Vec::new();
        
        // Read ten integers for the sequence
        for _ in 0..10 {
            let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            a.push(num);
        }

        // Use a HashSet to track distinct values
        let mut distinct_values = std::collections::HashSet::new();

        for &value in &a {
            distinct_values.insert(value);
            // Early exit if we exceed two distinct values
            if distinct_values.len() > 2 {
                println!("NO");
                break;
            }
        }

        // If we did not break, we have at most two distinct values
        if distinct_values.len() <= 2 {
            println!("YES");
        }
    }
}
