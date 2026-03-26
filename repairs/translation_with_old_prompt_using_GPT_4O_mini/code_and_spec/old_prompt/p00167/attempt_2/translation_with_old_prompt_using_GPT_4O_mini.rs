use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let n: usize = match line.trim().parse() {
            Ok(num) if num > 0 && num <= 100 => num,
            _ => break, // Terminate on invalid input
        };

        let mut a = Vec::with_capacity(n);
        let mut swap_count = 0;

        // Read the next line containing n integers
        let numbers_line = reader.lines().next();
        if let Some(numbers_line) = numbers_line {
            let numbers: Vec<i32> = numbers_line
                .expect("Failed to read line")
                .split_whitespace()
                .take(n)
                .filter_map(|s| s.parse().ok())
                .collect();

            if numbers.len() != n {
                break; // Terminate if we don't have exactly n numbers
            }

            a.extend(numbers);

            // Bubble Sort implementation
            for u in (1..n).rev() {
                for i in 0..u {
                    if a[i] > a[i + 1] {
                        a.swap(i, i + 1);
                        swap_count += 1;
                    }
                }
            }

            println!("{}", swap_count);
        } else {
            break; // Terminate if there are no more lines to read
        }
    }
}
