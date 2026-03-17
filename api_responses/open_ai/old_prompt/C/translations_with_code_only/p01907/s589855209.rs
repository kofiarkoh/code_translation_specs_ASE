// AOJ 2807 Fractal Tree
// 2018.3.21 bal4u

use std::io::{self, BufRead};

const MAX: usize = 100005;

#[derive(Debug)]
struct T {
    to: usize,
    c: i32,
}

struct Q {
    n: usize,
    d: usize,
}

fn gc() -> char {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.bytes().filter_map(Result::ok).map(|b| b as char).next().unwrap()
}

fn in_non_negative() -> usize {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' && c <= '9' {
        n = 10 * n + (c as usize - '0' as usize);
        c = gc();
    }
    n
}

fn dbl() -> f64 {
    let mut minus = false;
    let mut x = 0.0;
    let mut y = 1.0;
    let mut n = 0;
    let mut c = gc();

    if c == '-' {
        minus = true;
        c = gc();
    }

    while c >= '0' && c <= '9' {
        n = 10 * n + (c as usize - '0' as usize);
        c = gc();
    }

    if c == '.' {
        c = gc();
        while c >= '0' && c <= '9' {
            y *= 0.1;
            x += y * (c as usize - '0' as usize) as f64;
            c = gc();
        }
        x += n as f64;
    } else {
        x = n as f64;
    }

    if minus {
        x = -x;
    }
    x
}

fn dfs(hi: &[usize], t: &[Vec<T>], depth: &mut [usize], cost: &mut [i32], depth_max: &mut usize) {
    let mut q: Vec<Q> = Vec::new();
    let mut vis = vec![false; MAX];
    q.push(Q { n: 1, d: 0 });

    while let Some(Q { n: node, d }) = q.pop() {
        if vis[node] {
            continue;
        }
        vis[node] = true;

        depth[node] = d;
        if d > *depth_max {
            *depth_max = d;
        }
        for i in 0..hi[node] {
            let to = t[node][i].to;
            if !vis[to] {
                cost[to] = t[node][i].c;
                q.push(Q { n: to, d: d + 1 });
            }
        }
    }
}

fn main() {
    let mut hi = vec![0; MAX];
    let mut t: Vec<Vec<T>> = vec![Vec::new(); MAX];
    let mut cost = vec![0; MAX];
    let mut depth = vec![0; MAX];
    let mut depth_max = 0;

    let p = dbl();
    let n = in_non_negative();

    let mut memo = Vec::with_capacity(3 * n);
    for _ in 1..n {
        let x = in_non_negative();
        let y = in_non_negative();
        let c = in_non_negative();
        memo.push(x);
        memo.push(y);
        memo.push(c);
        hi[x] += 1;
        hi[y] += 1;
    }

    for i in 1..=n {
        if hi[i] > 0 {
            t[i] = vec![T { to: 0, c: 0 }; hi[i]];
        }
    }

    let mut i = 0;
    while i < memo.len() {
        let x = memo[i];
        let y = memo[i + 1];
        let c = memo[i + 2];
        let k = hi[x];
        t[x][k].to = y;
        t[x][k].c = c;
        hi[x] += 1;

        let k = hi[y];
        t[y][k].to = x;
        t[y][k].c = c;
        hi[y] += 1;

        i += 3;
    }

    dfs(&hi, &t, &mut depth, &mut cost, &mut depth_max);

    let mut p_values = vec![1.0; depth_max + 1];
    for i in 1..=depth_max {
        p_values[i] = p_values[i - 1] * p;
    }

    let mut a = 0.0;
    let mut b = 1.0;
    for i in 1..=n {
        a += cost[i] as f64 * p_values[depth[i]];
        b += p_values[depth[i]];
    }

    println!("{:.10}", a * b);
}
