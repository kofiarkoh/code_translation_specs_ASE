use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Preconditions: n must be positive
    assert!(n > 0, "The number of elements must be positive.");

    // Read the sequence of integers
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Initialize the answer
    let mut ans: i64 = 0;

    // Compute the result based on the specified rules
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
