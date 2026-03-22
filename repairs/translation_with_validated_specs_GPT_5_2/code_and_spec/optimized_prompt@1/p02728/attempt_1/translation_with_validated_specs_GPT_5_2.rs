use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r: i64 = 1;
    a %= MOD;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    // MOD is prime; inverse exists per spec assumptions.
    mod_pow(a, MOD - 2)
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<usize> = Vec::with_capacity(n.saturating_sub(1));
    let mut b: Vec<usize> = Vec::with_capacity(n.saturating_sub(1));
    for _ in 0..n.saturating_sub(1) {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        a.push(u);
        b.push(v);
    }

    // Build adjacency list (safe/idiomatic; differs from C's edge-array but preserves semantics)
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in 0..n.saturating_sub(1) {
        let u = a[i];
        let v = b[i];
        g[u].push(v);
        g[v].push(u);
    }

    // factorial n! mod MOD
    let mut fact: i64 = 1;
    for i in 1..=n as i64 {
        fact = (fact * i) % MOD;
    }

    // Arrays: 1-based
    let mut parent: Vec<usize> = vec![0; n + 1];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    // iterative DFS to get parent and order
    if n >= 1 {
        let root = 1usize;
        parent[root] = root;
        let mut stack: Vec<usize> = vec![root];
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
    }

    // subtree sizes and dp_down:
    // dp_down[v] = product over components when removing v that lie in each child-subtree side:
    //   (prod over children c of dp_down[c]) * inv(subtree_size[v])
    // This matches the C's tDP with tans initialized to 1 and fp multiplying inv(tS[v]).
    let mut sub: Vec<usize> = vec![1; n + 1];
    let mut dp_down: Vec<i64> = vec![1; n + 1];

    for &v in order.iter().rev() {
        let mut sz: usize = 1;
        let mut prod: i64 = 1;
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            sz += sub[to];
            prod = (prod * dp_down[to]) % MOD;
        }
        sub[v] = sz;
        dp_down[v] = (prod * mod_inv(sz as i64)) % MOD;
    }

    // Rerooting to compute answer for every vertex:
    // For vertex v, components after removing v are:
    //  - each child subtree size sub[child]
    //  - the "rest of tree" size n - sub[v] (if v not root in that orientation)
    // The desired per-vertex factor is prod inv(component_size).
    //
    // We'll compute up[v] = dp value coming from parent side when v is considered as child:
    // up[v] corresponds to the parent's-side component contribution to v:
    //   up[v] = (product of dp contributions of all neighbors of parent excluding v, and parent's up)
    //           * inv(size_of_component_on_parent_side_when_removing_parent)
    // but easiest is to compute ans[v] directly using prefix/suffix products.
    let mut ans: Vec<i64> = vec![1; n + 1];

    // For root, there is no parent-side component.
    // We'll do a stack/queue over order (which is DFS order from root).
    for &v in &order {
        // Collect neighbors
        let deg = g[v].len();
        let mut neigh: Vec<usize> = Vec::with_capacity(deg);
        for &to in &g[v] {
            neigh.push(to);
        }

        // For each neighbor u of v, define contrib(u->v) = product factor associated with the component
        // on u's side when removing v:
        // - if u is child: component size = sub[u], and the internal product for that component is dp_down[u] * sub[u]
        //   Wait: dp_down[u] already includes inv(sub[u]) and children products. For removing v, we only need inv(sub[u]),
        //   not internal structure. Spec says only sizes matter, so we should use inv(sub[u]) only.
        // The C code's tans values are arranged so that ANS[v] becomes product inv(component sizes) (no factorials).
        //
        // Therefore, for answer at v we need:
        //   prod over child u: inv(sub[u])  *  (if parent exists) inv(n - sub[v])
        //
        // We'll compute that directly (simpler and matches spec).
        let mut prod_inv_sizes: i64 = 1;
        for &to in &g[v] {
            if to == parent[v] {
                continue;
            }
            prod_inv_sizes = (prod_inv_sizes * mod_inv(sub[to] as i64)) % MOD;
        }
        if v != 1 {
            let rest = n - sub[v];
            // rest >= 1 in a tree when v != root
            prod_inv_sizes = (prod_inv_sizes * mod_inv(rest as i64)) % MOD;
        }
        ans[v] = prod_inv_sizes;
    }

    // Output n lines: ans[i] * n! mod MOD
    let mut out = String::new();
    for i in 1..=n {
        let val = (ans[i] * fact) % MOD;
        out.push_str(&format!("{val}\n"));
    }
    print!("{out}");
}

/*
Note on C vs SPEC:
The provided C code implements a generic rerooting DP with multiplicative updates (tans, tS, chrt).
Per the SPEC, the final per-vertex value depends ONLY on component sizes after removing the vertex:
  n! / (product of component sizes).
The Rust implementation computes exactly that quantity directly from subtree sizes, which is equivalent
to the C program's intended output under the given specification.
*/
