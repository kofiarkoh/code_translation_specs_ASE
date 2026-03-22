use std::io::{self, Read};

const C: i64 = 200_000_000_000_000_000;

fn predicate(points: &[(i64, i64)], a: i64, b: i64) -> i32 {
    // P(A,B): XOR over contributing indices of t_i, where
    // t_i = 1 iff every bit set in q_i is also set in p_i.
    // Contributing indices require A - x_i >= 0 and B - y_i >= 0.
    let mut parity: i32 = 0;
    for &(x, y) in points {
        if a >= x && b >= y {
            // Spec assumes these intermediate computations stay within i64 range.
            let dx = a - x;
            let dy = b - y;
            let p = dx + dy;
            let q = dy;

            // "every bit set in q is also set in p" <=> (q & !p) == 0
            // Use i64 bitwise ops directly; values are nonnegative by construction here.
            let t = if (q & !p) == 0 { 1 } else { 0 };
            parity ^= t;
        }
    }
    parity
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut points: Vec<(i64, i64)> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    // Bitwise-constructed search for maximum r in [0, 2^61 - 1] such that P(C - r, r) == 1.
    // Construct r from high bit to low bit, keeping the invariant that r stays in range.
    let mut r: i64 = 0;
    for bit in (0..=60).rev() {
        let candidate = r | (1_i64 << bit);
        // candidate is always within [0, 2^61 - 1] by construction.
        let a = C - candidate;
        let b = candidate;
        if predicate(&points, a, b) == 1 {
            r = candidate;
        }
    }

    let out = C - r;
    print!("{out}\n");
}
