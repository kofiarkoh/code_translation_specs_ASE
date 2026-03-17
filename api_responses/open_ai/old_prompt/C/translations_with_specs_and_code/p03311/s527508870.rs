use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n > 0, "n must be greater than 0");

    // Read the integers into a vector
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for i in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value - (i as i64 + 1)); // Adjust by index
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

    for i in 0..n {
        if i + 1 < n && a[i + 1] == level {
            continue;
        }
        let width = i + 1;
        if n - width * 2 <= 0 {
            break;
        }
        let height = a[i + 1] - a[i];
        if height * (n - width * 2) <= sadness {
            level = a[i + 1];
            sadness -= height * (n - width * 2);
            min = sadness;
        } else {
            let height = sadness / (n - width * 2);
            level += height;
            sadness -= height * (n - width * 2);
            if sadness <= min {
                min = sadness;
            }
            sadness = -(sadness - (n - width * 2));
            if sadness <= min {
                min = sadness;
            }
            break;
        }
    }

    // Output the minimum sadness
    println!("{}", min);
}
