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

    // The sample indicates the intended global multiplier is (n-1)! (not n!),
    // while the per-vertex factor is still product of inverses of component sizes.
    // This matches: ans(v) = (n-1)! / prod_i s_i (mod MOD).
    let mut fact: u64 = 1;
    for i in 1..=((n.saturating_sub(1)) as u64) {
        fact = (fact * (i % MOD)) % MOD;
    }

    if n == 0 {
        return;
    }

    // Root at 1; compute parent and subtree sizes.
    let root = 1usize;
    let mut parent = vec![0usize; n + 1];
    let mut order: Vec<usize> = Vec::with_capacity(n);

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

    let mut out = String::new();
    for v in 1..=n {
        let mut prod_inv: u64 = 1;

        // Child-side components
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            prod_inv = (prod_inv * mod_inv(sub[to] as u64)) % MOD;
        }

        // Parent-side component (if any)
        if v != root {
            let s = (n - sub[v]) as u64;
            prod_inv = (prod_inv * mod_inv(s)) % MOD;
        }

        let ans = (fact * prod_inv) % MOD;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
