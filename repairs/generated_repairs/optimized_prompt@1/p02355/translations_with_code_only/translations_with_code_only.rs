// AOJ DSL_3_B The Smallest Window II
// 2018.5.6 bal4u@uu

use std::io::{self, BufRead, Read};

fn in_number() -> usize {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut number = 0;
    let mut c = handle.bytes().next().unwrap().unwrap();
    
    while c >= b'0' {
        number = 10 * number + (c - b'0') as usize;
        c = handle.bytes().next().unwrap().unwrap();
    }
    
    number
}

const MAX_N: usize = 100003;

fn main() {
    let n = in_number();
    let k = in_number();
    
    if n < k {
        println!("0");
        return;
    }
    
    let mut a = vec![0; n];
    let mut f = vec![0; MAX_N];
    
    for i in 0..n {
        a[i] = in_number();
    }

    let mut s = 0;
    let mut i = 0;
    let mut j = 0;
    let mut ans = n + 1; // Start with a value larger than any possible window size

    while s < k && j < n {
        let t = a[j];
        j += 1;
        if t <= k {
            if f[t] == 0 {
                s += 1;
            }
            f[t] += 1;
        }
    }
    
    if s < k {
        println!("0");
        return;
    }
    
    while (a[i] > k || f[a[i]] > 1) && i < n {
        if a[i] <= k {
            f[a[i]] -= 1;
        }
        i += 1;
    }
    
    ans = j - i;

    while j < n {
        let t = a[j];
        j += 1;
        if t <= k {
            f[t] += 1;
        }
        while (a[i] > k || f[a[i]] > 1) && i < n {
            if a[i] <= k {
                f[a[i]] -= 1;
            }
            i += 1;
        }
        if j - i < ans {
            ans = j - i;
        }
    }
    
    println!("{}", ans);
}
