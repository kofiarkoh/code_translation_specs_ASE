fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut a = Vec::with_capacity(n);

    // Read the next n lines for the array a
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap().trim();
        let value: i32 = line.parse().expect("Failed to parse array element");
        a.push(value);
    }

    // Iterate from n-1 down to 0
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] -= 1; // Decrement a[i-1] if a[i] is less
        }
        if a[i] < a[i - 1] {
            // If after decrementing, a[i-1] is still greater, print "No" and return
            println!("No");
            return;
        }
    }
    // If we complete the loop without issues, print "Yes"
    println!("Yes");
}
