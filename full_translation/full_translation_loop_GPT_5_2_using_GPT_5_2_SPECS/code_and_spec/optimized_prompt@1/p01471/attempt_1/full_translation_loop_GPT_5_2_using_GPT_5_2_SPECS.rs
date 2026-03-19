use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    w: i32,
    v: i32,
    r: f64, // value/weight ratio
}

fn zero(a: f64) -> bool {
    a.abs() < 1e-7
}

fn main() {
    // Read entire stdin, parse as integers (C code uses a fast int reader).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut w_cap: f64 = it.next().unwrap().parse::<i32>().unwrap() as f64;

    let mut items: Vec<Item> = Vec::new();
    let mut ans: f64 = 0.0;

    for _ in 0..n {
        let mut w: i32 = it.next().unwrap().parse().unwrap();
        let mut v: i32 = it.next().unwrap().parse().unwrap();

        // Spec: items with nonnegative weight and nonpositive value are ignored.
        if w >= 0 && v <= 0 {
            continue;
        }

        // Spec: items with negative weight and positive value are always taken in full.
        if w < 0 && v > 0 {
            ans += v as f64;
            // subtracting a negative weight increases capacity
            w_cap -= w as f64;
            continue;
        }

        // Remaining cases are eligible for fractional selection after transformation.
        // This matches the C code's transformation:
        // if w < 0 then flip signs and adjust ans and capacity.
        if w < 0 {
            w = -w;
            v = -v;
            // C does: ans -= v (after v is negated), which equals ans += original_v.
            ans -= v as f64;
            w_cap += w as f64;
        }

        // Preconditions: effective weight for eligible items must be nonzero.
        // We do not add extra error handling; behavior is undefined if violated (per spec).
        let r = (v as f64) / (w as f64);
        items.push(Item { w, v, r });
    }

    // Sort by descending ratio r, with tolerance similar to C comparator.
    items.sort_by(|a, b| {
        let diff = b.r - a.r;
        if diff.abs() < 1e-8 {
            Ordering::Equal
        } else if a.r <= b.r {
            Ordering::Greater // a after b
        } else {
            Ordering::Less // a before b
        }
    });

    for item in items {
        if zero(w_cap) {
            break;
        }
        let mut x = w_cap / (item.w as f64);
        if x > 1.0 {
            x = 1.0;
        }
        w_cap -= x * (item.w as f64);
        ans += x * (item.v as f64);
    }

    // Match C's printf("%lf\n", ans);
    println!("{:.6}", ans);
}
