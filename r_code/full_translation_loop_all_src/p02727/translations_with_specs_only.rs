use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Input parsing: if anything is missing or invalid, exit non-zero with no guaranteed output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let x: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let y: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let a: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let b: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let c: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut p: Vec<i64> = Vec::with_capacity(a);
    for _ in 0..a {
        let v: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        p.push(v);
    }

    let mut q: Vec<i64> = Vec::with_capacity(b);
    for _ in 0..b {
        let v: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        q.push(v);
    }

    let mut r: Vec<i64> = Vec::with_capacity(c);
    for _ in 0..c {
        let v: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        r.push(v);
    }

    // Selection state: choose the best X from first group and best Y from second group.
    // We sort descending and take prefixes.
    p.sort_unstable_by(|u, v| v.cmp(u));
    q.sort_unstable_by(|u, v| v.cmp(u));
    r.sort_unstable_by(|u, v| v.cmp(u));

    // Maintain "currently chosen" items as a min-heap so we can replace the smallest chosen item
    // with a better third-group item, never decreasing total.
    let mut chosen: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut sum: i64 = 0;

    for &v in p.iter().take(x) {
        chosen.push(Reverse(v));
        sum += v;
    }
    for &v in q.iter().take(y) {
        chosen.push(Reverse(v));
        sum += v;
    }

    // Replacement state: iterate third group in descending order; apply improving replacements.
    for &v in &r {
        if let Some(&Reverse(min_chosen)) = chosen.peek() {
            if v > min_chosen {
                chosen.pop();
                chosen.push(Reverse(v));
                sum += v - min_chosen;
            }
        } else {
            // If X+Y == 0, nothing is chosen; no replacements are meaningful.
            break;
        }
    }

    // Output: maximum total sum as signed 64-bit integer.
    println!("{}", sum);
}
