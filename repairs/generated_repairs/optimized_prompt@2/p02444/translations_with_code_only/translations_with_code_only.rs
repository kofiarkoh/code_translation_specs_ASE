use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut a = vec![0; n]; // Adjust size of a to n
    let mut tmp = vec![0; n]; // Adjust size of tmp to n

    // Read the array a
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse array element");
    }

    // Read the number of queries q
    let q: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse q");
    
    for _ in 0..q {
        // Read b, m, e
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Failed to parse query element"))
            .collect();
        let (b, m, e) = (query[0], query[1], query[2]);

        // Ensure indices are within bounds
        if b < n && e <= n && b < e {
            // Copy elements from a to tmp
            for i in b..e {
                tmp[i] = a[i];
            }

            let e_len = e - b; // Adjust e to be the length of the segment
            let m_adjusted = (m - b) % e_len; // Adjust m to be the starting index in the segment

            // Rotate the elements in a
            for i in 0..e_len {
                a[b + i] = tmp[b + (i + m_adjusted) % e_len];
            }
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
