use std::io::{self, Read};

fn next_usize(it: &mut std::str::SplitWhitespace<'_>) -> usize {
    it.next().unwrap().parse::<usize>().unwrap()
}

fn next_i64(it: &mut std::str::SplitWhitespace<'_>) -> i64 {
    it.next().unwrap().parse::<i64>().unwrap()
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n = next_usize(&mut it);
    let m = next_usize(&mut it);
    let r = next_usize(&mut it);

    // Specified vertices to visit (indices are assumed valid per spec).
    // The specification does not state 0-based vs 1-based explicitly.
    // We choose the narrowest common interpretation for such problems: 1-based input.
    // If input is actually 0-based, behavior is unspecified by the spec.
    let mut special = Vec::with_capacity(r);
    for _ in 0..r {
        let v = next_usize(&mut it);
        special.push(v - 1);
    }

    // Large sentinel for unreachable distances.
    // Must be large enough to dominate any real path sum under the spec's "no overflow" precondition.
    let inf: i64 = 1_i64 << 60;

    // Initialize all-pairs distances.
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Read edges; undirected; keep the minimum cost if multiple edges exist.
    for _ in 0..m {
        let a = next_usize(&mut it) - 1;
        let b = next_usize(&mut it) - 1;
        let c = next_i64(&mut it);
        if c < dist[a][b] {
            dist[a][b] = c;
            dist[b][a] = c;
        }
    }

    // Floyd–Warshall all-pairs shortest paths.
    // Use sentinel-aware addition to avoid inf + x overflow; spec says no overflow for valid inputs.
    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik >= inf {
                continue;
            }
            for j in 0..n {
                let dkj = dist[k][j];
                if dkj >= inf {
                    continue;
                }
                let cand = dik + dkj;
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // Evaluate all permutations of the specified vertices; r <= 8.
    // Minimum sum of consecutive shortest-path distances; no return to start.
    special.sort_unstable();
    let mut best = inf;

    loop {
        let mut sum = 0_i64;
        for i in 0..r.saturating_sub(1) {
            let u = special[i];
            let v = special[i + 1];
            sum += dist[u][v];
        }
        if sum < best {
            best = sum;
        }

        if !next_permutation(&mut special) {
            break;
        }
    }

    println!("{}", best);
}

// Lexicographic next_permutation for usize slice.
// Returns false if it was the last permutation (i.e., now unchanged).
fn next_permutation(a: &mut [usize]) -> bool {
    if a.len() < 2 {
        return false;
    }
    let mut i = a.len() - 2;
    while a[i] >= a[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = a.len() - 1;
    while a[i] >= a[j] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
