use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse N");

    // Initialize arrays A and B
    let mut a = vec![0; n + 2]; // A[0] and A[n+1] are unused
    let mut b = vec![0; n + 2]; // B[0] and B[n+1] are used for boundary conditions

    // Read A values
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse::<i64>().expect("Failed to parse A value");
    }

    // Read B values
    for i in 1..=n {
        b[i] = lines.next().unwrap().unwrap().trim().parse::<i64>().expect("Failed to parse B value");
    }

    // Set boundary conditions
    b[0] = 0;
    b[n + 1] = 0;

    let mut s: i64 = 0; // Use i64 to match long long int in C
    for i in 1..=n + 1 {
        s += if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };
        s += if a[i] > b[i] { b[i] } else { a[i] };
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }

    // Print the result
    println!("{}", s);
}
