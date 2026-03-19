use std::io::{self, Read};

fn main() {
    // Read entire stdin (C code assumes well-formed non-negative integers separated by non-digits).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Parse next non-negative integer similarly to the C `in()`:
    // - Accumulate while current byte is a digit.
    // - Then advance one extra byte (assumes a separator exists).
    fn next_int(bytes: &[u8], idx: &mut usize) -> usize {
        let mut n: usize = 0;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            n = n * 10 + (bytes[*idx] - b'0') as usize;
            *idx += 1;
        }
        // Mimic `p++` in C: skip one separator character if present.
        if *idx < bytes.len() {
            *idx += 1;
        }
        n
    }

    let n = next_int(bytes, &mut idx) as usize;

    // Segment tree size: next power of two >= n (as in C).
    let mut sz: usize = 1;
    while sz < n {
        sz <<= 1;
    }

    // seg array: C uses fixed 262145, but spec allows sizing to next power of two.
    // We implement a classic iterative segment tree for max with 0 as identity.
    // Leaves correspond to indices [0, sz).
    let mut seg = vec![0i64; 2 * sz];

    // Query max on [l, r) over value indices.
    let mut query = |mut l: usize, mut r: usize, seg: &Vec<i64>, sz: usize| -> i64 {
        let mut res = 0i64;
        l += sz;
        r += sz;
        while l < r {
            if (l & 1) == 1 {
                res = res.max(seg[l]);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                res = res.max(seg[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    };

    // Update position pos to value v (overwrite), then recompute ancestors with max.
    let mut update = |pos: usize, v: i64, seg: &mut Vec<i64>, sz: usize| {
        let mut k = pos + sz;
        seg[k] = v;
        while k > 1 {
            k >>= 1;
            seg[k] = seg[k << 1].max(seg[(k << 1) | 1]);
        }
    };

    // Process n values.
    for _ in 0..n {
        let x = next_int(bytes, &mut idx) as usize;
        // State transition per spec:
        // best[x] = x + max(best[v]) for v in [1, x)  (strictly less than x)
        // C queries query(1, x, 0, 0, sz) and updates at index x.
        let best_prefix = query(1, x, &seg, sz);
        let new_val = x as i64 + best_prefix;
        update(x, new_val, &mut seg, sz);
    }

    let total = (n as i64) * ((n as i64) + 1) / 2;
    let m = query(1, n + 1, &seg, sz);
    let ans = total - m;

    print!("{ans}\n");
}
