use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn search(tbl: &Vec<Vec<(usize, i32)>>, start: usize, goal: usize) -> i32 {
    // node[v][c] = best known cost to reach v with c free transitions remaining.
    // c ranges 0..=2. -1 means unreachable.
    let n = tbl.len();
    let mut node = vec![vec![-1i32; 3]; n];

    // Priority queue ordered by smallest total cost (Dijkstra-like).
    // State: (total_cost, vertex, free_remaining)
    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    node[start][2] = 0;
    pq.push((Reverse(0), start, 2));

    while let Some((Reverse(_t), s, c)) = pq.pop() {
        // The original C code does not discard stale queue entries; it uses node[s][c]
        // as the authoritative distance when relaxing edges. We preserve that behavior.
        if s == goal {
            if c != 1 {
                break;
            }
        }

        let cur = node[s][c];
        if cur < 0 {
            // Shouldn't happen given how we push, but keep behavior robust.
            continue;
        }

        for &(s2, f2) in &tbl[s] {
            // Paid transition (only if c != 1)
            if c != 1 {
                let k = cur + f2;
                if node[s2][c] < 0 || k < node[s2][c] {
                    node[s2][c] = k;
                    pq.push((Reverse(k), s2, c));
                }
            }

            // Free transition (if any free transitions remain)
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

    // Match C's return logic exactly.
    let g0 = node[goal][0];
    let g2 = node[goal][2];
    if g0 >= 0 && g2 >= 0 {
        g0.min(g2)
    } else if g0 >= 0 {
        g0
    } else {
        g2
    }
}

fn main() {
    // Read all input; parse as integers.
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

        let mut tbl: Vec<Vec<(usize, i32)>> = vec![Vec::new(); size];
        for _ in 0..m {
            let mut a: usize = it.next().unwrap().parse().unwrap();
            let mut b: usize = it.next().unwrap().parse().unwrap();
            let c: i32 = it.next().unwrap().parse().unwrap();
            // Convert to 0-based indices as in C.
            a -= 1;
            b -= 1;
            tbl[a].push((b, c));
            tbl[b].push((a, c));
        }

        let start = 0usize;
        let goal = size - 1;
        let ans = search(&tbl, start, goal);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
