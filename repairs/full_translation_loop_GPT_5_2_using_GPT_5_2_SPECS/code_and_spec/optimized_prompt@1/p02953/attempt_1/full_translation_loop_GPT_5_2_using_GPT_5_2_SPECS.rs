use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Spec notes undefined behavior if n exceeds 100,009; we mirror the fixed-capacity intent.
    // In Rust we still allocate dynamically; we do not add extra checks to preserve behavior.
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Right-to-left pass; at most one decrement of the left element per adjacent pair.
    // Elements to the right are never modified.
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] -= 1;
        }
        if a[i] < a[i - 1] {
            print!("No");
            return;
        }
    }

    print!("Yes");
}
