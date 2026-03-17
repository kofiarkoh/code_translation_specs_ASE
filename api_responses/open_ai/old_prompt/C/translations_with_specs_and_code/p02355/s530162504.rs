use std::io::{self, BufRead};

const MAX_N: usize = 100_003;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let first_line = input.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    if n < k {
        println!("0");
        return;
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = input.next().unwrap().unwrap().parse().unwrap();
    }

    let mut f = vec![0; MAX_N];
    let mut s = 0; // Count of unique numbers from 1 to K
    let mut i = 0; // Start of the window
    let mut j = 0; // End of the window
    let mut ans = n + 1; // Initialize to a value larger than any possible subarray length

    // Expand the window to the right
    while s < k && j < n {
        let t = a[j];
        if t <= k {
            if f[t] == 0 {
                s += 1;
            }
            f[t] += 1;
        }
        j += 1;
    }

    // If we didn't find a valid window
    if s < k {
        println!("0");
        return;
    }

    // Contract the window from the left
    while (a[i] > k || f[a[i]] > 1) && i < j {
        if a[i] <= k {
            f[a[i]] -= 1;
        }
        i += 1;
    }
    ans = j - i;

    // Continue expanding the window to the right
    while j < n {
        let t = a[j];
        if t <= k {
            f[t] += 1;
        }
        j += 1;

        // Contract the window from the left
        while (a[i] > k || f[a[i]] > 1) && i < j {
            if a[i] <= k {
                f[a[i]] -= 1;
            }
            i += 1;
        }

        // Update the answer if we found a smaller valid window
        if j - i < ans {
            ans = j - i;
        }
    }

    println!("{}", ans);
}
