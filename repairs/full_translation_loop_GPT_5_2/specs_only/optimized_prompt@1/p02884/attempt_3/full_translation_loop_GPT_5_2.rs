use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const INF: i64 = 1_000_000_000; // large constant for distance initialization

fn dijkstra_skip_edge(n: usize, adj: &[Vec<(usize, usize)>], skip: Option<(usize, usize)>) -> Vec<i64> {
    let mut dist = vec![INF; n];
    if n == 0 {
        return dist;
    }
    dist[0] = 0;

    let mut pq: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), 0));

    while let Some((Reverse(d), u)) = pq.pop() {
        if d != dist[u] {
            continue;
        }
        for &(v, eid) in &adj[u] {
            if let Some((su, se)) = skip {
                if su == u && se == eid {
                    continue;
                }
            }
            let nd = d + 1;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push((Reverse(nd), v));
            }
        }
    }

    dist
}

fn average_excluding_start(dist: &[i64]) -> f64 {
    if dist.len() <= 1 {
        return 0.0;
    }
    let mut sum: i128 = 0;
    for &d in &dist[1..] {
        sum += d as i128;
    }
    (sum as f64) / ((dist.len() - 1) as f64)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => {
            println!("{:.10}", 0.0);
            return;
        }
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => 0,
    };

    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for eid in 0..m {
        let a = match it.next() {
            Some(x) => x.parse::<isize>().unwrap_or(0),
            None => break,
        };
        let b = match it.next() {
            Some(x) => x.parse::<isize>().unwrap_or(0),
            None => break,
        };

        // Input node indices are 1-based; convert to 0-based.
        let au = a - 1;
        let bu = b - 1;
        if au >= 0 && bu >= 0 {
            let au = au as usize;
            let bu = bu as usize;
            if au < n && bu < n {
                adj[au].push((bu, eid));
            }
        }
    }

    // The specification says "considering the possibility of removing one specific edge".
    // This is interpreted narrowly as: remove exactly one edge (not "remove none").
    // This matches the sample where removing edge 1->3 yields average 2.0, while removing none yields 4/3.
    let mut best = f64::INFINITY;

    for u in 0..n {
        for &(_v, eid) in &adj[u] {
            let dist = dijkstra_skip_edge(n, &adj, Some((u, eid)));
            let avg = average_excluding_start(&dist);
            if avg < best {
                best = avg;
            }
        }
    }

    // If there are no edges to remove, fall back to the initialized-distance-based computation.
    if !best.is_finite() {
        let dist = dijkstra_skip_edge(n, &adj, None);
        best = average_excluding_start(&dist);
    }

    println!("{:.10}", best);
}
