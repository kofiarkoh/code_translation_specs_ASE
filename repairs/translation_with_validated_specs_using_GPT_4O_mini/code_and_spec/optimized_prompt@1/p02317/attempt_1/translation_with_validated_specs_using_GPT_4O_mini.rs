use std::io::{self, BufRead};

fn binary_search(dp: &[i32], n: usize, c: i32) -> usize {
    let mut j = 0;
    let mut k = n as isize - 1;

    loop {
        if j <= k {
            let l = (j + k) / 2;
            if dp[l as usize] > c {
                k = l - 1;
            } else if dp[l as usize] < c {
                j = l + 1;
            } else {
                return l as usize;
            }
        } else {
            return (k + 1) as usize;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Preconditions check
    if n > 100_000 {
        panic!("Input n exceeds the maximum allowed value of 100,000.");
    }

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    let mut dp = vec![-1; n + 1]; // dp[0] is unused, dp[1] is initialized to a[0]
    dp[1] = a[0];
    let mut j = 1;

    for i in 1..n {
        if dp[j] < a[i] {
            dp[j + 1] = a[i];
            j += 1;
        } else {
            let pos = binary_search(&dp, j + 1, a[i]);
            dp[pos] = a[i];
        }
    }

    println!("{}", j);
}
