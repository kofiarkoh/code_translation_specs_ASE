use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

fn main() {
    // Read all input; if empty/malformed, behavior is unspecified by the spec.
    // To avoid runtime panics in such cases, we exit quietly.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let w: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let h: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // Start/goal are not explicitly specified; use the narrow conventional grid choice.
    let start = (0i64, 0i64);
    let goal = (w - 1, h - 1);

    // Build an undirected graph from the connections.
    // Edge weights are not specified; interpret each connection as cost 1.
    let mut adj: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    for _ in 0..m {
        // If the input has fewer integers than expected, stop consuming.
        // Spec says malformed input is undefined; this avoids panics.
        let x1: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let y1: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let x2: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let y2: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        let a = (x1, y1);
        let b = (x2, y2);
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }

    // If start or goal is not defined in the input, treat them as non-existent nodes.
    if !adj.contains_key(&start) || !adj.contains_key(&goal) {
        println!("-1");
        return;
    }

    // Dijkstra's algorithm with a min-heap (BinaryHeap of Reverse).
    let mut dist: HashMap<(i64, i64), i64> = HashMap::new();
    let mut heap: BinaryHeap<(Reverse<i64>, (i64, i64))> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push((Reverse(0), start));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist.get(&u).copied().unwrap_or(i64::MAX) != d {
            continue;
        }
        if u == goal {
            println!("{}", d);
            return;
        }

        if let Some(neis) = adj.get(&u) {
            for &v in neis {
                let nd = d + 1;
                let cur = dist.get(&v).copied().unwrap_or(i64::MAX);
                if nd < cur {
                    dist.insert(v, nd);
                    heap.push((Reverse(nd), v));
                }
            }
        }
    }

    println!("-1");
}
