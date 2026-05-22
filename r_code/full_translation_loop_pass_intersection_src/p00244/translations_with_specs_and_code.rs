use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Default)]
struct Adj {
    to: Vec<usize>,
    cost: Vec<i32>,
}

fn search(tbl: &[Adj], start: usize, goal: usize) -> i32 {
    // node[v][c] = best known cost to reach v with exactly c remaining waivers, or -1 if unknown.
    let n = tbl.len();
    let mut node = vec![vec![-1i32; 3]; n];

    // Min-heap by total cost t. State: (t, vertex, remaining_waivers)
    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    node[start][2] = 0;
    pq.push((Reverse(0), start, 2));

    while let Some((Reverse(_t), s, c)) = pq.pop() {
        // Skip stale entries (Dijkstra-style).
        let cur = node[s][c];
        if cur < 0 {
            continue;
        }

        // Matches C behavior: stop when we pop goal with c != 1.
        if s == goal && c != 1 {
            break;
        }

        let adj = &tbl[s];
        for i in 0..adj.to.len() {
            let s2 = adj.to[i];
            let f2 = adj.cost[i];

            // Transition without using a waiver: only permitted when remaining waivers != 1.
            if c != 1 {
                let k = cur.saturating_add(f2);
                if node[s2][c] < 0 || k < node[s2][c] {
                    node[s2][c] = k;
                    pq.push((Reverse(k), s2, c));
                }
            }

            // Transition using a waiver (if any remain): cost does not increase.
            if c > 0 {
                let c2 = c - 1;
                let k = cur;
                if node[s2][c2] < 0 || k < node[s2][c2] {
                    node[s2][c2] = k;
                    pq.push((Reverse(k), s2, c2));
                }
            }
        }
    }

    // Final answer: min(cost at goal with remaining waivers 0 or 2).
    // Destination with remaining waivers == 1 is not accepted.
    let a0 = node[goal][0];
    let a2 = node[goal][2];
    match (a0 >= 0, a2 >= 0) {
        (true, true) => a0.min(a2),
        (true, false) => a0,
        (false, true) => a2,
        (false, false) => {
            // Spec says unreachable behavior is undefined; C would return an invalid value.
            // We mirror that loosely by returning -1 if both are unknown.
            -1
        }
    }
}

fn main() {
    // Read all input, parse as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let size_opt = it.next();
        if size_opt.is_none() {
            break;
        }
        let size: usize = size_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        if size == 0 {
            break;
        }

        let mut tbl: Vec<Adj> = vec![Adj::default(); size];
        for _ in 0..m {
            let mut a: usize = it.next().unwrap().parse().unwrap();
            let mut b: usize = it.next().unwrap().parse().unwrap();
            let c: i32 = it.next().unwrap().parse().unwrap();
            // Input is 1-based in the C code.
            a -= 1;
            b -= 1;

            tbl[a].to.push(b);
            tbl[a].cost.push(c);
            tbl[b].to.push(a);
            tbl[b].cost.push(c);
        }

        let ans = search(&tbl, 0, size - 1);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
