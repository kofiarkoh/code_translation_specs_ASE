use std::io::{self, BufRead};

fn bs(d: &[i32], n: usize, c: i32) -> usize {
    let mut j = 0;
    let mut k = n as isize - 1; // Use isize to handle negative indexing

    loop {
        if j <= k as usize {
            let l = (j + k as usize) / 2;
            if d[l] > c {
                k = l as isize - 1;
            } else if d[l] < c {
                j = l + 1;
            } else {
                return l;
            }
        } else {
            return k as usize + 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse a[i]");
    }

    let mut dp = vec![-1; n + 1]; // Initialize dp with -1
    dp[1] = a[0];
    let mut j = 1;

    for i in 1..n {
        if dp[j] < a[i] {
            dp[j + 1] = a[i];
            j += 1;
        } else {
            let index = bs(&dp[0..=j], j + 1, a[i]);
            dp[index] = a[i]; // Store the value at the found index
        }
    }

    println!("{}", j);
}
