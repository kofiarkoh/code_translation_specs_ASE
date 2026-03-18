use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Item {
    weight: i64,
    value: i64,
    ratio: f64, // value / weight
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap_or(0),
        None => return,
    };
    let mut capacity: f64 = match it.next() {
        Some(s) => s.parse().unwrap_or(0.0),
        None => return,
    };

    // Preconditions state non-negative; clamp to preserve invariant.
    if capacity < 0.0 {
        capacity = 0.0;
    }

    let mut items: Vec<Item> = Vec::new();

    for _ in 0..n {
        let w: i64 = match it.next() {
            Some(s) => s.parse().unwrap_or(0),
            None => break,
        };
        let v: i64 = match it.next() {
            Some(s) => s.parse().unwrap_or(0),
            None => break,
        };

        // Items with non-positive values and non-negative weights are ignored.
        if v <= 0 && w >= 0 {
            continue;
        }

        // Maintain a list of items with positive value-to-weight ratios for consideration.
        // For zero weight and positive value, treat as +infinity ratio (always take).
        let ratio = if w == 0 {
            if v > 0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            }
        } else {
            (v as f64) / (w as f64)
        };

        if ratio == f64::INFINITY {
            items.push(Item {
                weight: w,
                value: v,
                ratio,
            });
        } else if ratio.is_finite() && ratio > 0.0 {
            items.push(Item {
                weight: w,
                value: v,
                ratio,
            });
        }
    }

    if items.is_empty() {
        // If all items are ignored, the output will be zero.
        println!("0.000000");
        return;
    }

    items.sort_by(|a, b| {
        match (a.ratio.is_nan(), b.ratio.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => b
                .ratio
                .partial_cmp(&a.ratio)
                .unwrap_or(Ordering::Equal),
        }
    });

    let mut total_value: f64 = 0.0;

    for item in items {
        if capacity <= 0.0 {
            break;
        }

        if item.weight == 0 {
            if item.value > 0 {
                total_value += item.value as f64;
            }
            continue;
        }

        // Negative weights are allowed by preconditions but not well-defined for knapsack capacity.
        // Narrowest interpretation consistent with the invariant: ignore them.
        if item.weight < 0 {
            continue;
        }

        let w = item.weight as f64;
        if w <= capacity {
            capacity -= w;
            total_value += item.value as f64;
        } else {
            let fraction = capacity / w;
            total_value += (item.value as f64) * fraction;
            capacity = 0.0;
            break;
        }
    }

    // Match expected formatting: fixed with 6 digits after decimal.
    println!("{:.6}", total_value);
}
