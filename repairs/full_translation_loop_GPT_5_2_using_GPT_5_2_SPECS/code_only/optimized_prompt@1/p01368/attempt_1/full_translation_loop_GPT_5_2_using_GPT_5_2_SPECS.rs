use std::io::{self, Read};

const MAX: usize = 2002;
const INF: i32 = 0x1010;

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

    // Emulates the C `in()`:
    // - assumes the next non-consumed byte is a digit
    // - reads consecutive digits, stops when c < '0'
    // This matches the original behavior on AOJ-style inputs.
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;
        loop {
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
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

fn bpm(
    u: usize,
    hi: &[i16; MAX],
    to: &[[i16; MAX]; MAX],
    seen: &mut [u8; MAX],
    match_: &mut [i16; MAX],
) -> bool {
    let hu = hi[u] as usize;
    for i in 0..hu {
        let v = to[u][i] as usize;
        if seen[v] != 0 {
            continue;
        }
        seen[v] = 1;
        let mv = match_[v];
        if mv < 0 || bpm(mv as usize, hi, to, seen, match_) {
            match_[u] = v as i16;
            match_[v] = u as i16;
            return true;
        }
    }
    false
}

fn main() {
    let mut sc = Scanner::new();

    // Global-like arrays (as in the C code)
    let mut hi: [i16; MAX] = [0; MAX];
    let mut to: [[i16; MAX]; MAX] = [[0; MAX]; MAX];
    let mut seen: [u8; MAX] = [0; MAX];
    let mut match_: [i16; MAX] = [-1; MAX];

    let mut dist: [[i32; 102]; 102] = [[INF; 102]; 102];
    let mut p: [i32; 1002] = [0; 1002];
    let mut t: [i32; 1002] = [0; 1002];

    let mut out = String::new();

    loop {
        let n = sc.in_i32();
        if n == 0 {
            break;
        }
        let n_us = n as usize;
        let m = sc.in_i32() as usize;
        let l = sc.in_i32() as usize;

        // memset(dist, INF, sizeof(dist)) where INF=0x1010 and dist is int:
        // This sets every byte to 0x10, so each i32 becomes 0x10101010.
        // Preserve that exact value.
        dist = [[0x10101010u32 as i32; 102]; 102];

        for k in 0..n_us {
            dist[k][k] = 0;
        }
        for _ in 0..m {
            let u = sc.in_i32() as usize;
            let v = sc.in_i32() as usize;
            let d = sc.in_i32();
            dist[u][v] = d;
            dist[v][u] = d;
        }

        // Floyd-Warshall with the same guards and i32 arithmetic.
        for k in 0..n_us {
            for u in 0..n_us {
                if dist[u][k] != (0x10101010u32 as i32) {
                    for v in 0..n_us {
                        if dist[k][v] != (0x10101010u32 as i32) {
                            let d = dist[u][k] + dist[k][v];
                            if dist[u][v] > d {
                                dist[u][v] = d;
                            }
                        }
                    }
                }
            }
        }

        // memset(hi, 0, sizeof(hi));
        hi = [0; MAX];

        for k in 0..l {
            p[k] = sc.in_i32();
            t[k] = sc.in_i32();
        }

        // Build bipartite edges: left [0..L), right [L..2L)
        for u in 0..l {
            for v in 0..l {
                if u == v {
                    continue;
                }
                let pu = p[u] as usize;
                let pv = p[v] as usize;
                if dist[pu][pv] <= t[v] - t[u] {
                    let idx = hi[u] as usize;
                    to[u][idx] = (v + l) as i16;
                    hi[u] += 1;
                }
            }
        }

        // memset(match, -1, L << 2);  // only first L ints (4 bytes each) in C
        // match is short[] here; but the C code writes bytes, effectively setting
        // first 4*L bytes to 0xFF => first 2*L shorts become -1.
        // We'll reproduce that effect.
        for i in 0..(2 * l).min(MAX) {
            match_[i] = -1;
        }

        let mut k_ans = l as i32;
        for u in 0..l {
            // memset(seen, 0, L << 1);  // first 2*L bytes
            for i in 0..(2 * l).min(MAX) {
                seen[i] = 0;
            }
            if bpm(u, &hi, &to, &mut seen, &mut match_) {
                k_ans -= 1;
            }
        }

        out.push_str(&format!("{}\n", k_ans));
    }

    print!("{out}");
}
