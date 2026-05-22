use std::io::{self, Read};

fn main() {
    // Fast input: read all bytes, parse signed integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse::<i64>().unwrap() as usize,
        None => {
            // No input: undefined by spec; do nothing.
            return;
        }
    };

    let mut weight = vec![0i64; n];
    let mut parent = vec![0i64; n];
    let mut child_count = vec![0usize; n];
    let mut roots: Vec<usize> = Vec::new();

    for i in 0..n {
        let w = it.next().unwrap().parse::<i64>().unwrap();
        let p = it.next().unwrap().parse::<i64>().unwrap();
        weight[i] = w;
        parent[i] = p;
        if p == -1 {
            roots.push(i);
        } else {
            let pu = p as usize;
            child_count[pu] += 1;
        }
    }

    // Build adjacency lists (children).
    let mut children: Vec<Vec<usize>> = (0..n)
        .map(|i| Vec::with_capacity(child_count[i]))
        .collect();
    for i in 0..n {
        let p = parent[i];
        if p != -1 {
            children[p as usize].push(i);
        }
    }

    // DP arrays.
    let mut s = vec![0i64; n]; // subtree weight sum
    let mut dp = vec![0i64; n]; // subtree cost
    let mut next_child_idx = vec![0usize; n]; // like a[] in C, per-root traversal

    // Iterative postorder traversal per root (matches C behavior; avoids recursion depth issues).
    let mut stack: Vec<usize> = Vec::new();
    let mut buf: Vec<i64> = Vec::new(); // reused buffer for sorting child subtree sums

    let mut ans: i64 = 0;
    for &r in &roots {
        stack.clear();
        stack.push(r);

        while let Some(&u) = stack.last() {
            if next_child_idx[u] < children[u].len() {
                let v = children[u][next_child_idx[u]];
                next_child_idx[u] += 1;
                stack.push(v);
            } else {
                // calc(u): compute s[u] and dp[u] from children (already computed).
                let mut su = weight[u];
                let mut dpu = 0i64;

                buf.clear();
                for &v in &children[u] {
                    buf.push(s[v]);
                    su += s[v];
                    dpu += dp[v];
                }

                // Minimized by sorting child subtree sums in non-increasing order.
                buf.sort_unstable_by(|a, b| b.cmp(a));

                for (i, &val) in buf.iter().enumerate() {
                    dpu += val * (i as i64);
                }

                s[u] = su;
                dp[u] = dpu;

                stack.pop();
            }
        }

        ans += dp[r];
    }

    print!("{ans}\n");
}
