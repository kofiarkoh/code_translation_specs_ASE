use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

fn dijkstra(
    graph: &[Vec<(usize, i32)>],
    lpg: &[bool],
    cap: usize,
    start: usize,
    goal: usize,
) -> i32 {
    // dist[city][remaining_range] = min total distance
    // Use a large sentinel similar in spirit to the C memset(0x10, ...)
    let inf: i32 = i32::MAX / 4;
    let n = graph.len();
    let mut dist = vec![vec![inf; cap + 1]; n];

    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
    dist[start][cap] = 0;
    pq.push((Reverse(0), start, cap));

    while let Some((Reverse(d), s, c0)) = pq.pop() {
        if s == goal {
            return d;
        }
        if dist[s][c0] < d {
            continue;
        }

        // Refuel happens when at a refueling city, before taking outgoing roads.
        let c = if lpg[s] { cap } else { c0 };

        for &(e, w) in &graph[s] {
            if w < 0 {
                // Spec says behavior unspecified; input assumed nonnegative.
                continue;
            }
            let w_usize = w as usize;
            if c < w_usize {
                continue;
            }
            let nc = c - w_usize;
            let nd = d + w;
            if nd < dist[e][nc] {
                dist[e][nc] = nd;
                pq.push((Reverse(nd), e, nc));
            }
        }
    }
    -1
}

fn main() {
    // Token-based input: whitespace-delimited, line boundaries irrelevant.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // no more datasets
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // termination condition
        }
        let m: usize = it.next().unwrap().parse().unwrap();
        let u: usize = it.next().unwrap().parse().unwrap();
        let cap: usize = 10 * u;

        let origin = it.next().unwrap().to_string();
        let destination = it.next().unwrap().to_string();

        // Map city names to ids; origin=0, destination=1 (as in C).
        let mut id: HashMap<String, usize> = HashMap::new();
        id.insert(origin, 0);
        id.insert(destination, 1);
        let mut next_id: usize = 2;

        // Read roads, building adjacency list on the fly.
        let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(), Vec::new()];

        for _ in 0..n {
            let a_name = it.next().unwrap().to_string();
            let b_name = it.next().unwrap().to_string();
            let d: i32 = it.next().unwrap().parse().unwrap();

            let a = *id.entry(a_name).or_insert_with(|| {
                let nid = next_id;
                next_id += 1;
                graph.push(Vec::new());
                nid
            });
            let b = *id.entry(b_name).or_insert_with(|| {
                let nid = next_id;
                next_id += 1;
                graph.push(Vec::new());
                nid
            });

            // Undirected
            graph[a].push((b, d));
            graph[b].push((a, d));
        }

        // Refueling cities: tokens that don't match known cities are ignored.
        let mut lpg = vec![false; next_id];
        for _ in 0..m {
            let name = it.next().unwrap();
            if let Some(&cid) = id.get(name) {
                lpg[cid] = true;
            }
        }

        let ans = dijkstra(&graph, &lpg, cap, 0, 1);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
