use std::io::{self, BufRead};
use std::cmp::Ordering;

fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a) // Reverse order to match the C comparison function
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut a = vec![0; 1_000_001]; // Allocate space for 1,000,001 integers
    let mut input = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();
        if n == 0 {
            break; // Exit on zero input
        }

        let mut m = 0;
        let mut k = 0;

        // Read n integers
        for i in 0..n {
            input.clear();
            let _ = reader.read_line(&mut input).unwrap(); // Read line into input
            a[i as usize] = input.trim().parse().unwrap();
        }

        // Sort the array in descending order
        a[0..n as usize].sort_by(cmp);

        // Find the longest segment of equal values
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j <= n && (j == n || a[i as usize] != a[j as usize]) {
                if m < j - i {
                    m = j - i;
                    k = a[i as usize];
                }
                i = j - 1; // Move i to the last position of the current segment
                break;
            }
            j += 1; // Increment j for the next iteration
        }

        if n / 2 < m {
            println!("{}", k);
        } else {
            println!("NO COLOR");
        }
    }
}
