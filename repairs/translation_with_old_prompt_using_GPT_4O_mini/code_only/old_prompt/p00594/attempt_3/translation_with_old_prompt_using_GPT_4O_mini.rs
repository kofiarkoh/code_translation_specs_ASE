use std::io::{self, BufRead};
use std::cmp::Ordering;

fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a) // Reverse order for sorting
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let mut a = vec![0; 1_000_001]; // Allocate space for 1,000,001 integers

    while let Some(Ok(line)) = input.next() {
        let n: i32 = line.trim().parse().expect("Failed to parse integer");
        if n == 0 {
            break; // Exit on zero input
        }

        let mut m = 0;
        let mut k = 0;

        for i in 0..n {
            let line = input.next().expect("Failed to read line").expect("Failed to unwrap line");
            a[i as usize] = line.trim().parse().expect("Failed to parse integer");
        }

        // Sort the array using Rust's sort function with custom comparator
        a[0..n as usize].sort_by(cmp);

        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j <= n {
                if j == n || a[i as usize] != a[j as usize] {
                    if m < j - i {
                        m = j - i;
                        k = a[i as usize];
                    }
                    i = j - 1; // Move i to the last position of the current group
                    break;
                }
                j += 1;
            }
            i += 1; // Increment i to continue the outer loop
        }

        if m > n / 2 {
            println!("{}", k);
        } else {
            println!("NO COLOR");
        }
    }
}
