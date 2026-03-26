// AOJ 2807 Fractal Tree
// 2018.3.21 bal4u

use std::io::{self, Read};

const MAX: usize = 100005;

#[derive(Debug, Clone)]
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
    let mut handle = stdin.lock();
    let mut buf = [0; 1];
    handle.read(&mut buf).unwrap();
    buf[0] as char
}

fn in_non_negative() -> usize {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' {
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
    
    while c >= '0' {
        n = 10 * n + (c as usize - '0' as usize);
        c = gc();
    }

    if c == '.' {
        c = gc();
        while c >= '0' {
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

fn dfs(hi: &mut Vec<usize>, t: &Vec<Vec<T>>, depth: &mut Vec<usize>, cost: &mut Vec<i32>, depth_max: &mut usize) {
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
    let mut p: Vec<f64> = vec![0.0; MAX];
    
    let p_value = dbl();
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
        let k = hi[x] - 1; // Use the last index
        t[x][k].to = y;
        t[x][k].c = c;
        
        hi[x] -= 1; // Decrement to use the next available index
        
        let k = hi[y] - 1; // Use the last index
        t[y][k].to = x;
        t[y][k].c = c;
        
        hi[y] -= 1; // Decrement to use the next available index
        
        i += 3;
    }

    dfs(&mut hi, &t, &mut depth, &mut cost, &mut depth_max);
    
    p[0] = 1.0;
    for i in 1..=depth_max {
        p[i] = p[i - 1] * p_value;
    }
    
    let mut a = 0.0;
    let mut b = 1.0;
    for i in 1..=n {
        a += cost[i] as f64 * p[depth[i]];
        b += p[depth[i]];
    }
    
    println!("{:.10}", a * b);
}
