use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    a %= MOD;
    while e > 0 {
        if (e & 1) == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(x: u64) -> u64 {
    // MOD is prime; preconditions guarantee x != 0 (mod MOD) when inverted.
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

    // n! modulo MOD
    let mut fact_n: u64 = 1;
    for i in 1..=n {
        fact_n = fact_n * (i as u64) % MOD;
    }

    // Build parent array and traversal order from an arbitrary root (1).
    let root = 1usize;
    let mut parent = vec![0usize; n + 1];
    let mut order = Vec::with_capacity(n);

    let mut stack = Vec::with_capacity(n);
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

    // Subtree sizes when rooted at 1.
    let mut sub = vec![1usize; n + 1];
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

    // The correct combinatorial value per root r is:
    // ans[r] = n! * Π_u inv(size(subtree_u when rooted at r))  (mod MOD).
    //
    // Let F[r] = Π_u inv(size(subtree_u when rooted at r)) (mod MOD).
    // Then ans[r] = n! * F[r] mod MOD.
    //
    // Reroot across edge u-v where v is a child of u in the current rooting:
    // Only subtree sizes of u and v change:
    //   size_v becomes n
    //   size_u becomes n - size_v(old)
    // Thus:
    //   F[v] = F[u] * inv(n) * inv(n - sub[v]) / (inv(sub[v]) * inv(n))
    //        = F[u] * sub[v] * inv(n - sub[v])   (mod MOD)
    //
    // (All other factors cancel; inv(n) cancels as well.)
    let mut f = vec![1u64; n + 1];
    let mut f_root = 1u64;
    for u in 1..=n {
        f_root = f_root * mod_inv(sub[u] as u64) % MOD;
    }
    f[root] = f_root;

    // Propagate F values from root to children using the fixed parent[] orientation.
    let mut stack2 = Vec::with_capacity(n);
    stack2.push(root);
    while let Some(u) = stack2.pop() {
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            let sv = sub[v] as u64 % MOD;
            let n_minus_sv = (n - sub[v]) as u64 % MOD;
            // F[v] = F[u] * sub[v] / (n - sub[v]) mod MOD
            f[v] = f[u] * sv % MOD * mod_inv(n_minus_sv) % MOD;
            stack2.push(v);
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        let ans = fact_n * f[i] % MOD;
        out.push_str(&format!("{ans}\n"));
    }
    print!("{out}");
}
