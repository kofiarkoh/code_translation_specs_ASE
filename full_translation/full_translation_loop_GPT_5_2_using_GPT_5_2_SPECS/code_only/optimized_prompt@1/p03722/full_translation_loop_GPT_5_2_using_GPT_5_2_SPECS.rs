use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;

#[derive(Clone, Copy)]
struct Edge {
    t: usize,
    h: usize,
    cost: i32,
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // no input
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let w: usize = it.next().unwrap().parse().unwrap();
        let c_in: i32 = it.next().unwrap().parse().unwrap();
        edges.push(Edge {
            t: u,
            h: w,
            cost: -c_in, // matches C: e[i].cost = -c;
        });
    }

    // min[1001] in C, 1-based indexing; keep size n+1 (assumes input respects bounds).
    let mut dist = vec![SUP; n + 1];
    if n >= 1 {
        dist[1] = 0;
    }

    // Bellman-Ford relaxations: k = 1..N-1
    for _k in 1..n {
        for e in &edges {
            let u = e.t;
            let w = e.h;
            let c = e.cost as i64;
            if dist[u] < SUP && dist[w] > dist[u] + c {
                dist[w] = dist[u] + c;
            }
        }
    }

    let ans = dist[n];

    // Extra N iterations to detect further improvement (negative cycle reachability effect).
    for _k in 1..=n {
        for e in &edges {
            let u = e.t;
            let w = e.h;
            let c = e.cost as i64;
            if dist[u] < SUP && dist[w] > dist[u] + c {
                dist[w] = dist[u] + c;
            }
        }
    }

    if dist[n] == ans {
        // C prints -ans as %lld
        println!("{}", -ans);
    } else {
        print!("inf\n");
    }
}
