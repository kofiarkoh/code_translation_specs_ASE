use std::io::{self, BufRead};

fn binary_search(dp: &[i32], n: usize, c: i32) -> usize {
    let mut j = 0;
    let mut k = n as isize - 1;

    loop {
        if j <= k as usize {
            let l = (j + k as usize) / 2;
            if dp[l] > c {
                k = l as isize - 1;
            } else if dp[l] < c {
                j = l + 1;
            } else {
                return l;
            }
        } else {
            return j;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n];
    let mut dp = vec![-1; n + 1]; // dp[0] is unused, dp[1] is initialized to a[0]
    
    // Read the sequence of integers
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    dp[1] = a[0];
    let mut j = 1;

    for i in 1..n {
        if dp[j] < a[i] {
            dp[j + 1] = a[i];
            j += 1;
        } else {
            let pos = binary_search(&dp[1..=j], j, a[i]);
            dp[pos + 1] = a[i];
        }
    }

    // Output the length of the longest increasing subsequence
    println!("{}", j);
}
