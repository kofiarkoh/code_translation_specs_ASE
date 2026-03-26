use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse N");

    // Initialize arrays A and B
    let mut a = vec![0; n + 2]; // A has N + 1 elements
    let mut b = vec![0; n + 1]; // B has N elements

    // Read elements into A
    for i in 1..=n + 1 {
        a[i] = lines.next().unwrap().unwrap().parse().expect("Failed to parse A[i]");
    }

    // Read elements into B
    for i in 1..=n {
        b[i] = lines.next().unwrap().unwrap().parse().expect("Failed to parse B[i]");
    }

    // Initialize B[0] and B[N+1]
    b[0] = 0;
    b[n + 1] = 0;

    let mut s = 0; // Total transferable resources

    // Calculate transferable resources
    for i in 1..=n + 1 {
        let transfer_from_a_to_b = if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        s += transfer_from_a_to_b;
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };
        
        let transfer_from_a_to_b_next = if a[i] > b[i] { b[i] } else { a[i] };
        s += transfer_from_a_to_b_next;
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }

    // Output the total transferable resources
    println!("{}", s);
}
