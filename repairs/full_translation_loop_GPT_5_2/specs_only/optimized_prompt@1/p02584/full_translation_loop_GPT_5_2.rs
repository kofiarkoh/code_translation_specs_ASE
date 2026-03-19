use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let x: i128 = it.next().unwrap().parse().unwrap();
    let d: i128 = it.next().unwrap().parse().unwrap();
    let k: i128 = it.next().unwrap().parse().unwrap();

    // Divisor must be non-zero per specification; behavior is undefined otherwise.
    //
    // Operation consistent with sample:
    // Starting from |x|, apply the operation "replace value with |value - d|" exactly k times.
    // This yields the minimal possible distance to a multiple of d after up to k steps, and can be
    // computed in O(1):
    //   Let a = |x|, r = a % d, m = min(r, d - r).
    //   If k == 0 => a
    //   Else if a / d >= k => a - k*d
    //   Else => m
    //
    // For the last case: once you can reach below d, further applications can only toggle between
    // r and d-r, so the minimum achievable is min(r, d-r).
    let a: i128 = if x == i128::MIN { i128::MAX } else { x.abs() };

    if k == 0 {
        print!("{a}");
        return;
    }

    let q = a / d;
    let result = if k <= q {
        a - k * d
    } else {
        let r = a % d;
        let dr = d - r;
        if r < dr { r } else { dr }
    };

    print!("{result}");
}
