use std::io::{self, Read};

const MAX_V: usize = 1002;
const MOD: i64 = 1_000_000_007;

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    // Emulates the C in(): reads a non-negative integer, assuming the stream is positioned
    // at a digit (AOJ style). Stops at first non-digit (< '0').
    fn in_u32(&mut self) -> u32 {
        let mut n: u32 = 0;
        // In the original C, it does not skip non-digits; it starts from current getchar.
        // For robustness with typical whitespace-separated input, we skip until digit.
        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            self.idx += 1;
        }
        n
    }
}

fn dfs(v: usize, to: &Vec<Vec<usize>>, used: &mut [u8], vs: &mut Vec<usize>) {
    used[v] = 1;
    for &u in &to[v] {
        if used[u] == 0 {
            dfs(u, to, used, vs);
        }
    }
    vs.push(v);
}

fn rdfs(v: usize, r_to: &Vec<Vec<usize>>, used: &mut [u8], cmp: &mut [i32], k: i32) {
    used[v] = 1;
    cmp[v] = k;
    for &u in &r_to[v] {
        if used[u] == 0 {
            rdfs(u, r_to, used, cmp, k);
        }
    }
}

// Kosaraju SCC; returns number of components, and fills cmp[v] with component id in [0..k).
fn scc(vn: usize, to: &Vec<Vec<usize>>, r_to: &Vec<Vec<usize>>, cmp: &mut [i32]) -> usize {
    let mut used = vec![0u8; vn];
    let mut vs: Vec<usize> = Vec::with_capacity(vn);
    for v in 0..vn {
        if used[v] == 0 {
            dfs(v, to, &mut used, &mut vs);
        }
    }
    used.fill(0);
    let mut k: i32 = 0;
    for &v in vs.iter().rev() {
        if used[v] == 0 {
            rdfs(v, r_to, &mut used, cmp, k);
            k += 1;
        }
    }
    k as usize
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.in_u32() as usize;
    let mut m = sc.in_u32() as usize;

    // The C code uses fixed MAX_V arrays; input constraints are assumed to fit.
    // We'll size by n but keep behavior equivalent for valid inputs.
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut r_to: Vec<Vec<usize>> = vec![Vec::new(); n];

    while m > 0 {
        m -= 1;
        let s = sc.in_u32() as i32 - 1;
        let d = sc.in_u32() as i32 - 1;
        if s >= 0 && d >= 0 {
            let su = s as usize;
            let du = d as usize;
            to[su].push(du);
            r_to[du].push(su);
        }
    }

    let mut cmp = vec![-1i32; n];
    let k = scc(n, &to, &r_to, &mut cmp);

    let mut nx = vec![-1i32; k];
    for i in 0..n {
        if !to[i].is_empty() {
            let ci = cmp[i] as usize;
            let cto0 = cmp[to[i][0]]; // first outgoing edge only, as in C
            nx[ci] = cto0;
        }
    }

    // dp and mk are global in C; mk is unused. dp is zero-initialized.
    let mut dp = vec![0i64; k];

    let mut ans: i64 = 1;
    for i in 0..k {
        if dp[i] != 0 {
            dp[i] += 1;
        } else {
            dp[i] = 2;
        }

        let ni = nx[i];
        if ni < 0 || ni == i as i32 {
            ans = (ans * (dp[i] % MOD)) % MOD;
        } else {
            let j = ni as usize;
            if dp[j] != 0 {
                dp[j] = (dp[j] * dp[i]) % MOD;
            } else {
                dp[j] = dp[i];
            }
        }
    }

    // Match C printf("%d\n", (int)ans);
    println!("{}", ans as i32);
}
