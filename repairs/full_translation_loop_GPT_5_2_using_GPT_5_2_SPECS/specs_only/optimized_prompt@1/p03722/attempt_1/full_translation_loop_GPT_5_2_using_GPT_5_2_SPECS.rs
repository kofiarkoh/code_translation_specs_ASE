use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
    w: i64,
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let w: i64 = it.next().unwrap().parse().unwrap();
        edges.push(Edge { a, b, w });
    }

    // Sentinel chosen to be a "large negative value" for unreachable nodes.
    // The spec says that if no path exists, the program outputs a large negative
    // value derived from its internal sentinel (not a meaningful indicator).
    // We use a typical very-negative i64 sentinel.
    let neg_inf: i64 = -9_000_000_000_000_000_000i64;

    // Best-known maximum path total to each node (1-indexed).
    let mut dist = vec![neg_inf; n + 1];
    if n >= 1 {
        dist[1] = 0;
    }

    // Initial stabilization phase: relax edges N-1 times for maximum path.
    // (Bellman-Ford style, but maximizing instead of minimizing.)
    for _ in 0..n.saturating_sub(1) {
        let mut changed = false;
        for e in &edges {
            if e.a <= n && e.b <= n {
                let da = dist[e.a];
                if da != neg_inf {
                    let cand = da + e.w; // Spec says overflow is not guaranteed-safe; we assume it doesn't happen.
                    if cand > dist[e.b] {
                        dist[e.b] = cand;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Infinity detection phase:
    // Perform additional relaxations; if node N can still be improved after
    // sufficient steps, report "inf".
    //
    // The spec frames this as "whether the best-known value for node N can still
    // be improved after sufficient relaxation steps", which corresponds to
    // detecting a reachable positive cycle that can still lead to N.
    let mut infinite = false;
    for _ in 0..n {
        let mut improved_n = false;
        for e in &edges {
            if e.a <= n && e.b <= n {
                let da = dist[e.a];
                if da != neg_inf {
                    let cand = da + e.w;
                    if cand > dist[e.b] {
                        dist[e.b] = cand;
                        if e.b == n {
                            improved_n = true;
                        }
                    }
                }
            }
        }
        if improved_n {
            infinite = true;
            break;
        }
    }

    if infinite {
        print!("inf");
    } else {
        // If no path exists, dist[n] remains the sentinel-derived large negative value.
        // This matches the spec's postcondition.
        if n <= dist.len() - 1 {
            print!("{}", dist[n]);
        } else {
            // Narrowest reasonable behavior if n==0 (not intended by spec): print sentinel.
            print!("{}", neg_inf);
        }
    }
}
