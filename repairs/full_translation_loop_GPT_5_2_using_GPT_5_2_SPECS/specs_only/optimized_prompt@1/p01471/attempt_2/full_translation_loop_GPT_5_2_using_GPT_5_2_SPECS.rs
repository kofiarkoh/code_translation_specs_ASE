use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Item {
    w: f64, // effective nonnegative weight for fractional selection
    v: f64, // effective value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Capacity is treated as real-valued.
    let mut cap: f64 = it.next().unwrap().parse::<f64>().unwrap();

    let mut base_value: f64 = 0.0;
    let mut eligible: Vec<Item> = Vec::new();
    eligible.reserve(n);

    for _ in 0..n {
        let w_i: i64 = it.next().unwrap().parse().unwrap();
        let v_i: i64 = it.next().unwrap().parse().unwrap();

        // Items with nonnegative weight and nonpositive value are ignored.
        if w_i >= 0 && v_i <= 0 {
            continue;
        }

        // Items with negative weight and positive value are always taken in full.
        if w_i < 0 && v_i > 0 {
            base_value += v_i as f64;
            cap += (-w_i) as f64;
            continue;
        }

        // All other items are transformed, if needed, into an equivalent representation with nonnegative weight.
        // Narrow interpretation: if weight is negative, flip both signs.
        let (ew, ev) = if w_i < 0 {
            (-(w_i as f64), -(v_i as f64))
        } else {
            (w_i as f64, v_i as f64)
        };

        eligible.push(Item { w: ew, v: ev });
    }

    // Sort by value density descending.
    eligible.sort_by(|a, b| {
        let ra = a.v / a.w;
        let rb = b.v / b.w;
        match rb.partial_cmp(&ra) {
            Some(ord) => ord,
            None => Ordering::Equal, // precondition violation (NaN) => unspecified; keep deterministic
        }
    });

    let tol: f64 = 1e-9;
    let mut total: f64 = base_value;

    for item in eligible {
        if cap <= tol {
            break;
        }
        if item.w <= 0.0 {
            // Precondition violation (zero effective weight for eligible item) => unspecified; skip.
            continue;
        }

        if item.w <= cap + tol {
            cap -= item.w;
            total += item.v;
        } else {
            let frac = (cap / item.w).clamp(0.0, 1.0);
            total += item.v * frac;
            cap = 0.0;
            break;
        }
    }

    // Observed required formatting: fixed with 6 digits after decimal.
    println!("{:.6}", total);
}
