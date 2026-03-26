use std::io::{self, BufRead};

fn main() {
    // Read input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse N");
    let mut a: Vec<i64> = vec![0; n + 1]; // 1-based indexing

    // Read the integers into the array
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().parse().expect("Failed to parse integer");
    }

    let mut ans = 0;

    // Compute the sum based on the specified logic
    for i in 1..n {
        ans += a[i] / 2;
        if a[i + 1] > 1 && a[i] % 2 == 1 {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n] / 2;

    // Output the result
    println!("{}", ans);
}
