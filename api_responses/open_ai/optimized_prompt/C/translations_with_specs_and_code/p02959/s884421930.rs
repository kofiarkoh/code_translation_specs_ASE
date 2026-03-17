use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize arrays A and B
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n];

    // Read array A
    for i in 1..=n + 1 {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    // Read array B
    for i in 1..=n {
        b[i - 1] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    // Initialize B[0] and B[N+1] as per the C code logic
    b.insert(0, 0); // B[0] = 0
    b.push(0); // B[N+1] = 0LL

    let mut s: i64 = 0; // Cumulative sum

    // Compute the cumulative sum and adjust arrays A and B
    for i in 1..=n + 1 {
        s += if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };
        s += if a[i] > b[i] { b[i] } else { a[i] };
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }

    // Print the result
    println!("{}", s);
}
