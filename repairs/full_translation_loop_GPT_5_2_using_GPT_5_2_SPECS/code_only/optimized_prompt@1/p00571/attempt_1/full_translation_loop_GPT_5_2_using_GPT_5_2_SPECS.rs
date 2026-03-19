use std::cmp::Ordering;
use std::io::{self, Read};

const INF: i64 = 0x1010_1010_1010_1010u64 as i64;

#[derive(Clone, Copy)]
struct T {
    a: i64,
    b: i32,
}

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

    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            0
        } else {
            let b = self.buf[self.idx];
            self.idx += 1;
            b
        }
    }

    // Emulates the C in(): reads first byte as-is (no skipping),
    // then accumulates digits while c >= '0' (note: this also includes ':'.. etc,
    // but matches the C code's condition).
    #[inline]
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = self.next_byte();
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            c = self.next_byte();
            if c < b'0' {
                break;
            }
        }
        n
    }

    // Emulates the C big(): same logic but long long.
    #[inline]
    fn in_i64(&mut self) -> i64 {
        let mut n: i64 = 0;
        let mut c = self.next_byte();
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i64);
            c = self.next_byte();
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

    // 1-based like the C code; index 0 is unused for tbl, and s[0]=0.
    let mut tbl = vec![T { a: 0, b: 0 }; n + 1];
    let mut s = vec![0i64; n + 1];

    for i in 1..=n {
        tbl[i].a = sc.in_i64();
        tbl[i].b = sc.in_i32();
    }

    // qsort(tbl+1, N, ...) with cmp: by a asc, then b asc.
    tbl[1..=n].sort_unstable_by(|x, y| match x.a.cmp(&y.a) {
        Ordering::Equal => x.b.cmp(&y.b),
        other => other,
    });

    let mut minv: i64 = INF;
    let mut maxv: i64 = -INF;

    for i in 1..=n {
        let t1 = s[i - 1].wrapping_sub(tbl[i].a);
        if t1 < minv {
            minv = t1;
        }
        s[i] = s[i - 1].wrapping_add(tbl[i].b as i64);
        let t2 = s[i]
            .wrapping_sub(tbl[i].a)
            .wrapping_sub(minv);
        if t2 > maxv {
            maxv = t2;
        }
    }

    print!("{maxv}\n");
}
