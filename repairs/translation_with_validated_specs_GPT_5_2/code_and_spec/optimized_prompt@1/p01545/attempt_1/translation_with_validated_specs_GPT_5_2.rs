use std::io::{self, Read};

fn main() {
    // Read entire stdin. The spec guarantees well-formed non-negative integers separated
    // by non-digit delimiters, so a simple digit-scanner matches the C behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    fn next_uint(bytes: &[u8], idx: &mut usize) -> usize {
        // Skip non-digits
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        let mut n: usize = 0;
        while *idx < bytes.len() && (b'0'..=b'9').contains(&bytes[*idx]) {
            n = n * 10 + (bytes[*idx] - b'0') as usize;
            *idx += 1;
        }
        n
    }

    let n = next_uint(bytes, &mut idx);

    // Segment tree size: smallest power of two >= n (as in C).
    let mut sz: usize = 1;
    while sz < n {
        sz <<= 1;
    }

    // seg[k] stores max over the segment for node k in a 0-based heap layout:
    // children of k are 2k+1 and 2k+2. Leaves start at index (sz-1).
    let mut seg = vec![0i64; 2 * sz - 1];

    // Update leaf corresponding to value k (1..=n) to v, then recompute ancestors.
    // This mirrors the C code's update() behavior.
    let mut update = |k: usize, v: i64, seg: &mut [i64], sz: usize| {
        let mut pos = k + sz - 1;
        seg[pos] = v;
        while pos > 0 {
            // In C: if (!(k & 1)) k--; then parent = k>>1, and compare siblings.
            // Here we compute parent and take max of its two children directly.
            let parent = (pos - 1) / 2;
            let left = 2 * parent + 1;
            let right = left + 1;
            let newv = if seg[left] < seg[right] { seg[right] } else { seg[left] };
            seg[parent] = newv;
            pos = parent;
        }
    };

    // Query max in [a, b) over value-domain indices, using segment tree range [l, r).
    fn query(a: usize, b: usize, k: usize, l: usize, r: usize, seg: &[i64]) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return seg[k];
        }
        let m = (l + r) >> 1;
        let left = query(a, b, 2 * k + 1, l, m, seg);
        let right = query(a, b, 2 * k + 2, m, r, seg);
        if left < right { right } else { left }
    }

    // Process n values.
    for _ in 0..n {
        let x = next_uint(bytes, &mut idx);
        // Spec precondition: 1 <= x <= n. Undefined behavior otherwise (as in C).
        let best_prefix = query(1, x, 0, 0, sz, &seg); // max over values in [1, x)
        let val = x as i64 + best_prefix;
        update(x, val, &mut seg, sz);
    }

    let s = (n as i64) * ((n as i64) + 1) / 2;
    let m = if n == 0 {
        0
    } else {
        query(1, n + 1, 0, 0, sz, &seg) // max over values in [1, n+1) == [1..=n]
    };
    let ans = s - m;
    print!("{ans}\n");
}
