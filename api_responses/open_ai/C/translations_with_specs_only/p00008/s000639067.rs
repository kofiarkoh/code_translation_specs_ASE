use std::io::{self, BufRead};

fn count_combinations(target: u32) -> u32 {
    let mut count = 0;

    // Iterate through all combinations of four digits (0-9)
    for a in 0..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    if a + b + c + d == target {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(input) = line {
            if let Ok(target) = input.parse::<u32>() {
                let combinations_count = count_combinations(target);
                println!("{}", combinations_count);
            }
        }
    }
}
