use std::io::{self, BufRead};

const M: i64 = 998244353;
const MAX_SIZE: usize = 90310;

fn main() {
    let stdin = io::stdin();
    let mut d = vec![0; MAX_SIZE];
    let mut e = vec![0; MAX_SIZE];
    let mut s = 0;
    let mut r = 1;

    // Read the first input
    let first_input = stdin.lock().lines().next().unwrap().unwrap();
    let first_value: i64 = first_input.trim().parse().expect("First input must be a valid integer");
    d[0] = first_value;
    e[0] = first_value;

    // Read subsequent inputs
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Ok(a) = line.trim().parse::<i64>() {
            let t = s;
            for t in (0..=t).rev() {
                d[(t + a) as usize] = (d[(t + a) as usize] + d[t as usize] * 2 % M) % M;
                e[(t + a) as usize] = (e[(t + a) as usize] + e[t as usize]) % M;
            }
            s += a;
            r = r * 3 % M;
        } else {
            break; // Stop reading on non-integer input
        }
    }

    // Compute the final result
    let mut t = s / 2;
    while t >= 0 {
        r = (r - d[t as usize] * 3) % M;
        t -= 1;
    }

    // Adjust for the final output
    let result = (r - (s % 2) * e[(s / 2) as usize] * 3 + M) % M;
    println!("{}", result);
}
