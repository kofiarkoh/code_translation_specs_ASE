use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the length of the character sequence
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    // Read the character sequence
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();

    // Read the number of queries
    let mut q = String::new();
    reader.read_line(&mut q).unwrap();
    let q: usize = q.trim().parse().unwrap();

    // Read the window sizes for the queries
    let mut k: Vec<usize> = Vec::new();
    for _ in 0..q {
        let mut window_size = String::new();
        reader.read_line(&mut window_size).unwrap();
        let window_size: usize = window_size.trim().parse().unwrap();
        k.push(window_size);
    }

    // Process each query
    for &window_size in &k {
        let mut ans: i64 = 0;
        let mut d: i64 = 0; // Count of 'D'
        let mut m: i64 = 0; // Count of 'M'
        let mut dm: i64 = 0; // Cumulative contribution of 'M'

        // Calculate the score for the initial window
        for i in 0..window_size {
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => unreachable!(), // According to the spec, only 'D', 'M', 'C' are valid
            }
        }

        // Slide the window across the character sequence
        for i in window_size..n {
            // Remove the character that is exiting the window
            match s[i - window_size] {
                'D' => {
                    dm -= m; // Adjust cumulative contribution
                    d -= 1; // Decrease count of 'D'
                }
                'M' => m -= 1, // Decrease count of 'M'
                'C' => {} // 'C' does not affect counts
                _ => unreachable!(),
            }

            // Add the character that is entering the window
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d; // Update cumulative contribution
                }
                'C' => ans += dm, // Update score for 'C'
                _ => unreachable!(),
            }
        }

        // Output the result for the current query
        println!("{}", ans);
    }
}
