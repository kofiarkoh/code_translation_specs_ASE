use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a = vec![0; 1000]; // Initialize array a with 1000 elements
    let mut tmp = vec![0; 1000]; // Initialize array tmp with 1000 elements

    // Read the next n integers into array a
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    // Read the number of queries q
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each query
    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        
        let b = query[0];
        let m = query[1];
        let e = query[2];

        // Copy elements from a to tmp
        for i in b..e {
            tmp[i] = a[i];
        }

        let e_len = e - b; // Length of the segment
        let m_adjusted = m - b; // Adjust m to be relative to b

        // Rotate the elements in a
        for i in 0..e_len {
            a[b + i] = tmp[b + (i + m_adjusted) % e_len];
        }
    }

    // Print the result
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!(); // Ensure a newline at the end
}
