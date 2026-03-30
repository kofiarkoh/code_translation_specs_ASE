use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Make reader mutable
    
    // Read the number of elements
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Initialize arrays A and B
    let mut a = vec![0; n + 2];
    let mut b = vec![0; n + 2];

    // Read elements into array A
    input.clear();
    reader.read_line(&mut input).unwrap();
    for (i, value) in input.trim().split_whitespace().enumerate() {
        a[i + 1] = value.parse::<i64>().unwrap();
    }

    // Read elements into array B
    input.clear();
    reader.read_line(&mut input).unwrap();
    for (i, value) in input.trim().split_whitespace().enumerate() {
        b[i + 1] = value.parse::<i64>().unwrap();
    }

    // Set B[0] and B[N+1] to zero
    b[0] = 0;
    b[n + 1] = 0;

    let mut s = 0;

    // Process the arrays to compute the cumulative sum
    for i in 1..=n + 1 {
        s += a[i].min(b[i - 1]);
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };
        s += a[i].min(b[i]);
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }

    // Output the cumulative sum
    println!("{}", s);
}
