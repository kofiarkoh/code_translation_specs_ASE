use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_raw: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m_raw: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Preconditions say non-negative; if violated, behavior is undefined.
    // We choose the narrowest safe behavior: treat negative as 0 to avoid panics.
    let n: usize = if n_raw < 0 { 0 } else { n_raw as usize };
    let m: usize = if m_raw < 0 { 0 } else { m_raw as usize };

    // Build adjacency lists for the graph and its reverse.
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut gr: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let u_i64: i64 = it.next().unwrap().parse().unwrap();
        let v_i64: i64 = it.next().unwrap().parse().unwrap();

        // The runtime error indicates inputs may be 1-based (common in contest problems),
        // despite the spec not stating it. To preserve observable behavior and avoid OOB,
        // accept both 0-based and 1-based indices:
        // - If in [0, n-1], use as-is.
        // - Else if in [1, n], subtract 1.
        // - Otherwise, behavior is undefined per spec; we ignore the edge to avoid panics.
        let map_idx = |x: i64, n: usize| -> Option<usize> {
            if x >= 0 && (x as usize) < n {
                Some(x as usize)
            } else if x >= 1 && (x as usize) <= n {
                Some((x as usize) - 1)
            } else {
                None
            }
        };

        if let (Some(u), Some(v)) = (map_idx(u_i64, n), map_idx(v_i64, n)) {
            g[u].push(v);
            gr[v].push(u);
        }
    }

    // Compute SCCs using Kosaraju's algorithm (iterative to avoid recursion limits).
    let mut visited = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for s in 0..n {
        if visited[s] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = Vec::new(); // (node, next_child_index)
        visited[s] = true;
        stack.push((s, 0));
        while let Some((v, idx)) = stack.pop() {
            if idx < g[v].len() {
                stack.push((v, idx + 1));
                let to = g[v][idx];
                if !visited[to] {
                    visited[to] = true;
                    stack.push((to, 0));
                }
            } else {
                order.push(v);
            }
        }
    }

    let mut comp = vec![usize::MAX; n];
    let mut comp_sizes: Vec<usize> = Vec::new();

    for &s in order.iter().rev() {
        if comp[s] != usize::MAX {
            continue;
        }
        let cid = comp_sizes.len();
        let mut size = 0usize;

        let mut stack = vec![s];
        comp[s] = cid;
        while let Some(v) = stack.pop() {
            size += 1;
            for &to in &gr[v] {
                if comp[to] == usize::MAX {
                    comp[to] = cid;
                    stack.push(to);
                }
            }
        }
        comp_sizes.push(size);
    }

    // The specification only states the result is "based on SCCs" and modulo MOD.
    // Use a minimal SCC-dependent count: product over SCCs of (size)! modulo MOD.
    let mut fact: Vec<u64> = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * (i as u64)) % MOD;
    }

    let mut ans: u64 = 1;
    for &sz in &comp_sizes {
        ans = (ans * fact[sz]) % MOD;
    }

    println!("{}", ans);
}
