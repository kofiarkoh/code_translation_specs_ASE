use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin; if missing required tokens, exit non-zero (like C returning 1).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let m: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Fixed internal capacity in the C code is 114514; we keep the same bound.
    // Preconditions guarantee n and edge counts fit; no extra validation is required.
    const CAP: usize = 114_514;

    // Adjacency list: Vec<Vec<usize>> is safe and idiomatic.
    // Note: C exits with status 2 on realloc failure; Rust OOM aborts by default.
    // We do not attempt to emulate exit(2) on OOM because Rust does not provide a
    // reliable, safe way to intercept allocation failure in stable std.
    let mut es: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut indeg: Vec<i32> = vec![0; n + 1];
    let mut answer: Vec<usize> = vec![0; n + 1];

    let total_edges = (n - 1).saturating_add(m);
    for _ in 0..total_edges {
        let a: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        // No range validation per spec (undefined behavior if violated in C).
        es[a].push(b);
        indeg[b] += 1;
    }

    // Two-phase frontier buffers, matching the C code's behavior.
    let mut next0: Vec<usize> = Vec::new();
    let mut next1: Vec<usize> = Vec::new();

    for v in 1..=n {
        if indeg[v] == 0 {
            next0.push(v);
        }
    }

    let mut phase = 0;
    while (if phase == 0 { !next0.is_empty() } else { !next1.is_empty() }) {
        if phase == 0 {
            next1.clear();
            for &u in &next0 {
                for &v in &es[u] {
                    indeg[v] -= 1;
                    if indeg[v] == 0 {
                        answer[v] = u; // record trigger predecessor exactly once
                        next1.push(v);
                    }
                }
            }
        } else {
            next0.clear();
            for &u in &next1 {
                for &v in &es[u] {
                    indeg[v] -= 1;
                    if indeg[v] == 0 {
                        answer[v] = u;
                        next0.push(v);
                    }
                }
            }
        }
        phase ^= 1;
    }

    // Output exactly one integer per line for nodes 1..=N.
    let mut out = String::new();
    for v in 1..=n {
        out.push_str(&format!("{}\n", answer[v]));
    }
    print!("{out}");

    // Keep CAP referenced to reflect the fixed-capacity nature from the spec/C.
    // (No runtime effect; avoids unused constant warning if CAP isn't otherwise used.)
    let _ = CAP;
}
