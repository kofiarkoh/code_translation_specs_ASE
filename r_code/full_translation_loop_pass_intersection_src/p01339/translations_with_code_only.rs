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
    fn in_u32(&mut self) -> u32 {
        // Mirrors the C in(): assumes next non-consumed byte is a digit.
        // Stops when c < '0' (i.e., any non-digit with ASCII < '0', including whitespace/newline).
        let mut n: u32 = 0;
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            if c < b'0' {
                break;
            }
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

fn scc(v: usize, to: &Vec<Vec<usize>>, r_to: &Vec<Vec<usize>>, cmp: &mut [i32]) -> usize {
    let mut used = vec![0u8; v];
    let mut vs: Vec<usize> = Vec::with_capacity(v);

    for i in 0..v {
        if used[i] == 0 {
            dfs(i, to, &mut used, &mut vs);
        }
    }

    used.fill(0);
    let mut k: i32 = 0;
    for &node in vs.iter().rev() {
        if used[node] == 0 {
            rdfs(node, r_to, &mut used, cmp, k);
            k += 1;
        }
    }
    k as usize
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_u32() as usize;
    let mut m = sc.in_u32() as i32;

    // Adjacency lists; C code uses manual realloc growth, but semantics are just dynamic arrays.
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut r_to: Vec<Vec<usize>> = vec![Vec::new(); n];

    while m > 0 {
        m -= 1;
        let s = sc.in_u32() as usize - 1;
        let d = sc.in_u32() as usize - 1;
        to[s].push(d);
        r_to[d].push(s);
    }

    let mut cmp = vec![-1i32; n];
    let k = scc(n, &to, &r_to, &mut cmp);

    let mut nx = vec![-1i32; k];
    for i in 0..n {
        if !to[i].is_empty() {
            let ci = cmp[i] as usize;
            let cto = cmp[to[i][0]]; // exactly the first outgoing edge, as in C
            nx[ci] = cto;
        }
    }

    let mut dp = vec![0i64; k];
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

    // C prints as int with newline.
    println!("{}", ans as i32);
}
