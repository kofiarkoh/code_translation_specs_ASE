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
    // MOD is prime; preconditions guarantee x != 0 (mod MOD) when inverted.
    mod_pow(x % MOD, MOD - 2)
}

fn main() {
    // Input: n, then n-1 undirected edges.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for _ in 0..(n.saturating_sub(1)) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        g[a].push(b);
        g[b].push(a);
    }

    // Precompute factorials and inverse factorials up to n.
    let mut fact: Vec<u64> = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * (i as u64)) % MOD;
    }

    // Root the tree at 1 for initial DP.
    let root = 1usize;
    let mut parent: Vec<usize> = vec![0; n + 1];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    // Iterative DFS to avoid recursion depth issues.
    let mut stack: Vec<usize> = Vec::with_capacity(n);
    stack.push(root);
    parent[root] = root;
    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            stack.push(v);
        }
    }

    // Compute subtree sizes for root=1 using reverse order.
    let mut sub: Vec<usize> = vec![1; n + 1];
    for &u in order.iter().rev() {
        let mut sz = 1usize;
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            sz += sub[v];
        }
        sub[u] = sz;
    }

    // For a fixed root r, answer is:
    // ans[r] = n! / (product over all nodes u of size(subtree_u_when_rooted_at_r)) mod MOD.
    //
    // Let P[r] = product of subtree sizes under root r (mod MOD).
    // Then ans[r] = fact[n] * inv(P[r]) mod MOD.
    //
    // Reroot transition across edge u-v (v is child of u in current rooting):
    // When moving root from u to v:
    // - subtree size of u becomes n - sub[v]
    // - subtree size of v becomes n
    // - all other nodes' subtree sizes unchanged
    // Therefore:
    // P[v] = P[u] * (n - sub[v]) * n / (sub[v] * n) = P[u] * (n - sub[v]) / sub[v]  (mod MOD)
    // (the n factors cancel; we keep the simplified form).
    //
    // This uses modular division via inverse; preconditions guarantee validity.
    let mut prod: Vec<u64> = vec![1; n + 1];
    let mut p_root = 1u64;
    for u in 1..=n {
        p_root = (p_root * (sub[u] as u64 % MOD)) % MOD;
    }
    prod[root] = p_root;

    // Propagate prod values with a second DFS in parent->child order.
    let mut stack2: Vec<usize> = Vec::with_capacity(n);
    stack2.push(root);
    while let Some(u) = stack2.pop() {
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            // v is child of u in the initial rooting; sub[v] is its subtree size under root=u.
            let sv = sub[v] as u64 % MOD;
            let n_minus_sv = ((n - sub[v]) as u64) % MOD;
            // prod[v] = prod[u] * (n - sub[v]) / sub[v] mod MOD
            let val = prod[u]
                .wrapping_mul(n_minus_sv) % MOD
                .wrapping_mul(mod_inv(sv)) % MOD;
            prod[v] = val;
            stack2.push(v);
        }
    }

    // Output answers for nodes 1..n, one per line.
    let mut out = String::new();
    for i in 1..=n {
        let ans = fact[n] * mod_inv(prod[i]) % MOD;
        out.push_str(&format!("{ans}\n"));
    }
    print!("{out}");
}
