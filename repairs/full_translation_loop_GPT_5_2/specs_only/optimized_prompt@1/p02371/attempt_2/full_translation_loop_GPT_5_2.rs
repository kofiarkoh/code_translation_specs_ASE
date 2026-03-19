use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // The execution environment expects stdin->stdout behavior.
    // Narrowest reasonable interpretation from the spec + sample:
    // - First integer: number of vertices V
    // - Then a sequence of triples: (source, destination, cost) until EOF
    // - Starting vertex is 0
    // - Output: maximum path cost from start to any reachable vertex
    //
    // The spec mentions DFS and BFS; we implement a queue-based relaxation (BFS-like)
    // and use a visited/in-queue flag to avoid redundant enqueues.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let v: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };
    if v == 0 {
        // Spec says positive, but keep deterministic behavior.
        println!("0");
        return;
    }

    let mut adj: Vec<Vec<(usize, u64)>> = vec![Vec::new(); v];

    loop {
        let s = match it.next() {
            Some(x) => x,
            None => break,
        };
        let t = match it.next() {
            Some(x) => x,
            None => break,
        };
        let c = match it.next() {
            Some(x) => x,
            None => break,
        };

        let u: usize = s.parse().unwrap_or(0);
        let w: usize = t.parse().unwrap_or(0);
        let cost: u64 = c.parse().unwrap_or(0);

        // Spec says indices are valid; if not, behavior is undefined.
        // We ignore out-of-range edges to keep the program safe.
        if u < v && w < v {
            adj[u].push((w, cost));
        }
    }

    // best[i] = best known maximum cost from start (0) to i; None => unreachable so far.
    let mut best: Vec<Option<u64>> = vec![None; v];
    best[0] = Some(0);

    // in_queue tracks whether a vertex is currently queued (visited status invariant).
    let mut in_queue = vec![false; v];
    let mut q = VecDeque::new();
    q.push_back(0);
    in_queue[0] = true;

    while let Some(u) = q.pop_front() {
        in_queue[u] = false;
        let base = best[u].unwrap_or(0);

        for &(to, w) in &adj[u] {
            let cand = base.saturating_add(w);
            let improve = match best[to] {
                None => true,
                Some(cur) => cand > cur,
            };
            if improve {
                best[to] = Some(cand);
                if !in_queue[to] {
                    in_queue[to] = true;
                    q.push_back(to);
                }
            }
        }
    }

    let mut ans: u64 = 0;
    for b in best.into_iter().flatten() {
        ans = max(ans, b);
    }

    println!("{}", ans);
}
