use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification provides no error handling requirements.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    // Distances: None = unreached, Some(d) = finalized shortest distance.
    // This matches the "unreached -> reached with finite distance" transition.
    let mut dist: Vec<Option<u32>> = vec![None; h * w];
    let mut q: VecDeque<usize> = VecDeque::new();

    for r in 0..h {
        let row = it.next().unwrap();
        // Preconditions guarantee row length == w and only expected cell types.
        for (c, ch) in row.bytes().take(w).enumerate() {
            let idx = r * w + c;
            // The specification does not name the two cell types; interpret "source"
            // as '#', and "empty" as '.' (common convention). Behavior is unspecified
            // for other characters per spec.
            if ch == b'#' {
                dist[idx] = Some(0);
                q.push_back(idx);
            }
        }
    }

    // Multi-source BFS over orthogonal neighbors.
    while let Some(idx) = q.pop_front() {
        let d = dist[idx].unwrap();
        let r = idx / w;
        let c = idx % w;

        // Up
        if r > 0 {
            let n = idx - w;
            if dist[n].is_none() {
                dist[n] = Some(d + 1);
                q.push_back(n);
            }
        }
        // Down
        if r + 1 < h {
            let n = idx + w;
            if dist[n].is_none() {
                dist[n] = Some(d + 1);
                q.push_back(n);
            }
        }
        // Left
        if c > 0 {
            let n = idx - 1;
            if dist[n].is_none() {
                dist[n] = Some(d + 1);
                q.push_back(n);
            }
        }
        // Right
        if c + 1 < w {
            let n = idx + 1;
            if dist[n].is_none() {
                dist[n] = Some(d + 1);
                q.push_back(n);
            }
        }
    }

    // Compute maximum distance; precondition guarantees at least one source exists,
    // and thus all cells are reachable in a rectangular grid with orthogonal moves.
    let mut ans: u32 = 0;
    for d in dist {
        if let Some(v) = d {
            ans = max(ans, v);
        }
    }

    println!("{}", ans);
}
