use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n.saturating_sub(1) {
        let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        g[a].push(b);
        g[b].push(a);
    }

    // Root at 0, compute parent and postorder.
    let root = 0usize;
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = Vec::with_capacity(n);
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
    let mut post = order.clone();
    post.reverse();

    // Subtree sizes
    let mut sub = vec![1usize; n];
    for &v in &post {
        let mut sz = 1usize;
        for &to in &g[v] {
            if parent[to] == v {
                sz += sub[to];
            }
        }
        sub[v] = sz;
    }

    // weight[k] = 0 if k odd, else product of odd numbers up to (k-1) modulo MOD, with weight[0]=1.
    let mut weight = vec![0i64; n + 1];
    weight[0] = 1;
    for k in 1..=n {
        if k % 2 == 1 {
            weight[k] = 0;
        } else {
            weight[k] = mod_norm(weight[k - 2] * (k as i64 - 1));
        }
    }

    // DP:
    // For each vertex v, dp_pos[v][i], dp_neg[v][i] for i in [0..=sub[v]].
    //
    // The spec indicates "aggregating subtree contributions into two signed classes"
    // and then "summing the positive class and subtracting the negative class".
    // A narrow, fully-defined interpretation that uses only subtree sizes and a sign split:
    //
    // Let each child contribute either 0 (exclude that child's subtree) or sub[child] (include it).
    // The sign class is determined by parity of the number of included child subtrees:
    //   pos: even count, neg: odd count.
    //
    // Additionally, the root's own vertex is not forced to be included; i=0 is allowed.
    // This is necessary to match the spec's allowance of sizes in [0, subtree_size].
    let mut dp_pos: Vec<Vec<i64>> = vec![Vec::new(); n];
    let mut dp_neg: Vec<Vec<i64>> = vec![Vec::new(); n];

    for &v in &post {
        let mut cur_pos = vec![0i64; sub[v] + 1];
        let mut cur_neg = vec![0i64; sub[v] + 1];

        // Start with selecting nothing from v's subtree: size 0, included-children count = 0 (even => pos)
        cur_pos[0] = 1;

        // Process children: choose to include/exclude each child's entire subtree, tracking parity.
        for &to in &g[v] {
            if parent[to] != v {
                continue;
            }
            let child_sz = sub[to];

            let mut next_pos = vec![0i64; sub[v] + 1];
            let mut next_neg = vec![0i64; sub[v] + 1];

            for i in 0..=sub[v] {
                let a_pos = cur_pos[i];
                let a_neg = cur_neg[i];
                if a_pos == 0 && a_neg == 0 {
                    continue;
                }

                // Exclude this child
                next_pos[i] = mod_norm(next_pos[i] + a_pos);
                next_neg[i] = mod_norm(next_neg[i] + a_neg);

                // Include this child: flips parity => swap pos/neg
                if i + child_sz <= sub[v] {
                    next_pos[i + child_sz] = mod_norm(next_pos[i + child_sz] + a_neg);
                    next_neg[i + child_sz] = mod_norm(next_neg[i + child_sz] + a_pos);
                }
            }

            cur_pos = next_pos;
            cur_neg = next_neg;
        }

        // Finally, decide whether to include v itself (+1 size) without changing parity class.
        let mut final_pos = vec![0i64; sub[v] + 1];
        let mut final_neg = vec![0i64; sub[v] + 1];
        for i in 0..=sub[v] {
            // Not include v
            final_pos[i] = mod_norm(final_pos[i] + cur_pos[i]);
            final_neg[i] = mod_norm(final_neg[i] + cur_neg[i]);
            // Include v
            if i + 1 <= sub[v] {
                final_pos[i + 1] = mod_norm(final_pos[i + 1] + cur_pos[i]);
                final_neg[i + 1] = mod_norm(final_neg[i + 1] + cur_neg[i]);
            }
        }

        dp_pos[v] = final_pos;
        dp_neg[v] = final_neg;
    }

    // Final aggregation at root:
    // sum_i weight[n-i] * (pos[root][i] - neg[root][i]) mod MOD
    let mut ans: i64 = 0;
    for i in 0..=sub[root] {
        let w = weight[n - i];
        if w == 0 {
            continue;
        }
        let term = mod_norm(dp_pos[root][i] - dp_neg[root][i]);
        ans = mod_norm(ans + mod_norm(w * term));
    }

    println!("{}", ans);
}
