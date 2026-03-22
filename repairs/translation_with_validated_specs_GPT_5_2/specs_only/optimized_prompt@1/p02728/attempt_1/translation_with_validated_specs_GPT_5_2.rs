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

fn mod_inv(a: u64) -> u64 {
    // MOD is prime; caller assumes a != 0 (mod MOD) as per specification.
    mod_pow(a % MOD, MOD - 2)
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for _ in 0..n.saturating_sub(1) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        g[a].push(b);
        g[b].push(a);
    }

    // Precompute factorial n! mod MOD
    let mut fact_n: u64 = 1;
    for i in 1..=n as u64 {
        fact_n = (fact_n * i) % MOD;
    }

    if n == 1 {
        // Removing the only vertex leaves no components; empty product = 1; 1! = 1.
        println!("1");
        return;
    }

    // Root the tree at 1 and compute parent and order (iterative DFS).
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

    // Compute subtree sizes in reverse order.
    let mut sub = vec![0usize; n + 1];
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

    // For each vertex v, compute product of inverses of component sizes after removing v:
    // components are: each child subtree size sub[child], and the "rest of tree" size n - sub[v] (if > 0).
    let mut prod_inv = vec![1u64; n + 1];
    for v in 1..=n {
        let mut p = 1u64;
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            p = (p * mod_inv(sub[to] as u64)) % MOD;
        }
        let rest = n - sub[v];
        if rest > 0 {
            p = (p * mod_inv(rest as u64)) % MOD;
        }
        prod_inv[v] = p;
    }

    // Output n! * prod_inv[v] mod MOD for v=1..n, one per line.
    let mut out = String::new();
    for v in 1..=n {
        let ans = (fact_n * prod_inv[v]) % MOD;
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}
