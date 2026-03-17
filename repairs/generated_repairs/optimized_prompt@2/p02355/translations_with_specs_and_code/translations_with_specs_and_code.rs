use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>());
    let n = match nums.next() {
        Some(Ok(num)) => num,
        _ => {
            println!("0");
            return;
        }
    };
    let k = match nums.next() {
        Some(Ok(num)) => num,
        _ => {
            println!("0");
            return;
        }
    };

    if n < k {
        println!("0");
        return;
    }

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        match line.trim().parse::<usize>() {
            Ok(num) => a.push(num),
            Err(_) => {
                println!("0");
                return;
            }
        }
    }

    let mut f = vec![0; k + 1];
    let mut s = 0; // Count of unique integers from 1 to K
    let mut ans = n + 1; // Initialize to a value larger than any possible subarray length
    let (mut i, mut j) = (0, 0);

    // Expand the right boundary of the window
    while j < n {
        let t = a[j];
        if t <= k {
            if f[t] == 0 {
                s += 1;
            }
            f[t] += 1;
        }
        j += 1;

        // Check if we have a valid window
        while s == k {
            ans = ans.min(j - i);
            let t = a[i];
            if t <= k {
                f[t] -= 1;
                if f[t] == 0 {
                    s -= 1;
                }
            }
            i += 1;
        }
    }

    println!("{}", if ans == n + 1 { 0 } else { ans });
}
