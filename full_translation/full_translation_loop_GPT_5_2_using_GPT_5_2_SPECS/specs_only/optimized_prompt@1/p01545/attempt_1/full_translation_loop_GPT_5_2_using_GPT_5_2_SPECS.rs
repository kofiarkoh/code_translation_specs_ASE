use std::io::{self, Read};

fn main() {
    // Input is specified as well-formed non-negative decimal integers separated by non-digits.
    // We implement a digit-scanner to match that model (and avoid relying on whitespace splitting).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut i = 0usize;

    fn next_u64(bytes: &[u8], i: &mut usize) -> Option<u64> {
        while *i < bytes.len() && !(bytes[*i] as char).is_ascii_digit() {
            *i += 1;
        }
        if *i >= bytes.len() {
            return None;
        }
        let mut v: u64 = 0;
        while *i < bytes.len() && (bytes[*i] as char).is_ascii_digit() {
            v = v * 10 + (bytes[*i] - b'0') as u64;
            *i += 1;
        }
        Some(v)
    }

    let n_u64 = match next_u64(bytes, &mut i) {
        Some(v) => v,
        None => return, // No explicit error handling specified; just do nothing if no input.
    };
    let n = n_u64 as usize;

    // Segment tree over indices [0..=n] (inclusive), storing non-negative best sums.
    // Preconditions say n <= 262144 and values are intended to be within [0, n].
    // If violated, behavior is undefined; we do not add checks beyond what is needed to compile.
    let mut size = 1usize;
    while size < (n + 1) {
        size <<= 1;
    }
    let mut seg = vec![0u64; 2 * size];

    // Query maximum on inclusive range [l, r] within [0, size-1].
    fn query(seg: &[u64], size: usize, mut l: usize, mut r: usize) -> u64 {
        if l > r {
            return 0;
        }
        l += size;
        r += size;
        let mut res = 0u64;
        while l <= r {
            if (l & 1) == 1 {
                res = res.max(seg[l]);
                l += 1;
            }
            if (r & 1) == 0 {
                res = res.max(seg[r]);
                if r == 0 {
                    break;
                }
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }

    // Point update: set position p to max(current, val), then fix ancestors.
    fn update(seg: &mut [u64], size: usize, mut p: usize, val: u64) {
        p += size;
        if seg[p] >= val {
            return;
        }
        seg[p] = val;
        p >>= 1;
        while p > 0 {
            let newv = seg[p << 1].max(seg[(p << 1) | 1]);
            if seg[p] == newv {
                break;
            }
            seg[p] = newv;
            p >>= 1;
        }
    }

    for _ in 0..n {
        let x_u64 = match next_u64(bytes, &mut i) {
            Some(v) => v,
            None => 0, // Undefined per spec if insufficient numbers; keep narrow behavior.
        };
        let x = x_u64 as usize;

        // State transition:
        // best[x] = x + max(best[0..x-1]) (strictly less than x).
        // Stored best sums are non-negative; x is non-negative.
        let best_prev = if x == 0 { 0 } else { query(&seg, size, 0, x - 1) };
        let cand = (x as u64).wrapping_add(best_prev);
        update(&mut seg, size, x, cand);
    }

    // M is the global maximum best sum over values in [1, n].
    let m = if n >= 1 {
        query(&seg, size, 1, n)
    } else {
        0
    };

    // T = n(n+1)/2, then ans = T - M.
    // Use wrapping arithmetic to emulate potential overflow behavior if preconditions are violated.
    let n64 = n as u64;
    let t = n64.wrapping_mul(n64.wrapping_add(1)) / 2;
    let ans_u64 = t.wrapping_sub(m);

    // Output is a single signed integer followed by newline.
    // Under preconditions, ans fits in signed range; if not, behavior is unspecified.
    let ans_i64 = ans_u64 as i64;
    print!("{ans_i64}\n");
}
