use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    // Read the length of the string
    let mut n_str = String::new();
    input.read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    // Read the string
    let mut s = String::new();
    input.read_line(&mut s).unwrap();
    let s = s.trim(); // Ensure we trim any newline characters

    // Read the number of queries
    let mut q_str = String::new();
    input.read_line(&mut q_str).unwrap();
    let q: usize = q_str.trim().parse().unwrap();

    // Read the lengths of substrings for each query
    let mut k = Vec::with_capacity(q);
    for _ in 0..q {
        let mut k_str = String::new();
        input.read_line(&mut k_str).unwrap();
        let k_value: usize = k_str.trim().parse().unwrap();
        k.push(k_value);
    }

    // Process each query
    for &query_length in &k {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        // Initial window calculation
        for i in 0..query_length {
            match s.chars().nth(i).unwrap() {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => {}
            }
        }

        // Sliding window calculation
        for i in query_length..n {
            // Remove the character going out of the window
            match s.chars().nth(i - query_length).unwrap() {
                'D' => {
                    dm -= m;
                    d -= 1;
                }
                'M' => m -= 1,
                _ => {}
            }

            // Add the new character coming into the window
            match s.chars().nth(i).unwrap() {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => {}
            }
        }

        // Output the result for the current query
        println!("{}", ans);
    }
}
