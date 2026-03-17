use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut a = vec![0; m + 1]; // Adjust size to m + 1
    let mut b = vec![0; m + 1]; // Adjust size to m + 1
    let mut count1 = 0;
    let mut count2 = 0;

    // Read pairs of a[i] and b[i]
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        let mut pair = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a[i] = pair.next().unwrap();
        b[i] = pair.next().unwrap();

        if a[i] >= n as i32 {
            a[i] = 0;
            b[i] = 0;
            count1 += 1;
        }
    }

    if m > count1 { // Corrected condition
        for i in 1..=m {
            if a[i] < n as i32 { // Only update b[i] if a[i] is valid
                b[i] = n as i32 - a[i];
            }
        }

        // Bubble sort to sort b
        for i in 1..=m {
            for j in i + 1..=m {
                if b[i] > b[j] {
                    b.swap(i, j); // Use swap for idiomatic Rust
                }
            }
        }

        for i in 1..=m - count1 { // Adjusted loop to count valid b[i]
            count2 += b[i];
        }
    }

    println!("{}", count2);
}
