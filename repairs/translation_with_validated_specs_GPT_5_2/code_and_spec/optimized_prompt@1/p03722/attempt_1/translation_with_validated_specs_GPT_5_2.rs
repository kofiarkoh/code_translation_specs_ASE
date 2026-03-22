use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;

#[derive(Clone, Copy)]
struct Edge {
    t: usize,
    h: usize,
    cost: i64, // stored as -c (we run shortest path to compute maximum in original graph)
}

fn main() {
    // Read all input
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
        let u: usize = it.next().unwrap().parse().unwrap();
        let w: usize = it.next().unwrap().parse().unwrap();
        let c: i64 = it.next().unwrap().parse().unwrap();
        edges.push(Edge {
            t: u,
            h: w,
            cost: -c,
        });
    }

    // Bellman-Ford on negated weights:
    // min[v] is the minimum total negated cost from 1 to v,
    // so -min[N] is the maximum total original weight from 1 to N.
    let mut min_dist = vec![SUP; n + 1];
    min_dist[1] = 0;

    for _k in 1..n {
        for e in &edges {
            let u = e.t;
            let w = e.h;
            let c = e.cost;
            if min_dist[u] < SUP {
                let cand = min_dist[u] + c;
                if min_dist[w] > cand {
                    min_dist[w] = cand;
                }
            }
        }
    }

    let ans = min_dist[n];

    // One more phase to detect a reachable negative cycle (in negated graph),
    // which corresponds to a reachable positive cycle in the original graph.
    // Note: This matches the provided C code behavior: it only checks whether
    // min_dist[N] changes after N more relaxations, and does not explicitly
    // verify that the cycle can reach N beyond what affects min_dist[N].
    for _k in 1..=n {
        for e in &edges {
            let u = e.t;
            let w = e.h;
            let c = e.cost;
            if min_dist[u] < SUP {
                let cand = min_dist[u] + c;
                if min_dist[w] > cand {
                    min_dist[w] = cand;
                }
            }
        }
    }

    if min_dist[n] == ans {
        // Finite (or unreachable sentinel): print -ans
        println!("{}", -ans);
    } else {
        println!("inf");
    }
}
