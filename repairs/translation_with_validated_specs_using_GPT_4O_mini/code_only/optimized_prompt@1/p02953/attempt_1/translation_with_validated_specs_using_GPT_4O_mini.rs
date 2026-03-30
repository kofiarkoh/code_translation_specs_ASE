use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a = vec![0; n];

    // Read the next n lines for the array a
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    // Iterate from the end of the array to the beginning
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] = a[i - 1].wrapping_sub(1); // Emulate C's integer wraparound behavior
        }
        if a[i] < a[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
