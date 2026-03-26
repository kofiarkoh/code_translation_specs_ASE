use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut a = Vec::with_capacity(n); // Allocate only the necessary size for a

    // Read the next n lines for the array a
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse a[i]");
        a.push(value);
    }

    // Iterate from the end of the array to the beginning
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] -= 1; // Decrement a[i - 1] if a[i] is less
        }
        if a[i] < a[i - 1] {
            // If after decrementing, a[i - 1] is still greater than a[i], print "No" and return
            println!("No");
            return;
        }
    }
    // If we complete the loop without returning, print "Yes"
    println!("Yes");
}
