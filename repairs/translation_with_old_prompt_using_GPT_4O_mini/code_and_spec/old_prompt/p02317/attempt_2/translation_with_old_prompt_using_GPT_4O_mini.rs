use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Handle the case where n is 0
    if n == 0 {
        println!("0");
        return;
    }

    // Read the sequence of integers
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Initialize dp array
    let mut dp: Vec<i32> = vec![-1; n + 1];
    dp[1] = a[0];
    let mut j = 1;

    for i in 1..n {
        if dp[j] < a[i] {
            dp[j + 1] = a[i];
            j += 1;
        } else {
            let pos = bs(&dp, j + 1, a[i]);
            dp[pos] = a[i];
        }
    }

    // Output the length of the longest increasing subsequence
    println!("{}", j);
}

// Binary search function to find the position to replace
fn bs(D: &[i32], n: usize, c: i32) -> usize {
    let mut j = 0;
    let mut k = n as isize - 1;

    loop {
        if j <= k as usize {
            let l = (j + k as usize) / 2;
            if D[l] > c {
                k = l as isize - 1;
            } else if D[l] < c {
                j = l + 1;
            } else {
                return l;
            }
        } else {
            return k as usize + 1;
        }
    }
}
