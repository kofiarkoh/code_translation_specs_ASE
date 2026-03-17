use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        // Terminate if both n and m are zero
        if n == 0 && m == 0 {
            break;
        }

        let mut e: Vec<String> = Vec::new();
        for _ in 0..n {
            let string = parts.next().unwrap().to_string();
            e.push(string);
        }

        let target: String = parts.next().unwrap().to_string();

        // Call the function to count valid concatenations
        let count = count_valid_concatenations(&e, &target);
        println!("{}", count);
    }
}

// Function to count valid concatenations of strings in e to form the target string
fn count_valid_concatenations(e: &[String], target: &str) -> usize {
    let target_len = target.len();
    let mut flag = vec![0; target_len + 1]; // Initialize flag array
    flag[0] = 1; // Base case: one way to form an empty string

    // Iterate over each position in the target string
    for i in 0..target_len {
        // Check each string in e
        for s in e {
            let s_len = s.len();
            // Check if the substring matches the end of the target string
            if i >= s_len && &target[i - s_len..i] == s {
                flag[i + 1] += flag[i - s_len + 1]; // Update the flag array
            }
        }
    }

    flag[target_len] // Return the count of valid concatenations
}
