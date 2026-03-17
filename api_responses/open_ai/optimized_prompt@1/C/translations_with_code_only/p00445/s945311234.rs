use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let str = line.expect("Failed to read line");
        let mut joi = 0;
        let mut ioi = 0;

        // We only need to check up to len - 2 to avoid out-of-bounds
        let len = str.len();
        for i in 0..len.saturating_sub(2) {
            if &str[i..i + 3] == "JOI" {
                joi += 1;
            }
            if &str[i..i + 3] == "IOI" {
                ioi += 1;
            }
        }

        // Print results as per the original C code
        println!("{}", joi);
        println!("{}", ioi);
    }
}
