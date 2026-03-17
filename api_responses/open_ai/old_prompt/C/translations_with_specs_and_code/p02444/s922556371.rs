use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n];

    // Read the array elements
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each query
    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let b = query[0];
        let m = query[1];
        let e = query[2];

        // Create a temporary array to hold the segment
        let mut tmp = vec![0; e - b];

        // Copy the relevant segment of `a` into `tmp`
        for i in b..e {
            tmp[i - b] = a[i];
        }

        // Perform the circular shift
        let e_len = e - b;
        let m_adjusted = m % e_len; // Ensure m is within bounds
        for i in 0..e_len {
            a[b + i] = tmp[(i + m_adjusted) % e_len];
        }
    }

    // Output the modified array
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!(); // Ensure a newline at the end
}
