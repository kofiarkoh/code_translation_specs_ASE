use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let str = line.expect("Failed to read line");
        let mut joi = 0;
        let mut ioi = 0;

        // Iterate over the string, checking for "JOI" and "IOI"
        for i in 0..str.len().saturating_sub(2) { // saturating_sub to avoid underflow
            if str.get(i..i + 3) == Some("JOI") {
                joi += 1;
            }
            if str.get(i..i + 3) == Some("IOI") {
                ioi += 1;
            }
        }

        // Print results
        println!("{}", joi);
        println!("{}", ioi);
    }
}
