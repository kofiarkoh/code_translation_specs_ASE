use std::io::{self, BufRead};

fn main() {
    // Read the number of elements in the array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut array: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Extract the start index, shift amount, and end index from the query
        let start = query[0];
        let shift = query[1];
        let end = query[2];

        // Calculate the effective shift amount
        let length = end - start + 1;
        let effective_shift = shift % length;

        // Perform the circular shift
        if effective_shift > 0 {
            let subarray = &mut array[start..=end];
            let len = subarray.len();
            let temp: Vec<i32> = subarray[len - effective_shift..].to_vec();
            subarray.copy_within(0..len - effective_shift, effective_shift);
            subarray[..effective_shift].copy_from_slice(&temp);
        }
    }

    // Output the modified array
    for value in array {
        print!("{} ", value);
    }
    println!(); // Print a newline at the end
}
