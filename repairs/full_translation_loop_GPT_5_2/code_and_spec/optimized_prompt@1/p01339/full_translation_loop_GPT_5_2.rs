use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Fast input: read all bytes, parse non-negative integers (as in C in()).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    fn read_u(bytes: &[u8], idx: &mut usize) -> usize {
        // C version assumes next char is a digit; we skip non-digits to be robust.
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        let mut n: usize = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            *idx += 1;
        }
        n
    }

    let n = read_u(bytes, &mut idx);
    let m = read_u(bytes, &mut idx);

    // Graph adjacency lists (forward and reverse).
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut rg: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let s = read_u(bytes, &mut idx).wrapping_sub(1);
        let d = read_u(bytes, &mut idx).wrapping_sub(1);
        // Spec says invalid indices => undefined; we mirror C behavior (no checks).
        g[s].push(d);
        rg[d].push(s);
    }

    // Kosaraju SCC.
    let mut used = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    fn dfs(v: usize, g: &Vec<Vec<usize>>, used: &mut [bool], order: &mut Vec<usize>) {
        used[v] = true;
        for &to in &g[v] {
            if !used[to] {
                dfs(to, g, used, order);
            }
        }
        order.push(v);
    }

    for v in 0..n {
        if !used[v] {
            dfs(v, &g, &mut used, &mut order);
        }
    }

    let mut comp = vec![0usize; n];
    used.fill(false);

    fn rdfs(v: usize, k: usize, rg: &Vec<Vec<usize>>, used: &mut [bool], comp: &mut [usize]) {
        used[v] = true;
        comp[v] = k;
        for &to in &rg[v] {
            if !used[to] {
                rdfs(to, k, rg, used, comp);
            }
        }
    }

    let mut k = 0usize;
    for &v in order.iter().rev() {
        if !used[v] {
            rdfs(v, k, &rg, &mut used, &mut comp);
            k += 1;
        }
    }

    // Build nx per SCC: in the C code, for each vertex with outgoing edges,
    // nx[cmp[v]] is set to cmp[to[v][0]] (only the first outgoing edge is used).
    // This is a notable semantic choice: it does NOT consider all edges.
    let mut nx: Vec<i32> = vec![-1; k];
    for v in 0..n {
        if !g[v].is_empty() {
            let from_c = comp[v];
            let to_c = comp[g[v][0]];
            nx[from_c] = to_c as i32;
        }
    }

    // DP/answer computation exactly as in C.
    let mut dp: Vec<i64> = vec![0; k];
    let mut ans: i64 = 1;

    for i in 0..k {
        if dp[i] != 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }

        let ni = nx[i];
        if ni < 0 || ni as usize == i {
            ans = (ans * (dp[i] % MOD)) % MOD;
        } else {
            let j = ni as usize;
            if dp[j] != 0 {
                dp[j] = (dp[j] * (dp[i] % MOD)) % MOD;
            } else {
                dp[j] = dp[i] % MOD;
            }
        }
    }

    // Output formatting: single integer + newline.
    println!("{}", ans % MOD);
}
