use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the first line to get the number of heights
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize the heights array with zeros
    let mut h = vec![0; n + 10];

    // Read the heights
    for i in 0..n {
        h[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    let mut max_count = 0;
    let mut count = 0;

    // Calculate the maximum length of consecutive non-increasing heights
    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            count += 1;
        } else {
            count = 0;
        }
        max_count = max_count.max(count);
    }

    // Output the result
    println!("{}", max_count);
}
