use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(x: u64) -> u64 {
    // MOD is prime; inverse exists per specification assumptions.
    mod_pow(x % MOD, MOD - 2)
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for _ in 0..n.saturating_sub(1) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        g[a].push(b);
        g[b].push(a);
    }

    // Precompute factorial n! mod MOD (global multiplier)
    let mut fact_n: u64 = 1;
    for i in 1..=n as u64 {
        fact_n = (fact_n * (i % MOD)) % MOD;
    }

    if n == 0 {
        // Not expected by spec (n >= 1), but keep behavior minimal.
        return;
    }

    // Root the tree at 1 and compute parent + subtree sizes.
    let root = 1usize;
    let mut parent = vec![0usize; n + 1];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    // Iterative DFS to avoid recursion limits.
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    parent[root] = root;
    stack.push(root);
    while let Some(v) = stack.pop() {
        order.push(v);
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            parent[to] = v;
            stack.push(to);
        }
    }

    // Post-order for subtree sizes.
    let mut sub = vec![1usize; n + 1];
    for &v in order.iter().rev() {
        let mut sz = 1usize;
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            sz += sub[to];
        }
        sub[v] = sz;
    }

    // For each vertex v, removing v yields components:
    // - for each child c of v: size = sub[c]
    // - if v is not root: size = n - sub[v] (the "parent side")
    // We need product of inverses of these sizes, then multiply by n!.
    let mut out = String::new();
    for v in 1..=n {
        let mut prod_inv: u64 = 1;

        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            let s = sub[to] as u64;
            prod_inv = (prod_inv * mod_inv(s)) % MOD;
        }

        if v != root {
            let s = (n - sub[v]) as u64;
            // For n=1, this branch never triggers; for n>1, s>=1.
            prod_inv = (prod_inv * mod_inv(s)) % MOD;
        }

        let ans = (fact_n * prod_inv) % MOD;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
