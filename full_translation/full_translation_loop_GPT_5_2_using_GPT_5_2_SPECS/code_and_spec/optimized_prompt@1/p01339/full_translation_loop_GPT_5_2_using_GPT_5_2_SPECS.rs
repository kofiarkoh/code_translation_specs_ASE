use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read all input as bytes, parse non-negative integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let n = match it.next() {
        Some(v) => v,
        None => return, // no input
    };
    let m = it.next().unwrap_or(0);

    // Graph storage (safe Vec-based, instead of manual realloc in C).
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut rg: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let s = it.next().unwrap() - 1;
        let d = it.next().unwrap() - 1;
        g[s].push(d);
        rg[d].push(s);
    }

    // Kosaraju SCC: produces component indices in topological order
    // (as in the C code: second pass assigns increasing k in reverse finishing order).
    let mut used = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    // Iterative DFS to avoid recursion depth concerns.
    for start in 0..n {
        if used[start] {
            continue;
        }
        // stack of (v, next_edge_index)
        let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
        used[start] = true;
        while let Some((v, ei)) = stack.pop() {
            if ei < g[v].len() {
                // resume later
                stack.push((v, ei + 1));
                let to = g[v][ei];
                if !used[to] {
                    used[to] = true;
                    stack.push((to, 0));
                }
            } else {
                order.push(v);
            }
        }
    }

    let mut comp = vec![usize::MAX; n];
    used.fill(false);
    let mut k = 0usize;

    for &v in order.iter().rev() {
        if used[v] {
            continue;
        }
        // iterative reverse DFS
        let mut stack = vec![v];
        used[v] = true;
        comp[v] = k;
        while let Some(x) = stack.pop() {
            for &p in &rg[x] {
                if !used[p] {
                    used[p] = true;
                    comp[p] = k;
                    stack.push(p);
                }
            }
        }
        k += 1;
    }

    // Build next relation:
    // For each SCC i, next(i) is determined by the first outgoing edge of the
    // last vertex (in increasing vertex index order) within that SCC that has any outgoing edge.
    // This matches the C loop: for i=0..N-1 if (hi[i]) nx[cmp[i]] = cmp[to[i][0]];
    let mut nx: Vec<Option<usize>> = vec![None; k];
    for v in 0..n {
        if !g[v].is_empty() {
            let c = comp[v];
            let to0 = g[v][0];
            nx[c] = Some(comp[to0]);
        }
    }

    // DP aggregation pass in increasing SCC index order.
    let mut dp: Vec<i64> = vec![0; k];
    let mut ans: i64 = 1;

    for i in 0..k {
        if dp[i] != 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }

        match nx[i] {
            None => {
                ans = (ans * (dp[i] % MOD)) % MOD;
            }
            Some(j) if j == i => {
                ans = (ans * (dp[i] % MOD)) % MOD;
            }
            Some(j) => {
                if dp[j] != 0 {
                    dp[j] = (dp[j] * (dp[i] % MOD)) % MOD;
                } else {
                    dp[j] = dp[i] % MOD;
                }
            }
        }
    }

    println!("{}", ans % MOD);
}
