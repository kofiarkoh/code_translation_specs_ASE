use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize arrays
    let mut a = vec![0; m + 1]; // 1-based indexing
    let mut b = vec![0; m + 1]; // 1-based indexing
    let mut count1 = 0;
    let mut count2 = 0;

    // Read pairs and process
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        let mut pair_parts = line.split_whitespace();
        a[i] = pair_parts.next().unwrap().parse().unwrap();
        b[i] = pair_parts.next().unwrap().parse().unwrap();

        if a[i] >= n {
            a[i] = 0;
            b[i] = 0;
            count1 += 1;
        }
    }

    if m - 1 > count1 {
        for i in 1..=m {
            b[i] = n - a[i];
        }

        // Sort b using bubble sort (as in the C code)
        for i in 1..m {
            for j in (i + 1)..=m {
                if b[i] > b[j] {
                    b.swap(i, j);
                }
            }
        }

        for i in 1..=m - count1 - 1 {
            count2 += b[i];
        }
    }

    // Output the result
    println!("{}", count2);
}
