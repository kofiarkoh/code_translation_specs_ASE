use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = match line.trim().parse() {
            Ok(num) if num > 0 && num <= 100 => num,
            _ => continue, // Skip invalid input
        };

        let mut a = Vec::with_capacity(n);
        let mut swap_count = 0;

        // Read the next line for the integers to sort
        let line = reader.lines().next().unwrap().unwrap();
        for num in line.split_whitespace().take(n) {
            if let Ok(value) = num.parse::<i32>() {
                a.push(value);
            }
        }

        // Bubble Sort implementation
        for u in (1..n).rev() {
            for i in 0..u {
                if a[i] > a[i + 1] {
                    a.swap(i, i + 1);
                    swap_count += 1;
                }
            }
        }

        // Output the number of swaps
        println!("{}", swap_count);
    }
}
