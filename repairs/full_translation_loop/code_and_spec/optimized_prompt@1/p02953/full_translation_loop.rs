use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return, // Handle case where input is not valid
    };

    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the sequence of integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.trim().parse::<i32>() {
                a.push(value);
            } else {
                return; // Handle case where input is not a valid integer
            }
        } else {
            return; // Handle case where input is not valid
        }
    }

    // Check for non-increasing order possibility
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] = a[i - 1].max(a[i]); // Ensure a[i-1] is not decremented below a[i]
        }
        if a[i] < a[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
