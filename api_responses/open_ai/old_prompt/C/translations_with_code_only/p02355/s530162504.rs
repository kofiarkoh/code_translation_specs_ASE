// AOJ DSL_3_B The Smallest Window II
// 2018.5.6 bal4u@uu

use std::io::{self, BufRead};

fn gc() -> char {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.bytes().next().unwrap().unwrap() as char
}

fn in_number() -> i32 {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' {
        n = 10 * n + (c as i32 - '0' as i32);
        c = gc();
    }
    n
}

const MAX_N: usize = 100003;

fn main() {
    let n = in_number();
    let k = in_number();
    
    if n < k {
        println!("0");
        return;
    }

    let mut a = vec![0; n as usize];
    let mut f = vec![0; MAX_N];
    
    for i in 0..n {
        a[i as usize] = in_number();
    }

    let mut s = 0;
    let mut i = 0;
    let mut j = 0;
    let mut ans = n + 1; // Start with a value larger than any possible window

    while s < k && j < n {
        let t = a[j as usize];
        j += 1;
        if t <= k {
            if f[t as usize] == 0 {
                s += 1;
            }
            f[t as usize] += 1;
        }
    }

    if s < k {
        println!("0");
        return;
    }

    while (a[i as usize] > k || f[a[i as usize] as usize] > 1) && i < j {
        if a[i as usize] <= k {
            f[a[i as usize] as usize] -= 1;
        }
        i += 1;
    }
    ans = j - i;

    while j < n {
        let t = a[j as usize];
        j += 1;
        if t <= k {
            f[t as usize] += 1;
        }
        while (a[i as usize] > k || f[a[i as usize] as usize] > 1) && i < j {
            if a[i as usize] <= k {
                f[a[i as usize] as usize] -= 1;
            }
            i += 1;
        }
        if j - i < ans {
            ans = j - i;
        }
    }
    
    println!("{}", ans);
}
