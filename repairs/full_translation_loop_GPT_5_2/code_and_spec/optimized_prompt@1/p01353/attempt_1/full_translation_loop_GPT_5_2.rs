use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    a: i64,
    d: i32,
}

fn main() {
    // Fast input: read all bytes, parse non-negative integers (matches C in()).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    fn next_int(bytes: &[u8], idx: &mut usize) -> i32 {
        // C code assumes digits start immediately; we skip non-digits to be robust.
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        let mut n: i32 = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            *idx += 1;
        }
        n
    }

    let n = next_int(bytes, &mut idx) as usize;

    // Arrays sized n+1 (C uses indices 0..=n).
    let mut h = vec![0i32; n + 1];
    let mut a = vec![0i32; n + 1];
    let mut d = vec![0i32; n + 1];
    let mut s = vec![0i32; n + 1];

    for i in 0..=n {
        h[i] = next_int(bytes, &mut idx);
        a[i] = next_int(bytes, &mut idx);
        d[i] = next_int(bytes, &mut idx);
        s[i] = next_int(bytes, &mut idx);
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
            // Error indicator, terminate immediately.
            print!("-1\n");
            return;
        }

        turn += k as i64;
        if s[i] > s[0] {
            ans += k as i64;
        }

        // ceil((h[i]-d[i]) / (a[0]-d[i])) with integer arithmetic:
        // ((h[i]-d[i] + (a0-di) - 1) / (a0-di))
        let denom = (a[0] - d[i]) as i64; // positive due to check above
        let numer = (h[i] - d[i]) as i64 + denom - 1;
        let aa = numer / denom;

        tbl.push(T { a: aa, d: k });
    }

    // C's qsort comparator returns (a1*d2 > a2*d1), which is not a strict weak ordering.
    // We implement a consistent ordering that matches the intended sort by ratio a/d:
    // sort ascending by a/d using cross-multiplication.
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

    print!("{ans}\n");
}
