use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    // Read the length of the character sequence
    let mut n_str = String::new();
    input.read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().expect("Invalid input for length");

    // Read the character sequence
    let mut s = String::new();
    input.read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();

    // Read the number of queries
    let mut q_str = String::new();
    input.read_line(&mut q_str).unwrap();
    let q: usize = q_str.trim().parse().expect("Invalid input for number of queries");

    // Read the lengths for each query
    let mut k = Vec::new();
    for _ in 0..q {
        let mut k_str = String::new();
        input.read_line(&mut k_str).unwrap();
        let query_length: usize = k_str.trim().parse().expect("Invalid input for query length");
        k.push(query_length);
    }

    // Process each query
    for &query_length in &k {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        // Calculate the score for the first substring of length `query_length`
        for i in 0..query_length {
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => unreachable!(), // According to the spec, only 'D', 'M', and 'C' are valid
            }
        }

        // Slide the window across the rest of the string
        for i in query_length..n {
            // Remove the character going out of the window
            match s[i - query_length] {
                'D' => {
                    dm -= m;
                    d -= 1;
                }
                'M' => m -= 1,
                _ => {}
            }

            // Add the new character coming into the window
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => unreachable!(), // According to the spec, only 'D', 'M', and 'C' are valid
            }
        }

        // Output the result for the current query
        println!("{}", ans);
    }
}
