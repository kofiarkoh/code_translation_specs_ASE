// translated from the provided C code according to the SPEC.
// Note: The original C logic would print the sentinel value (100000) if the
// computed distance equals the sentinel and the query limit v1 >= 100000.
// The SPEC requires that unreachable distances (the sentinel) be treated as
// infinite and printed as "NA". This Rust translation follows the SPEC
// (i.e., prints "NA" whenever the returned cost is the sentinel), which
// is a deliberate deviation from the original C program in that specific case.

use std::io::{self, Read};

const MAX_NODES: usize = 100;
const SENTINEL: i32 = 100000;

fn dijkstra(start0: usize, goal0: usize, n: usize, r: &[[i32; MAX_NODES]; MAX_NODES]) -> i32 {
    // start0 and goal0 are 0-based indices.
    let mut d = [SENTINEL; MAX_NODES];
    let mut visited = [false; MAX_NODES];

    d[start0] = 1; // source contributes 1

    loop {
        let mut minimum = SENTINEL;
        let mut p: usize = 0;
        // find unvisited node with smallest d
        for i in 0..n {
            if !visited[i] && d[i] < minimum {
                minimum = d[i];
                p = i;
            }
        }
        if minimum == SENTINEL {
            break;
        }
        visited[p] = true;

        // relax edges from p to all nodes (only consider first n nodes)
        for i in 0..n {
            // r[p][i] is either 1 for an edge or SENTINEL for no edge
            let tentative = d[p].saturating_add(r[p][i]); // safe addition
            if tentative < d[i] {
                d[i] = tentative;
            }
        }
    }

    d[goal0]
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Initialize adjacency matrix with SENTINEL.
    let mut r = [[SENTINEL; MAX_NODES]; MAX_NODES];

    // Read n
    let n_i32 = match iter.next() {
        Some(v) => v,
        None => return, // no input; per SPEC behavior is undefined but we exit
    };
    let n = n_i32 as usize;

    // For each of n nodes, read r1 and k1 and k1 neighbors
    for _ in 0..n {
        let r1 = iter.next().expect("missing r1") as usize; // 1-based
        let k1 = iter.next().expect("missing k1") as usize;
        for _ in 0..k1 {
            let temp = iter.next().expect("missing neighbor") as usize; // 1-based
            // set directed edge r1-1 -> temp-1 with cost 1
            r[r1 - 1][temp - 1] = 1;
        }
    }

    // Read p queries
    let p = match iter.next() {
        Some(v) => v as usize,
        None => 0,
    };

    for _ in 0..p {
        let s1 = iter.next().expect("missing s1") as usize; // 1-based
        let d1 = iter.next().expect("missing d1") as usize; // 1-based
        let v1 = iter.next().expect("missing v1") as i32;

        let cost = dijkstra(s1 - 1, d1 - 1, n, &r);

        // Per SPEC: print the computed cost only if cost is finite (< SENTINEL)
        // and cost <= v1. Otherwise print "NA".
        if cost < SENTINEL && cost <= v1 {
            println!("{}", cost);
        } else {
            println!("NA");
        }
    }
}
