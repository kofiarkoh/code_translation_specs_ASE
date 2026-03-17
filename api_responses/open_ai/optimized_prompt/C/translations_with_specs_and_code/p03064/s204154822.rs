use std::io::{self, BufRead};

const M: i64 = 998244353;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut d = vec![0; 90310];
    let mut e = vec![0; 90310];
    let mut s = 0;
    let mut r = 1;

    // Read the first input, which must be a non-negative integer
    let mut first_input = String::new();
    reader.read_line(&mut first_input).expect("Failed to read line");
    let initial: i64 = first_input.trim().parse().expect("Invalid input");

    // Initialize the first element of d and e
    d[0] = initial;
    e[0] = 0; // e[0] is initialized to 0 as per the C code

    // Process subsequent inputs
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let a: i64 = line.trim().parse().expect("Invalid input");

        // Update the arrays based on the input
        for t in (0..=s).rev() {
            d[(t + a) as usize] = (d[(t + a) as usize] + d[t as usize] * 2 % M) % M;
            e[(t + a) as usize] = (e[(t + a) as usize] + e[t as usize]) % M;
        }
        s += a;
        r = r * 3 % M;
    }

    // Final computation
    for t in (0..=s / 2).rev() {
        r = (r - d[t as usize] * 3) % M;
    }

    // Adjust for the final output
    let output = (r - (s % 2) * e[(s / 2) as usize] * 3 + M) % M;
    println!("{}", output);
}
