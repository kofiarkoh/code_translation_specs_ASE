// AOJ 2456: Usoperanto
// Translated from C to Rust

use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX: usize = 1_000_002;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
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

        if memo[i] == usize::MAX { // -1 in C
            base.push(i);
        } else {
            hi[memo[i]] += 1;
        }
    }

    let mut dp = vec![0; n];
    let mut s = vec![0; n];

    for i in 0..n {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    let mut bsz = 0;
    for i in 0..n {
        let l = memo[i];
        if l == usize::MAX { // -1 in C
            bsz += 1;
        } else {
            to[l][hi[l] - 1] = i;
            hi[l] -= 1;
        }
    }

    for &root in &base {
        let mut que = VecDeque::new();
        let mut a = vec![0; n];
        que.push_back(root);

        while let Some(t) = que.pop_back() {
            if a[t] < hi[t] {
                que.push_back(to[t][a[t]]);
                a[t] += 1;
            } else {
                calc(t, &m, &mut s, &mut dp, &to);
            }
        }
        ans += dp[root];
    }

    println!("{}", ans);
}

fn calc(u: usize, m: &[i64], s: &mut [i64], dp: &mut [i64], to: &[Vec<usize>]) {
    let mut sz = 0;
    s[u] = m[u];
    dp[u] = 0;

    let mut b = vec![0; to[u].len()];

    for &v in &to[u] {
        b[sz] = s[v];
        s[u] += s[v];
        dp[u] += dp[v];
        sz += 1;
    }

    b.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

    for i in 0..sz {
        dp[u] += b[i] * i as i64;
    }
}
```

### Notes:
1. **Input Handling**: The input is read using `BufRead` for efficiency, and parsing is done using `split_whitespace` to handle multiple integers in a single line.
2. **Dynamic Arrays**: Rust's `Vec` is used to handle dynamic arrays instead of manual memory management as in C.
3. **Sorting**: The sorting is done using `sort_unstable_by` to sort in descending order, which matches the behavior of the C `qsort` function.
4. **Error Handling**: The Rust code assumes well-formed input as per the specification, similar to the C code.
5. **Memory Safety**: The code avoids `unsafe` blocks by using safe Rust constructs, ensuring memory safety throughout.
