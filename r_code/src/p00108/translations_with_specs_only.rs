use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check for termination condition
        if numbers.is_empty() || numbers[0] == 0 {
            break;
        }

        // The first number indicates the count of subsequent integers
        let count = numbers[0];
        if count != numbers.len() - 1 {
            // Undefined behavior as per specification; we will just terminate
            return;
        }

        let mut sequence = numbers[1..].to_vec();
        let mut iterations = 0;

        loop {
            iterations += 1;
            let mut counts = vec![0; 14]; // Fixed size for counts (0-13)
            for &num in &sequence {
                counts[num] += 1;
            }

            let new_sequence: Vec<usize> = counts.iter()
                .enumerate()
                .filter_map(|(num, &count)| if count > 0 { Some(count) } else { None })
                .collect();

            // Check for stability
            if new_sequence == sequence {
                break;
            }
            sequence = new_sequence;
        }

        // Output the number of iterations and the final sequence
        println!("{}", iterations);
        println!("{}", sequence.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
    }
}
