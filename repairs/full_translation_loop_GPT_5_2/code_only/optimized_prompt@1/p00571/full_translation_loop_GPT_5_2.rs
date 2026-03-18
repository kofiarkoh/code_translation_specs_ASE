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

    // Emulates the C in()/big() behavior:
    // - starts from current byte (no skipping of non-digits)
    // - reads at least one byte, then continues while c >= '0' (not <= '9'!)
    // - digit value is (c & 0x0f)
    fn read_i32_like_c_in(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c: u8 = self.next_byte();
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            c = self.next_byte();
            if c < b'0' {
                break;
            }
        }
        n
    }

    fn read_i64_like_c_big(&mut self) -> i64 {
        let mut n: i64 = 0;
        let mut c: u8 = self.next_byte();
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i64);
            c = self.next_byte();
            if c < b'0' {
                break;
            }
        }
        n
    }

    fn next_byte(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            // C getchar_unlocked() would return EOF (-1). Here we map to 0xFF.
            // This preserves the loop condition behavior (>= '0') as false.
            return 0xFF;
        }
        let b = self.buf[self.idx];
        self.idx += 1;
        b
    }
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.read_i32_like_c_in() as usize;

    // tbl[0] unused; keep 1-based indexing like the C code.
    let mut tbl = vec![T { a: 0, b: 0 }; n + 1];
    let mut s = vec![0i64; n + 1];

    for i in 1..=n {
        tbl[i].a = sc.read_i64_like_c_big();
        tbl[i].b = sc.read_i32_like_c_in();
    }

    // qsort with cmp: by a ascending, then b ascending.
    tbl[1..=n].sort_by(|x, y| match x.a.cmp(&y.a) {
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
