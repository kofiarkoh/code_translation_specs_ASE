use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(x: u64) -> u64 {
    // MOD is prime; caller assumes x != 0 (mod MOD).
    mod_pow(x % MOD, MOD - 2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n.saturating_sub(1) {
        let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        g[a].push(b);
        g[b].push(a);
    }

    if n == 1 {
        println!("1");
        return;
    }

    // Root at 0 to compute subtree sizes.
    let root = 0usize;
    let mut parent: Vec<usize> = vec![usize::MAX; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    parent[root] = root;
    stack.push(root);
    while let Some(v) = stack.pop() {
        order.push(v);
        for &to in &g[v] {
            if parent[to] == usize::MAX {
                parent[to] = v;
                stack.push(to);
            }
        }
    }

    let mut sub: Vec<usize> = vec![1; n];
    for &v in order.iter().rev() {
        let mut sz = 1usize;
        for &to in &g[v] {
            if parent[to] == v {
                sz += sub[to];
            }
        }
        sub[v] = sz;
    }

    // Precompute inverses inv[k] for k=1..=n
    let mut inv: Vec<u64> = vec![0; n + 1];
    for k in 1..=n {
        inv[k] = mod_inv(k as u64);
    }

    // Based on the sample behavior, the intended formula is:
    //   (n-1)! / Π component_sizes   (mod MOD)
    // where component sizes are those after removing v.
    let mut fact_n_minus_1: u64 = 1;
    for i in 1..=(n as u64 - 1) {
        fact_n_minus_1 = (fact_n_minus_1 * (i % MOD)) % MOD;
    }

    // For each vertex v, removing v yields components:
    // - for each child c of v: size = sub[c]
    // - plus (if v != root): parent-side size = n - sub[v]
    //
    // IMPORTANT: this is one component per incident edge, but for v the parent-side
    // component appears only once (not once per neighbor).
    let mut out = String::new();
    for v in 0..n {
        let mut prod = 1u64;

        // child-side components
        for &to in &g[v] {
            if parent[to] == v {
                prod = (prod * inv[sub[to]]) % MOD;
            }
        }

        // parent-side component (single)
        if v != root {
            let ps = n - sub[v];
            prod = (prod * inv[ps]) % MOD;
        }

        let ans = (fact_n_minus_1 * prod) % MOD;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
