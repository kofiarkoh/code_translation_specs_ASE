use std::io::{self, Read};

fn main() {
    // Read entire stdin (C uses fgets twice, but behavior depends only on the bytes read).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Emulate the C in() parser:
    // int n=0; while (*p>='0') n=10*n+(*p++&0xf); p++; return n;
    // This assumes there is at least one non-digit delimiter after each number.
    fn next_int(bytes: &[u8], idx: &mut usize) -> i32 {
        let mut n: i32 = 0;
        while *idx < bytes.len() && bytes[*idx] >= b'0' {
            // (*p & 0xf) is equivalent to digit value for ASCII digits.
            n = n * 10 + ((bytes[*idx] & 0x0f) as i32);
            *idx += 1;
        }
        // Skip exactly one delimiter character (as in the C code's p++).
        if *idx < bytes.len() {
            *idx += 1;
        }
        n
    }

    let n = next_int(bytes, &mut idx) as usize;

    // sz = 1; while (sz < n) sz <<= 1;
    let mut sz: usize = 1;
    while sz < n {
        sz <<= 1;
    }

    // Segment tree array: C has seg[262145] and uses indices up to 2*sz-1.
    // Here we size it exactly to 2*sz (enough for indices 0..=2*sz-1).
    let mut seg = vec![0i64; 2 * sz];

    // update(k, v): k is 1-based-ish in the problem input; C does k += sz-1.
    fn update(seg: &mut [i64], sz: usize, mut k: usize, mut v: i64) {
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

    // query(a,b,k,l,r): max on [a,b), with node k covering [l,r)
    fn query(seg: &[i64], a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return seg[k];
        }
        let m = (l + r) >> 1;
        // C does: k <<= 1; lmax = query(..., ++k, ...); rmax = query(..., ++k, ...);
        // That corresponds to children indices 1 and 2 when parent is 0, and in general:
        // left = 2*k+1, right = 2*k+2.
        let left = 2 * k + 1;
        let right = left + 1;
        let lmax = query(seg, a, b, left, l, m);
        let rmax = query(seg, a, b, right, m, r);
        if lmax < rmax { rmax } else { lmax }
    }

    // Read n numbers and process.
    for _ in 0..n {
        let x = next_int(bytes, &mut idx) as usize;
        let best = query(&seg, 1, x, 0, 0, sz);
        update(&mut seg, sz, x, x as i64 + best);
    }

    let total = ((n as i64) * ((n as i64) + 1)) >> 1;
    let best_all = query(&seg, 1, n + 1, 0, 0, sz);
    let ans = total - best_all;

    print!("{ans}\n");
}
