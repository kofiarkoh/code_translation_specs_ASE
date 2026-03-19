use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    a: i64,
    d: i32,
}

fn main() {
    // Read entire stdin (AOJ style fast input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Emulate the C in(): reads a non-negative integer, assuming input is well-formed.
    // It starts from the current byte and accumulates digits until a non-digit (< '0') is seen.
    fn in_int(bytes: &[u8], idx: &mut usize) -> i32 {
        // Skip any bytes < '0' (whitespace/newlines), matching typical judge input.
        while *idx < bytes.len() && bytes[*idx] < b'0' {
            *idx += 1;
        }
        let mut n: i32 = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            // n = n*10 + (c & 0xf)
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            *idx += 1;
        }
        n
    }

    let n = in_int(bytes, &mut idx) as usize;

    // Arrays sized like the C code (need indices 0..=n).
    let mut h = vec![0i32; n + 1];
    let mut a = vec![0i32; n + 1];
    let mut d = vec![0i32; n + 1];
    let mut s = vec![0i32; n + 1];

    for i in 0..=n {
        h[i] = in_int(bytes, &mut idx);
        a[i] = in_int(bytes, &mut idx);
        d[i] = in_int(bytes, &mut idx);
        s[i] = in_int(bytes, &mut idx);
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

        let denom = (a[0] - d[i]) as i64; // > 0 due to check above
        let numer = h[i] as i64 - d[i] as i64 + a[0] as i64 - 1;
        let aa = numer / denom; // C integer division truncates toward zero; denom>0 so same.

        tbl.push(T { a: aa, d: k });
    }

    // C qsort comparator returns 0/1 (not a strict weak ordering), but intended order is by
    // comparing a1*d2 > a2*d1 (i.e., sort by a/d descending). We implement that ordering.
    tbl.sort_by(|x, y| {
        let left = (x.a as i128) * (y.d as i128);
        let right = (y.a as i128) * (x.d as i128);
        if left > right {
            Ordering::Less // x before y (descending)
        } else if left < right {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for t in &tbl {
        ans += turn * t.a - t.d as i64;
        if (h[0] as i64) <= ans {
            print!("-1\n");
            return;
        }
        turn -= t.d as i64;
    }

    print!("{ans}\n");
}
