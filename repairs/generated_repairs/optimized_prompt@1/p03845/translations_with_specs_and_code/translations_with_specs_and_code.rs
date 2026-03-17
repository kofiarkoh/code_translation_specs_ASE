fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the first list
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n > 0, "The number of elements must be greater than zero.");

    // Read the list of integers
    let mut t = Vec::with_capacity(n);
    let t_line: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    t.extend(t_line);

    // Read the number of updates
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(m >= 0, "The number of updates must be greater than or equal to zero.");

    // Read the update pairs
    let mut p = Vec::with_capacity(m);
    let mut x = Vec::with_capacity(m);
    for _ in 0..m {
        let update: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        p.push(update[0]);
        x.push(update[1]);
    }

    // Calculate the total sum of the first list
    let sum: i32 = t.iter().sum();

    // Output the modified sums based on the updates
    for i in 0..m {
        // p[i] is 1-based index, so we need to subtract 1 for 0-based indexing
        let index = p[i] - 1;
        if index < n {
            println!("{}", sum - t[index] + x[i]);
        } else {
            // Behavior is not defined for out-of-bounds indices
            // In a real application, you might want to handle this case
            // For now, we will just continue
            continue;
        }
    }
}
