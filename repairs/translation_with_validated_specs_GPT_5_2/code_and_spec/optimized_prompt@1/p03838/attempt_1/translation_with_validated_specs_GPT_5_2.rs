use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Spec: two signed integers provided as input
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();

    // Minimum operations to transform x -> y using:
    // 1) increment by 1
    // 2) multiply by -1 (sign flip)
    //
    // Consider two strategies:
    // - No flip: only increments, possible iff y >= x, cost = y - x
    // - One flip at some time: flip once (cost 1), then only increments.
    //   If we flip immediately, value becomes -x, then need y - (-x) increments, possible iff y >= -x.
    //   Cost = 1 + (y + x).
    //
    // Flipping later after some increments from x to x+k yields -(x+k) <= -x, which cannot reduce
    // required increments to reach y compared to flipping immediately (since only increments allowed after).
    // Thus, it suffices to take min of the feasible strategies above.
    let mut ans = i64::MAX;

    if y >= x {
        ans = ans.min(y - x);
    }
    if y >= -x {
        ans = ans.min(1 + (y + x));
    }

    // Per spec, result fits in i64 and is non-negative.
    print!("{}", ans);
}
