use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Item {
    w: i64,
    v: i64,
    r: f64,
}

fn zero(a: f64) -> bool {
    a.abs() < 1e-7
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    // Spec says W is a double; parse as f64.
    let mut w_cap: f64 = it.next().unwrap().parse().unwrap();

    let mut ans: f64 = 0.0;
    let mut items: Vec<Item> = Vec::new();

    for _ in 0..n {
        let w_in: i64 = it.next().unwrap().parse().unwrap();
        let v_in: i64 = it.next().unwrap().parse().unwrap();

        // Spec: ignore items with non-positive values and non-negative weights.
        if w_in >= 0 && v_in <= 0 {
            continue;
        }

        // Follow C behavior for negative weights/values.
        if w_in < 0 && v_in > 0 {
            ans += v_in as f64;
            w_cap -= w_in as f64; // subtracting negative increases capacity
        } else {
            let mut w = w_in;
            let mut v = v_in;
            if w < 0 {
                w = -w;
                v = -v;
                ans -= v as f64;
                w_cap += w as f64;
            }
            if w == 0 {
                // Avoid division by zero; C would be undefined here.
                continue;
            }
            items.push(Item {
                w,
                v,
                r: (v as f64) / (w as f64),
            });
        }
    }

    // Sort by descending ratio, with epsilon tie handling like the C comparator.
    items.sort_by(|a, b| {
        if (b.r - a.r).abs() < 1e-8 {
            Ordering::Equal
        } else if a.r <= b.r {
            Ordering::Greater
        } else {
            Ordering::Less
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

    // Match C's printf("%lf\n", ans) formatting: 6 digits after decimal.
    print!("{:.6}\n", ans);
}
