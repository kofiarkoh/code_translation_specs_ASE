fn main() {
    // Read input values
    let (n, a, q, queries) = read_input();

    // Process each query
    let mut a = a; // Mutable array to perform shifts
    for (b, m, e) in queries {
        // Create a temporary array to hold the segment to be shifted
        let mut tmp = a[b..e].to_vec(); // Copy the relevant segment

        // Perform the circular shift
        let len = e - b; // Length of the segment
        let shift = m % len; // Effective shift (to handle cases where m >= len)
        if shift > 0 {
            // Shift the segment
            for i in 0..len {
                a[b + i] = tmp[(i + len - shift) % len]; // Circular indexing
            }
        }
    }

    // Output the modified array
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

// Function to read input values
fn read_input() -> (usize, Vec<i32>, usize, Vec<(usize, usize, usize)>) {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read n
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    // Read array a
    let mut a = Vec::new();
    if n > 0 {
        let mut a_input = String::new();
        reader.read_line(&mut a_input).unwrap();
        a = a_input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    // Read q
    let mut q = String::new();
    reader.read_line(&mut q).unwrap();
    let q: usize = q.trim().parse().unwrap();

    // Read queries
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut query_input = String::new();
        reader.read_line(&mut query_input).unwrap();
        let query: Vec<usize> = query_input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        queries.push((query[0], query[1], query[2]));
    }

    (n, a, q, queries)
}
