// Translated from provided C code.
// Preserves behavior of the original program (including 1-based node IDs and INF = 100000).
// Uses safe, idiomatic Rust. No `unsafe` required.

use std::io::{self, Read};

const INF: i32 = 100_000;

fn dijkstra(start: i32, goal: i32, n: usize, r: &[[i32; 100]; 100]) -> i32 {
    // Mirrors the C implementation precisely:
    // - arrays of length 100
    // - d[start-1] = 1
    // - choose unvisited node with minimum d among 0..n-1
    let mut d = [INF; 100];
    let mut f = [0i32; 100];

    // Initialize
    for i in 0..100 {
        d[i] = INF;
        f[i] = 0;
    }

    // Convert 1-based start/goal to 0-based indices like C (this will panic on invalid values,
    // similar to how invalid indices in C would lead to undefined behavior).
    let s_idx = (start - 1) as usize;
    let g_idx = (goal - 1) as usize;

    d[s_idx] = 1;

    loop {
        let mut minimum = INF;
        let mut p: usize = 0;
        for i in 0..n {
            if d[i] < minimum && f[i] == 0 {
                minimum = d[i];
                p = i;
            }
        }
        if minimum == INF {
            break;
        }
        f[p] = 1;

        for i in 0..n {
            // preserve C semantics: plain addition and comparison on i32
            let newd = d[p].saturating_add(r[p][i]);
            if newd < d[i] {
                d[i] = newd;
            }
        }
    }

    d[g_idx]
}

fn main() {
    // Read all input tokens as integers, akin to successive scanf calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace();

    // Helper to parse next token as i32, similar to scanf("%d", &x)
    let mut next_i32 = || -> Option<i32> {
        it.next().and_then(|s| s.parse::<i32>().ok())
    };

    // Initialize adjacency matrix r[100][100] = INF
    let mut r = [[INF; 100]; 100];

    // Read n
    let n_i32 = match next_i32() {
        Some(v) => v,
        None => return, // no input
    };
    let n = n_i32 as usize;

    // For each node, read r1 and k1, then k1 destinations
    for _ in 0..n {
        let r1 = match next_i32() {
            Some(v) => v,
            None => return,
        };
        let k1 = match next_i32() {
            Some(v) => v,
            None => return,
        };
        let r_idx = (r1 - 1) as usize;
        for _ in 0..k1 {
            let temp = match next_i32() {
                Some(v) => v,
                None => return,
            };
            let dest = (temp - 1) as usize;
            r[r_idx][dest] = 1;
        }
    }

    // Read p (number of queries)
    let p = match next_i32() {
        Some(v) => v as usize,
        None => return,
    };

    for _ in 0..p {
        let s1 = match next_i32() {
            Some(v) => v,
            None => return,
        };
        let d1 = match next_i32() {
            Some(v) => v,
            None => return,
        };
        let v1 = match next_i32() {
            Some(v) => v,
            None => return,
        };

        let temp = dijkstra(s1, d1, n, &r);
        // Preserve C logic: if (v1 - temp < 0) print "NA", else print temp.
        if v1 - temp < 0 {
            println!("NA");
        } else {
            println!("{}", temp);
        }
    }
}
