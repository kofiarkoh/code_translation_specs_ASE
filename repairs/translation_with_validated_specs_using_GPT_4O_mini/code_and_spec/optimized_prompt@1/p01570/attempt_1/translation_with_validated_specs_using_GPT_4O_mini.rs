use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX: usize = 1_000_002;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut m = vec![0; n];
    let mut memo = vec![0; n];
    let mut hi = vec![0; n];
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut base = Vec::new();
    let mut ans: i64 = 0;

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        m[i] = parts[0];
        memo[i] = parts[1] as usize;
        if memo[i] == usize::MAX {
            base.push(i);
        } else {
            hi[memo[i]] += 1;
        }
    }

    for i in 0..n {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    let mut bsz = 0;
    for i in 0..n {
        let l = memo[i];
        if l == usize::MAX {
            base.push(i);
        } else {
            to[l][hi[l] - 1] = i;
            hi[l] -= 1;
        }
    }

    for &root in &base {
        let mut queue = VecDeque::new();
        let mut a = vec![0; n];
        let mut dp = vec![0; n];
        let mut s = vec![0; n];

        queue.push_back(root);
        while let Some(t) = queue.pop_back() {
            if a[t] < hi[t] {
                queue.push_back(to[t][a[t]]);
                a[t] += 1;
            } else {
                calc(t, &m, &to, &mut s, &mut dp, hi[t]);
            }
        }
        ans += dp[root];
    }

    println!("{}", ans);
}

fn calc(u: usize, m: &[i64], to: &[Vec<usize>], s: &mut [i64], dp: &mut [i64], hi: usize) {
    let mut b = Vec::new();
    s[u] = m[u];
    dp[u] = 0;

    for &v in &to[u] {
        b.push(s[v]);
        s[u] += s[v];
        dp[u] += dp[v];
    }

    b.sort_by(|a, b| b.cmp(a)); // Sort in descending order
    for (i, &value) in b.iter().enumerate() {
        dp[u] += value * i as i64;
    }
}
```

### Notes:
1. **Input Handling**: The C code uses `getchar_unlocked()` for fast input, which is not directly available in Rust. Instead, I used `BufRead` to read input efficiently.
2. **Memory Management**: The C code uses dynamic memory allocation with `malloc`. In Rust, we use vectors which handle memory automatically.
3. **Error Handling**: The Rust code does not explicitly handle memory allocation failures, as per the specification. Rust's ownership model generally prevents many common memory issues.
4. **Sorting**: The sorting in the `calc` function is done in descending order to match the behavior of the C code.
5. **Indexing**: The C code uses `-1` to indicate a root node, which is translated to `usize::MAX` in Rust for clarity and safety.
6. **Output**: The output format is preserved exactly as specified.
