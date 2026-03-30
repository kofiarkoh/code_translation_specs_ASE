use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>());
    
    // Handle potential parsing errors
    let n = nums.next().expect("Failed to read n").expect("Invalid number for n");
    let m = nums.next().expect("Failed to read m").expect("Invalid number for m");

    // Initialize the array with 1001 elements for 1-based indexing
    let mut a = vec![0; 1001];

    // Read the array elements
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        a[i as usize] = line.trim().parse::<i32>().expect("Invalid number in array");
    }

    let mut k = 1;
    while k <= m {
        let mut swapped = false;
        for i in 1..n {
            let j = i + 1;
            if j <= n && a[i as usize] % k > a[j as usize] % k {
                // Swap elements
                a.swap(i as usize, j as usize);
                swapped = true;
            }
        }
        k += 1;
        if !swapped {
            break;
        }
    }

    // Print the sorted array
    for i in 1..=n {
        println!("{}", a[i as usize]);
    }
}
