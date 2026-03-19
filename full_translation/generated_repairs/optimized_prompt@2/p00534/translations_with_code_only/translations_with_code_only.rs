use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>());
    let n = nums.next().unwrap().expect("Failed to parse n");
    let m = nums.next().unwrap().expect("Failed to parse m");

    let mut k = vec![0; (n + 1) as usize];
    let mut t = vec![0; (m + 1) as usize];
    let mut hi = vec![vec![1_000_000_000; (n + 1) as usize]; (m + 1) as usize];

    // Read k values
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        k[i as usize] = line.trim().parse::<i32>().unwrap_or_else(|_| {
            panic!("Failed to parse k value at index {}", i);
        });
    }

    // Read t values
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        t[i as usize] = line.trim().parse::<i32>().unwrap_or_else(|_| {
            panic!("Failed to parse t value at index {}", i);
        });
    }

    // Initialize hi array
    for i in 0..=m {
        hi[i as usize][0] = 0;
    }

    // Fill the hi array
    for i in 1..=m {
        for j in 1..=n {
            hi[i as usize][j as usize] = k[j as usize] * t[i as usize] + hi[(i - 1) as usize][(j - 1) as usize];
            if hi[i as usize][j as usize] > hi[(i - 1) as usize][j as usize] {
                hi[i as usize][j as usize] = hi[(i - 1) as usize][j as usize];
            }
        }
    }

    // Print the result
    println!("{}", hi[m as usize][n as usize]);
}
