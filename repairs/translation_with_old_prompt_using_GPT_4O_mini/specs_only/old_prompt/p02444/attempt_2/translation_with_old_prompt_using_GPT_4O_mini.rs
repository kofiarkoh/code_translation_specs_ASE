use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements in the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize the array `a`
    let mut a: Vec<i32> = Vec::new();
    if n > 0 {
        // Read the array elements
        a.extend(lines.next().unwrap().unwrap().trim().split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())); // Specify i32 for parsing
    }

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each query
    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let b = query[0];
        let m = query[1];
        let e = query[2];

        // Create a temporary array to hold the segment
        let mut tmp: Vec<i32> = a[b..e].to_vec();

        // Perform the circular shift
        let len = e - b;
        for i in 0..len {
            a[b + i] = tmp[(i + len - m) % len];
        }
    }

    // Output the modified array
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
