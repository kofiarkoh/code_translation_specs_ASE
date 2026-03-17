use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of elements in array B
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize arrays A and B
    let mut a = vec![0; n + 2]; // A has N + 1 elements
    let mut b = vec![0; n + 1]; // B has N elements
    
    // Read elements into array A
    for i in 1..=n + 1 {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Read elements into array B
    for i in 1..=n {
        b[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Initialize B[0] and B[N+1] as per the C code
    b.insert(0, 0); // B[0] = 0
    b.push(0); // B[N+1] = 0LL
    
    let mut s: i64 = 0; // Total transferable resources
    
    // Calculate transferable resources
    for i in 1..=n + 1 {
        s += if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };
        s += if a[i] > b[i] { b[i] } else { a[i] };
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }
    
    // Output the total transferable resources
    println!("{}", s);
}
