use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n > 0, "n must be greater than 0");

    // Read the integers into a vector
    let mut a: Vec<i64> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Adjust each integer by subtracting its index (1-based)
    for i in 0..n {
        a[i] -= (i + 1) as i64;
    }

    // Sort the adjusted integers
    a.sort();

    let mut all = 0;
    for i in 1..n {
        all += a[i] - a[0];
    }

    let mut sadness = all;
    let mut min = all;
    let mut level = a[0];
    let mut width: usize;
    let mut height: i64;

    for i in 0..n {
        if i + 1 < n && a[i + 1] == level {
            continue;
        }
        width = i + 1;
        if n - width * 2 <= 0 {
            break;
        }
        height = a[i + 1] - a[i];
        if height * (n - width * 2) <= sadness {
            level = a[i + 1];
            sadness -= height * (n - width * 2);
            min = sadness;
        } else {
            height = sadness / (n - width * 2) as i64;
            level += height;
            sadness -= height * (n - width * 2);
            if sadness <= min {
                min = sadness;
            }
            sadness = -(sadness - (n - width * 2) as i64);
            if sadness <= min {
                min = sadness;
            }
            break;
        }
    }

    // Output the minimum sadness
    println!("{}", min);
}
