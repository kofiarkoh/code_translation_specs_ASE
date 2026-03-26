use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Ensure n is within the valid range
    assert!(n >= 1 && n <= 9, "n must be between 1 and 9");

    let mut A = vec![0; 9];
    let mut next_A = vec![0; 9];
    let mut prev_A = vec![0; 9];

    // Read the integers into A, next_A, and prev_A
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..n {
        A[i] = nums[i];
        next_A[i] = A[i];
        prev_A[i] = A[i];
    }

    if n == 1 {
        println!("{}", A[0]);
    } else {
        // Search for previous permutation
        let mut flag = false;
        let mut i = n - 2;
        while i >= 0 {
            if prev_A[i] > prev_A[i + 1] {
                flag = true;
                break;
            }
            if i == 0 { break; } // Prevent underflow
            i -= 1;
        }

        if flag {
            let i = i as usize;
            let mut j = n - 1;
            while j > i {
                if prev_A[j] < prev_A[i] {
                    prev_A.swap(j, i);
                    break;
                }
                j -= 1;
            }
            let mid = (n - i - 1) / 2;
            for k in 1..=mid {
                prev_A.swap(i + k, n - k);
            }
            // Print the previous permutation
            print!("{}", prev_A[0]);
            for i in 1..n {
                print!(" {}", prev_A[i]);
            }
            println!();
        }

        // Print the original sequence
        print!("{}", A[0]);
        for i in 1..n {
            print!(" {}", A[i]);
        }
        println!();

        // Search for next permutation
        flag = false;
        let mut i = n - 2;
        while i >= 0 {
            if next_A[i] < next_A[i + 1] {
                flag = true;
                break;
            }
            if i == 0 { break; } // Prevent underflow
            i -= 1;
        }

        if flag {
            let i = i as usize;
            let mut j = n - 1;
            while j > i {
                if next_A[j] > next_A[i] {
                    next_A.swap(j, i);
                    break;
                }
                j -= 1;
            }
            let mid = (n - i - 1) / 2;
            for k in 1..=mid {
                next_A.swap(i + k, n - k);
            }
            // Print the next permutation
            print!("{}", next_A[0]);
            for i in 1..n {
                print!(" {}", next_A[i]);
            }
            println!();
        }
    }
}
