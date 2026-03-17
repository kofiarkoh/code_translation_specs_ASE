use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read N and K
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the array A
    let mut a = vec![0; n + 1]; // Using 1-based indexing
    let second_line = lines.next().unwrap().unwrap();
    let values: Vec<i32> = second_line.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
    for i in 1..=n {
        a[i] = values[i - 1]; // Fill A with input values
    }

    let mut b = vec![0; n + 2]; // Auxiliary array B
    for _ in 0..k {
        let mut min = n; // Initialize min to N
        for i in 1..=n + 1 {
            b[i] = 0; // Reset B
        }
        for i in 1..=n {
            if a[i] >= i as i32 {
                b[1] += 1; // Increment B[1]
            } else {
                b[i - a[i] as usize] += 1; // Increment B[i - A[i]]
            }
            if a[i] >= (n - i) as i32 {
                b[n + 1] -= 1; // Decrement B[N + 1]
            } else {
                b[i + a[i] as usize + 1] -= 1; // Decrement B[i + A[i] + 1]
            }
        }
        let mut tmp = 0;
        for i in 1..=n {
            tmp += b[i]; // Cumulative sum
            a[i] = tmp; // Update A
            if a[i] < min {
                min = a[i]; // Update min
            }
        }
    }

    // Output the transformed array
    print!("{}", a[1]);
    for i in 2..=n {
        print!(" {}", a[i]);
    }
    println!(); // Newline at the end
}
