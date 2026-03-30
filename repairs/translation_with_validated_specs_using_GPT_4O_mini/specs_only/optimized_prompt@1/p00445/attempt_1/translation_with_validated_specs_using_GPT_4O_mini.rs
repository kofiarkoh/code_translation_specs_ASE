use std::io::{self, BufRead};

fn count_occurrences(s: &str, substring: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = s[start..].find(substring) {
        count += 1;
        start += pos + 1; // Move start to allow for overlapping occurrences
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(input) = line {
            if input.is_empty() {
                continue; // If the input is empty, no output is produced
            }

            let joi_count = count_occurrences(&input, "JOI");
            let ioi_count = count_occurrences(&input, "IOI");

            println!("{}", joi_count);
            println!("{}", ioi_count);
        }
    }
}
