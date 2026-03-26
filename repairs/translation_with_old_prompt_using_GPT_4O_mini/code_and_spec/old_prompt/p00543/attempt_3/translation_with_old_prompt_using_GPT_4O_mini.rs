use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = params.next().unwrap().parse().expect("Failed to parse m");

    // Read the array a
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: i32 = line.trim().parse().expect("Failed to parse array element");
        a.push(value);
    }

    // Sorting based on modulo operation
    for k in 1..=m {
        for i in 0..n - 1 {
            for j in i + 1..n {
                // Ensure both indices are within bounds and use i32 for modulo
                if a[i] % k as i32 > a[j] % k as i32 {
                    a.swap(i, j);
                }
            }
        }
    }

    // Output the sorted array
    for value in a {
        println!("{}", value);
    }
}
