use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin
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

    // Graph: adjacency list, 1-based vertices.
    // C uses realloc per edge; in Rust we use Vec<Vec<usize>>.
    let mut es: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut indeg: Vec<i32> = vec![0; n + 1];

    let edge_count = n
        .checked_sub(1)
        .and_then(|x| x.checked_add(m))
        .unwrap_or_else(|| process::exit(1));

    for _ in 0..edge_count {
        let a: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        // Per spec, labels are within 1..=N; behavior undefined otherwise.
        es[a].push(b);
        indeg[b] += 1;
    }

    // Two-phase buffers (like the C code) to process "layers" of zero-indegree vertices.
    let mut next0: Vec<usize> = Vec::new();
    let mut next1: Vec<usize> = Vec::new();

    for v in 1..=n {
        if indeg[v] == 0 {
            next0.push(v);
        }
    }

    let mut answer: Vec<usize> = vec![0; n + 1];

    // Note: The C code overwrites answer[v] if v reaches indeg==0 multiple times,
    // but that cannot happen because indeg only decreases and hits 0 once.
    // This matches the spec invariant "once nonzero, not changed thereafter".
    let mut phase = 0;
    while (phase == 0 && !next0.is_empty()) || (phase == 1 && !next1.is_empty()) {
        if phase == 0 {
            next1.clear();
            for &target in &next0 {
                for &to in &es[target] {
                    indeg[to] -= 1;
                    if indeg[to] == 0 {
                        answer[to] = target;
                        next1.push(to);
                    }
                }
            }
        } else {
            next0.clear();
            for &target in &next1 {
                for &to in &es[target] {
                    indeg[to] -= 1;
                    if indeg[to] == 0 {
                        answer[to] = target;
                        next0.push(to);
                    }
                }
            }
        }
        phase ^= 1;
    }

    let mut out = String::new();
    for v in 1..=n {
        out.push_str(&format!("{}\n", answer[v]));
    }
    print!("{out}");
}
