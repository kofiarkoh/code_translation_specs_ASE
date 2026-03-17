fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the first list
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n > 0, "The number of elements must be greater than zero.");

    // Read the list of integers
    let mut t = vec![0; n];
    for i in 0..n {
        t[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Read the number of updates
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(m >= 0, "The number of updates must be greater than or equal to zero.");

    // Read the update pairs
    let mut p = vec![0; m];
    let mut x = vec![0; m];
    for i in 0..m {
        p[i] = lines.next().unwrap().unwrap().parse().unwrap();
        x[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Calculate the total sum of the first list
    let sum: i32 = t.iter().sum();

    // Output the modified sums based on the updates
    for i in 0..m {
        // Check if the index is valid (1-based index in input)
        if p[i] > 0 && p[i] <= n {
            println!("{}", sum - t[p[i] - 1] + x[i]);
        } else {
            // Behavior is not defined for out-of-bounds indices
            // In a real application, you might want to handle this case
            // For now, we will just skip the output for invalid indices
            // println!("Index out of bounds: {}", p[i]);
        }
    }
}
