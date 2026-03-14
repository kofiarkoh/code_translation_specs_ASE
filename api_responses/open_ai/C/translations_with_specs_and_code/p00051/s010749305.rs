use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read each 8-digit number
        let m: u32 = lines.next().unwrap().unwrap().parse().unwrap();

        // Extract digits
        let mut digits: Vec<u32> = Vec::new();
        let mut temp = m;

        for _ in 0..8 {
            digits.push(temp % 10);
            temp /= 10;
        }

        // Sort digits to find min and max
        digits.sort();

        // Calculate minimum value
        let min: u32 = digits.iter().enumerate().fold(0, |acc, (i, &digit)| {
            acc * 10 + digit
        });

        // Calculate maximum value
        let max: u32 = digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| {
            acc * 10 + digit
        });

        // Output the difference
        println!("{}", max - min);
    }
}
