use std::io::{self, Read};

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

    // Mimic the C in(): parse consecutive digits starting at current position,
    // then advance one extra byte (assumes a delimiter exists).
    fn in_int(&mut self) -> i32 {
        let mut n: i32 = 0;
        while self.idx < self.buf.len() && self.buf[self.idx] >= b'0' {
            n = n * 10 + ((self.buf[self.idx] & 0x0f) as i32);
            self.idx += 1;
        }
        // C does p++; unconditionally. We do the same if possible.
        if self.idx < self.buf.len() {
            self.idx += 1;
        }
        n
    }
}

fn update(seg: &mut [i64], sz: usize, mut k: usize, mut v: i64) {
    // k is 0-based index into leaves in this implementation, matching the C code usage.
    k += sz - 1;
    seg[k] = v;
    while k > 0 {
        if (k & 1) == 0 {
            k -= 1;
        }
        let t = k;
        k >>= 1;
        let mut mx = seg[t];
        if mx < seg[t + 1] {
            mx = seg[t + 1];
        }
        v = mx;
        seg[k] = v;
    }
}

fn query(seg: &[i64], a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
    if r <= a || b <= l {
        return 0;
    }
    if a <= l && r <= b {
        return seg[k];
    }
    let m = (l + r) >> 1;
    // In C: k <<= 1; lmax = query(..., ++k, ...); rmax = query(..., ++k, ...);
    // That means children indices are 1 and 2 when parent is 0, and generally 2k+1, 2k+2.
    let left = query(seg, a, b, 2 * k + 1, l, m);
    let right = query(seg, a, b, 2 * k + 2, m, r);
    if left < right { right } else { left }
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_int() as usize;

    let mut sz: usize = 1;
    while sz < n {
        sz <<= 1;
    }

    // C has seg[262145]; enough for sz up to 2^18. We'll allocate 2*sz safely.
    let mut seg = vec![0i64; 2 * sz];

    for _ in 0..n {
        let x = sc.in_int() as usize;
        let best = query(&seg, 1, x, 0, 0, sz);
        update(&mut seg, sz, x, x as i64 + best);
    }

    let total = ((n as i64) * ((n + 1) as i64)) >> 1;
    let best_all = query(&seg, 1, n + 1, 0, 0, sz);
    let ans = total - best_all;

    print!("{ans}\n");
}
