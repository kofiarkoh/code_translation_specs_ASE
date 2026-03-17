fn main() {
    use std::io::{self, BufRead};

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Preconditions check
    if n == 0 || m == 0 {
        // Behavior is not defined for non-positive n or m
        return;
    }

    // Read the array
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: i32 = line.trim().parse().unwrap();
        a.push(value);
    }

    // Sorting based on the specification
    for k in 1..=m {
        for i in 0..n - 1 {
            for j in i + 1..n {
                if a[i] % k > a[j] % k {
                    a.swap(i, j); // Stable sort using swap
                }
            }
        }
    }

    // Output the sorted array
    for value in a {
        println!("{}", value);
    }
}
