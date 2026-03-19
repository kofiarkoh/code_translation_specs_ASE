use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    a: i64,
    d: i32,
}

// Fast input matching the C code behavior: reads non-negative integers,
// stops when the next byte is < '0'. (No skipping of leading whitespace.)
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
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        // Mimic getchar_unlocked() returning EOF as -1; here we assume input is valid.
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

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_i32() as usize;

    let mut h = vec![0i32; n + 1];
    let mut a = vec![0i32; n + 1];
    let mut d = vec![0i32; n + 1];
    let mut s = vec![0i32; n + 1];

    for i in 0..=n {
        h[i] = sc.in_i32();
        a[i] = sc.in_i32();
        d[i] = sc.in_i32();
        s[i] = sc.in_i32();
    }

    let mut tbl: Vec<T> = Vec::with_capacity(n);
    let mut turn: i64 = 0;
    let mut ans: i64 = 0;

    for i in 1..=n {
        let k = a[i] - d[0];
        if k <= 0 {
            continue;
        }
        if a[0] <= d[i] {
            print!("-1\n");
            return;
        }

        turn += k as i64;
        if s[i] > s[0] {
            ans += k as i64;
        }

        let denom = (a[0] - d[i]) as i64;
        // C integer division truncates toward zero; all values here are intended positive.
        let ai = ((h[i] as i64) - (d[i] as i64) + (a[0] as i64) - 1) / denom;

        tbl.push(T { a: ai, d: k });
    }

    // C qsort comparator returns 0/1 (not a strict weak ordering). We emulate the
    // intended ordering by sorting by the cross-multiplication comparison:
    // a1/d1 < a2/d2  <=>  a1*d2 < a2*d1.
    // Use i128 to avoid overflow that could occur in C (which would be UB there).
    tbl.sort_by(|x, y| {
        let lhs = (x.a as i128) * (y.d as i128);
        let rhs = (y.a as i128) * (x.d as i128);
        if lhs < rhs {
            Ordering::Less
        } else if lhs > rhs {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for t in &tbl {
        ans += turn * t.a - (t.d as i64);
        if (h[0] as i64) <= ans {
            print!("-1\n");
            return;
        }
        turn -= t.d as i64;
    }

    print!("{}\n", ans);
}
