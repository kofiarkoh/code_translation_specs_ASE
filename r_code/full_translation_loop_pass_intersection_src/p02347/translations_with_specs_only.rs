use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    id: u32,
}

fn main() {
    // Read entire stdin; behavior is undefined on invalid/incomplete input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut points: Vec<Point> = Vec::with_capacity(n);
    for id in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        points.push(Point {
            x,
            y,
            id: id as u32,
        });
    }

    // Build an index once: sort points by x (then y, then id for determinism).
    // This does not change point identifiers; it only reorders for query scanning.
    let mut index = points;
    index.sort_unstable_by(|a, b| {
        match a.x.cmp(&b.x) {
            Ordering::Equal => match a.y.cmp(&b.y) {
                Ordering::Equal => a.id.cmp(&b.id),
                other => other,
            },
            other => other,
        }
    });

    let q: usize = it.next().unwrap().parse().unwrap();

    let mut out = String::new();
    let mut results: Vec<u32> = Vec::new();

    for _ in 0..q {
        let x1: i64 = it.next().unwrap().parse().unwrap();
        let x2: i64 = it.next().unwrap().parse().unwrap();
        let y1: i64 = it.next().unwrap().parse().unwrap();
        let y2: i64 = it.next().unwrap().parse().unwrap();

        results.clear();

        // Find the range of points with x in [x1, x2] using binary search.
        // If query bounds violate preconditions, behavior is not guaranteed by spec;
        // we still perform the same comparisons as written.
        let lo = lower_bound_x(&index, x1);
        let hi = upper_bound_x(&index, x2);

        for p in &index[lo..hi] {
            if p.y >= y1 && p.y <= y2 {
                results.push(p.id);
            }
        }

        // Output identifiers in nondecreasing order (ascending id).
        results.sort_unstable();

        for id in &results {
            out.push_str(&id.to_string());
            out.push('\n');
        }
        out.push('\n');
    }

    print!("{out}");
}

fn lower_bound_x(a: &[Point], x: i64) -> usize {
    let mut l = 0usize;
    let mut r = a.len();
    while l < r {
        let m = l + (r - l) / 2;
        if a[m].x < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn upper_bound_x(a: &[Point], x: i64) -> usize {
    let mut l = 0usize;
    let mut r = a.len();
    while l < r {
        let m = l + (r - l) / 2;
        if a[m].x <= x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}
