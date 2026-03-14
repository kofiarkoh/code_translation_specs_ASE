use std::io::{self, BufRead};
use std::f64;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = match iter.next() {
            Some(num) => num.parse().unwrap(),
            None => break,
        };
        let h: i32 = match iter.next() {
            Some(num) => num.parse().unwrap(),
            None => break,
        };

        // Check if x + h is non-zero to continue the loop
        if x + h == 0 {
            break;
        }

        // Calculate the result as per the C code
        let result = (f64::sqrt((x * x) as f64 + 4.0 * (h * h) as f64) * (x as f64) + (x * x) as f64);
        println!("{:.6}", result); // Preserving the output format
    }
}
