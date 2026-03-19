use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn calc(x: usize, pw: &[i64]) -> i64 {
    // Matches C macro: (x&1)?0:pw[x>>1]
    if (x & 1) == 1 {
        0
    } else {
        pw[x >> 1]
    }
}

fn add_mod(a: &mut i64, b: i64) {
    let mut v = *a + b;
    v %= MOD;
    if v < 0 {
        v += MOD;
    }
    *a = v;
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Adjacency list (safe, idiomatic replacement for C arrays)
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 1..n {
        let mut u: usize = it.next().unwrap().parse().unwrap();
        let mut v: usize = it.next().unwrap().parse().unwrap();
        u -= 1;
        v -= 1;
        g[u].push(v);
        g[v].push(u);
    }

    // pw[i] = product_{k=1..i} (2k-1) mod MOD, with pw[0]=1
    // C allocates pw[5005], but we only need up to n (and calc uses up to n/2).
    let mut pw = vec![0i64; n + 1];
    pw[0] = 1;
    for i in 1..=n {
        let term = (2 * i - 1) as i64;
        pw[i] = (pw[i - 1] * term) % MOD;
    }

    // Compute parent order by iterative DFS from root 0
    let root = 0usize;
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![root];
    parent[root] = root;
    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            if parent[v] == usize::MAX {
                parent[v] = u;
                stack.push(v);
            }
        }
    }

    // Subtree sizes in reverse order
    let mut sz = vec![1usize; n];
    for &u in order.iter().rev() {
        let mut s = 1usize;
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            s += sz[v];
        }
        sz[u] = s;
    }

    // dp[u][k][parity] stored as Vec<[i64;2]> of length (sz[u]+1) eventually.
    // dp_sz[u] tracks current maximum k used (like C's dp_sz).
    let mut dp: Vec<Vec<[i64; 2]>> = vec![Vec::new(); n];
    let mut dp_sz = vec![0usize; n];

    // Post-order DP
    for &u in order.iter().rev() {
        // Initialize dp[u][0][0] = 1
        dp[u] = vec![[0i64, 0i64]; sz[u] + 1];
        dp[u][0][0] = 1;
        dp_sz[u] = 0;

        // Merge children
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }

            // tmp sized to current dp_sz[u] + dp_sz[v] (max index), but we can allocate full sz[u]+1.
            let mut tmp = vec![[0i64, 0i64]; sz[u] + 1];

            for x in (0..=dp_sz[u]).rev() {
                let a0 = dp[u][x][0];
                let a1 = dp[u][x][1];
                if a0 == 0 && a1 == 0 {
                    continue;
                }
                for y in (0..=dp_sz[v]).rev() {
                    let b0 = dp[v][y][0];
                    let b1 = dp[v][y][1];
                    if b0 == 0 && b1 == 0 {
                        continue;
                    }
                    let idx = x + y;

                    // tmp[idx][0] += a0*b0 + a1*b1
                    let t0 = (a0 * b0 + a1 * b1) % MOD;
                    tmp[idx][0] = (tmp[idx][0] + t0) % MOD;

                    // tmp[idx][1] += a0*b1 + a1*b0
                    let t1 = (a0 * b1 + a1 * b0) % MOD;
                    tmp[idx][1] = (tmp[idx][1] + t1) % MOD;
                }
            }

            // Copy back and update dp_sz
            for i in 0..=dp_sz[u] + dp_sz[v] {
                dp[u][i] = tmp[i];
            }
            dp_sz[u] += dp_sz[v];
        }

        // If u != root, perform the "closing" transition into size sz[u]
        if u != root {
            let target = sz[u];
            for x in (0..=dp_sz[u]).rev() {
                let diff = target - x;
                let c = calc(diff, &pw);
                if c == 0 {
                    continue;
                }
                let add0 = (dp[u][x][1] * c) % MOD;
                let add1 = (dp[u][x][0] * c) % MOD;
                dp[u][target][0] = (dp[u][target][0] + add0) % MOD;
                dp[u][target][1] = (dp[u][target][1] + add1) % MOD;
            }
        }

        dp_sz[u] += 1;
    }

    // Final aggregation at root
    let mut ans: i64 = 0;
    for i in 0..=n {
        let w = calc(n - i, &pw);
        if w == 0 {
            continue;
        }
        ans = (ans + dp[root][i][0] * w) % MOD;
        ans = (ans - dp[root][i][1] * w) % MOD;
    }
    ans %= MOD;
    if ans < 0 {
        ans += MOD;
    }

    print!("{ans}\n");
}
