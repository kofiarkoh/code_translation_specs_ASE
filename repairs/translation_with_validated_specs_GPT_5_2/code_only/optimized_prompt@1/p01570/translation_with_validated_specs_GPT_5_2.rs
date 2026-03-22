use std::cmp::Ordering;
use std::io::{self, Read};

const MAX: usize = 1_000_002;

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
    fn next_i32(&mut self) -> i32 {
        // The C code assumes well-formed input; it does not skip whitespace explicitly.
        // We'll skip whitespace to be robust while preserving numeric parsing semantics.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut neg = false;
        if self.idx < self.buf.len() && self.buf[self.idx] == b'-' {
            neg = true;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            // C uses (c & 0xf); for ASCII digits this equals c - '0'.
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            self.idx += 1;
        }
        if neg { -n } else { n }
    }
}

fn calc(u: usize, m: &[i32], to: &[Vec<usize>], s: &mut [i64], dp: &mut [i64]) {
    let mut b: Vec<i64> = Vec::with_capacity(to[u].len());
    s[u] = m[u] as i64;
    dp[u] = 0;
    for &v in &to[u] {
        b.push(s[v]);
        s[u] += s[v];
        dp[u] += dp[v];
    }
    // C comparator sorts descending (returns 1 when a<b).
    b.sort_unstable_by(|x, y| y.cmp(x));
    for (i, &val) in b.iter().enumerate() {
        dp[u] += val * (i as i64);
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_i32() as usize;

    let mut m = vec![0i32; n];
    let mut memo = vec![0i32; n];
    let mut hi = vec![0usize; n];

    let mut bsz: usize = 0;
    for i in 0..n {
        m[i] = sc.next_i32();
        let l = sc.next_i32();
        memo[i] = l;
        if l == -1 {
            bsz += 1;
        } else {
            let p = l as usize;
            hi[p] += 1;
        }
    }

    let mut base: Vec<usize> = Vec::with_capacity(bsz);
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if hi[i] != 0 {
            to[i] = Vec::with_capacity(hi[i]);
        }
    }
    // Reset hi to 0 like the C code (it reuses hi as fill counters).
    for x in &mut hi {
        *x = 0;
    }

    for i in 0..n {
        let l = memo[i];
        if l == -1 {
            base.push(i);
        } else {
            let p = l as usize;
            to[p].push(i);
            hi[p] += 1;
        }
    }

    let mut a = vec![0usize; n];
    let mut s = vec![0i64; n];
    let mut dp = vec![0i64; n];

    let mut que = vec![0usize; n + 2]; // 1-based stack like the C code
    let mut ans: i64 = 0;

    for &root in &base {
        que[1] = root;
        let mut top: usize = 1;
        while top != 0 {
            let t = que[top];
            if a[t] < hi[t] {
                let child = to[t][a[t]];
                a[t] += 1;
                top += 1;
                que[top] = child;
            } else {
                calc(t, &m, &to, &mut s, &mut dp);
                top -= 1;
            }
        }
        ans += dp[root];
    }

    print!("{}\n", ans);
}
