use std::io::{self, Read};

const MAX: usize = 1005;

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
    fn gc(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            0 // emulate EOF as 0; C code assumes valid input
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }
    // Non-negative integer input, mirroring the C code:
    // int n=0,c=gc(); do n=10*n+(c&0xf), c=gc(); while(c>='0');
    fn in_nonneg(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = self.gc();
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            c = self.gc();
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn outs(out: &mut String, s: &str) {
    out.push_str(s);
    out.push('\n');
}

fn out_num(out: &mut String, mut n: i64) {
    // C version prints positive integers; if n==0 it would print just '\n'.
    if n == 0 {
        out.push('\n');
        return;
    }
    let mut buf = [0u8; 40];
    let mut i = 0usize;
    while n != 0 {
        let d = (n % 10) as u8;
        buf[i] = b'0' + d;
        i += 1;
        n /= 10;
    }
    while i > 0 {
        i -= 1;
        out.push(buf[i] as char);
    }
    out.push('\n');
}

struct Solver {
    hi: [usize; MAX],
    to: [[i32; MAX]; MAX],
    seen: [i32; MAX],
    matchv: [i32; MAX],
    mark: i32,
}

impl Solver {
    fn new() -> Self {
        Self {
            hi: [0; MAX],
            to: [[0; MAX]; MAX],
            seen: [0; MAX],
            matchv: [-1; MAX],
            mark: 1,
        }
    }

    fn bpm(&mut self, u: i32) -> bool {
        let uu = u as usize;
        self.seen[uu] = self.mark;
        for i in 0..self.hi[uu] {
            let v = self.to[uu][i];
            let w = self.matchv[v as usize];
            if w < 0 || (self.seen[w as usize] != self.mark && self.bpm(w)) {
                self.matchv[v as usize] = u;
                self.matchv[uu] = v;
                return true;
            }
        }
        false
    }

    fn bipartite_matching(&mut self, m: i32, _n: i32) -> i32 {
        // memset(match, -1, sizeof(match));
        self.matchv = [-1; MAX];
        let mut maxm = 0;
        for u in 0..m {
            if self.matchv[u as usize] < 0 {
                self.mark += 1;
                if self.bpm(u) {
                    maxm += 1;
                }
            }
        }
        maxm
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut sol = Solver::new();
    let mut out = String::new();

    let n_input = sc.in_nonneg();
    let n = n_input.wrapping_sub(1); // N = in()-1; preserve wrapping behavior
    let n_usize = if n <= 0 { 0usize } else { n as usize };

    // Read N x N grid, consuming exactly as C does:
    // if (gc() & 1) ...; gc();
    for i in 0..n_usize {
        for j in 0..n_usize {
            let c1 = sc.gc();
            if (c1 & 1) != 0 {
                let idx = sol.hi[j];
                sol.to[j][idx] = (n_usize + i) as i32;
                sol.hi[j] = idx + 1;
            }
            let _c2 = sc.gc();
        }
    }

    if sol.bipartite_matching(n, n) < n {
        outs(&mut out, "no");
    } else {
        outs(&mut out, "yes");
        for i in 0..n_usize {
            let mi = sol.matchv[i] as usize;
            if mi < MAX {
                sol.matchv[mi] = -1;
            }
            sol.matchv[i] = -1;
            sol.mark += 1;
            sol.bpm(i as i32);

            sol.hi[i] = 0;
            sol.to[i][0] = sol.matchv[i];
            sol.hi[i] = 1;

            let val = sol.matchv[i] as i64 - n as i64 + 1;
            out_num(&mut out, val);
        }
    }

    print!("{out}");
}
