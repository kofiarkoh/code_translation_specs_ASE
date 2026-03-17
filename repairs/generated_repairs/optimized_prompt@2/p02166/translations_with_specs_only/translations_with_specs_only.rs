fn main() {
    use std::io::{self, BufRead};
    use std::convert::TryInto;

    // Read input from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Preconditions check
    if n < 1 || n > 100_000 || k < 0 {
        // Undefined behavior as per specification; we do not handle this explicitly.
        return;
    }

    // Read the array of integers
    let mut array: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Check if the number of elements equals k
    if n == k.try_into().unwrap() {
        // Check if all elements satisfy the modular condition
        let all_satisfy = array.iter().all(|&x| k != 0 && x % k == 0);
        // Mark processed elements
        for elem in &mut array {
            *elem = -1; // Mark as processed
        }
        // Output result
        if all_satisfy {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        // Evaluate the parity of the count of processed elements
        let processed_count = array.iter().filter(|&&x| x == -1).count();
        // Output "Yes" if processed count is even, "No" if odd
        if processed_count % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
