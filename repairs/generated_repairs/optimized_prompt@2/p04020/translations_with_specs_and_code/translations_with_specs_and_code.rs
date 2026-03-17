use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");
    
    // Preconditions check
    assert!(n >= 1 && n <= 100_000, "N must be between 1 and 100,000");

    // Read the integers into a vector
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap().trim().to_string(); // Convert to String
        if let Ok(value) = line.parse::<i64>() {
            a.push(value);
        } else {
            eprintln!("Invalid input for integer: {}", line);
            return;
        }
    }

    let mut ans: i64 = 0;

    // Compute the result based on the specification
    for i in 0..(n - 1) {
        ans += a[i] / 2;
        if a[i + 1] > 1 && a[i] % 2 == 1 {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n - 1] / 2;

    // Output the result
    println!("{}", ans);
}
