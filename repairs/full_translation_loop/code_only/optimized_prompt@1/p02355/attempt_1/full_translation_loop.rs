use std::io::{self, BufRead};

fn gc() -> char {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.bytes().filter_map(Result::ok).map(|b| b as char).next().unwrap()
}

fn in_number() -> usize {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' && c <= '9' {
        n = 10 * n + (c as usize - '0' as usize);
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

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = in_number();
    }

    let mut f = vec![0; MAX_N];
    let mut s = 0;
    let mut i = 0;
    let mut j = 0;
    let mut ans = n + 1; // Start with a value larger than any possible answer

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
