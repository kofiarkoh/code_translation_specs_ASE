fn main() {
    use std::io::{self, BufRead};

    // Read the number of elements in the sequence
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut sequence: Vec<i32> = lines.next().unwrap().unwrap()
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

        // Calculate the length of the subarray
        let len = end - start + 1;

        // Normalize the shift amount to avoid unnecessary full rotations
        let effective_shift = shift % len;

        // Perform the circular shift
        if effective_shift > 0 {
            let subarray = sequence[start..=end].to_vec();
            let new_subarray = [
                &subarray[len - effective_shift..],
                &subarray[..len - effective_shift],
            ].concat();
            sequence[start..=end].copy_from_slice(&new_subarray);
        }
    }

    // Print the modified sequence
    println!("{}", sequence.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
