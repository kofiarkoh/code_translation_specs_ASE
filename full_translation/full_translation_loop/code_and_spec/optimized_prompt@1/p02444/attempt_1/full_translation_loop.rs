use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the elements into the array
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each query
    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let b = query[0];
        let m = query[1];
        let e = query[2];

        // Create a temporary vector to hold the subarray
        let mut tmp: Vec<i32> = a[b..e].to_vec();
        let e_len = e - b;
        let m_adjusted = m % e_len;

        // Perform the circular shift
        for i in 0..e_len {
            a[b + i] = tmp[(i + m_adjusted) % e_len];
        }
    }

    // Output the modified array
    for (i, &value) in a.iter().enumerate() {
        if i == n - 1 {
            print!("{}", value);
        } else {
            print!("{} ", value);
        }
    }
    println!(); // Ensure a newline at the end
}
