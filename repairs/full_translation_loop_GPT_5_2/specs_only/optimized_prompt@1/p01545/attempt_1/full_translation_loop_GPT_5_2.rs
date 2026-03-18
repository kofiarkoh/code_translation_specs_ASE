use std::io::{self, Read};

fn next_pow2(mut x: usize) -> usize {
    // Smallest power of two >= x, with x >= 1 per preconditions.
    let mut p = 1usize;
    while p < x {
        p <<= 1;
    }
    p
}

fn main() {
    // No specific error handling required by the specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Segment tree storing maximums; size is a power of two.
    // Leaves represent positions in the "dynamic array".
    let size = next_pow2(n.max(1));
    let mut seg = vec![0i64; 2 * size];

    // Update operation: set position `pos` to max(current, value),
    // then recompute max on the path to the root.
    let mut update = |pos: usize, value: i64, seg: &mut [i64], size: usize| {
        // If pos is out of bounds, behavior is undefined per specification;
        // we do not add checks that would change observable behavior.
        let mut idx = size + pos;
        if seg[idx] < value {
            seg[idx] = value;
            idx >>= 1;
            while idx > 0 {
                let left = seg[idx << 1];
                let right = seg[(idx << 1) | 1];
                let m = if left > right { left } else { right };
                if seg[idx] == m {
                    break;
                }
                seg[idx] = m;
                idx >>= 1;
            }
        }
    };

    // Query maximum on inclusive range [l, r].
    let mut query_max = |l: usize, r: usize, seg: &[i64], size: usize| -> i64 {
        // If l/r are out of bounds, behavior is undefined per specification.
        let mut l = l + size;
        let mut r = r + size;
        let mut res = 0i64;
        while l <= r {
            if (l & 1) == 1 {
                if seg[l] > res {
                    res = seg[l];
                }
                l += 1;
            }
            if (r & 1) == 0 {
                if seg[r] > res {
                    res = seg[r];
                }
                if r == 0 {
                    break;
                }
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        res
    };

    // The specification does not define the exact update/query semantics beyond:
    // - process n integers (positions)
    // - maintain maximums with logarithmic updates/queries
    // - output: sum_{k=1..n} k - (maximum value found in a specified range after all updates)
    //
    // Narrowest reasonable interpretation consistent with this:
    // - For each input position p (0-based assumed by "position in the dynamic array"),
    //   perform an update that increments the value at p by 1 (tracked via max structure).
    // - After all updates, query the maximum over the full range [0, n-1].
    //
    // This yields: result = n(n+1)/2 - max_count, where max_count is the maximum frequency
    // of any position among the n inputs.
    //
    // If inputs are 1-based, behavior is undefined by the spec; we do not adjust.
    let mut counts = vec![0i64; n];
    for _ in 0..n {
        let p: usize = it.next().unwrap().parse().unwrap();
        // Undefined if p >= n; we intentionally do not guard.
        counts[p] += 1;
        update(p, counts[p], &mut seg, size);
    }

    let max_in_range = if n == 0 {
        0i64
    } else {
        query_max(0, n - 1, &seg, size)
    };

    // Use i128 for intermediate to avoid overflow in Rust; the spec says "long integer"
    // but does not specify overflow behavior. This is the narrowest safe choice.
    let sum_first_n = (n as i128) * ((n as i128) + 1) / 2;
    let result = sum_first_n - (max_in_range as i128);

    println!("{}", result);
}
