use std::io::{self, BufRead};

fn main() {
    let mut c = [0; 16];
    let mut p = [0; 8];

    // Initialize the constants array `c` as per the C code
    for i in 0..16 {
        if i % 2 == 0 {
            c[i] = 1;
        }
    }

    c[1] = 2;
    c[3] = 2;
    c[5] = 4;
    c[7] = 4;
    c[9] = 2;
    c[11] = 2;
    c[13] = 4;
    c[15] = 4;

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if we have exactly 8 integers
        if nums.len() != 8 {
            break; // Terminate input processing if fewer than 8 integers
        }

        p.copy_from_slice(&nums[0..8]);

        let mut min_s = calculate_score(&p, &c, 0);
        let mut min_p = 0;

        for i in 1..=8 {
            let cur_s = calculate_score(&p, &c, i);
            if cur_s < min_s {
                min_s = cur_s;
                min_p = i;
            }
        }

        // Output the optimal configuration
        let output: Vec<String> = c[min_p..min_p + 8].iter().map(|&x| x.to_string()).collect();
        println!("{}", output.join(" "));
    }
}

// Function to calculate the score based on the input and constants
fn calculate_score(p: &[i32; 8], c: &[i32; 16], offset: usize) -> i32 {
    let mut score = 0;
    for j in 0..8 {
        if p[j] - c[offset + j] > 0 {
            score += p[j] - c[offset + j];
        }
    }
    score
}
