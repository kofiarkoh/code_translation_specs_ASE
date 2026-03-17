use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Initialize the array with 1000 elements
    let mut a = vec![0; 1000];

    // Read the array elements
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        a[i as usize] = line.trim().parse::<i32>().unwrap();
    }

    let mut k = 1;
    loop {
        // Break the loop if k exceeds m
        if k > m {
            break;
        }

        for i in 1..n {
            let j = i + 1;
            if j <= n {
                // Compare a[i] % k with a[j] % k
                if a[i as usize] % k > a[j as usize] % k {
                    // Swap elements
                    a.swap(i as usize, j as usize);
                }
            }
        }
        k += 1;
    }

    // Print the sorted array
    for i in 1..=n {
        println!("{}", a[i as usize]);
    }
}
