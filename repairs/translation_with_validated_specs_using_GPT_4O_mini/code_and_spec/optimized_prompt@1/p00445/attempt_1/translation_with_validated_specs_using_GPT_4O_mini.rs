use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let (joi_count, ioi_count) = count_occurrences(&s);
                println!("{}", joi_count);
                println!("{}", ioi_count);
            }
            Err(_) => break, // Handle any read errors gracefully
        }
    }
}

fn count_occurrences(s: &str) -> (usize, usize) {
    let mut joi_count = 0;
    let mut ioi_count = 0;

    // Iterate through the string, checking for "JOI" and "IOI"
    for i in 0..s.len().saturating_sub(2) {
        if &s[i..i + 3] == "JOI" {
            joi_count += 1;
        }
        if &s[i..i + 3] == "IOI" {
            ioi_count += 1;
        }
    }

    (joi_count, ioi_count)
}
