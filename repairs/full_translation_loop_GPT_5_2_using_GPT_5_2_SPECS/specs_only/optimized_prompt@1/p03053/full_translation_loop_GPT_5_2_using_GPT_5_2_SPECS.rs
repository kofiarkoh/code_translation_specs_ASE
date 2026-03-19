use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    // Fixed limits are preconditions; we do not enforce them.
    // Width < 10000 is a precondition for the original encoding; we do not rely on encoding here.

    let n = h * w;
    let mut dist: Vec<i32> = vec![-1; n]; // -1 means unvisited; distances are non-negative for visited cells.
    let mut q: VecDeque<usize> = VecDeque::with_capacity(n);

    // The specification says there are exactly two cell types: source marker and empty marker,
    // but does not define which character is which. We interpret '#' as source and everything
    // else as empty, which is the narrowest common convention without inventing extra markers.
    for r in 0..h {
        let row = it.next().unwrap();
        let bytes = row.as_bytes();
        // Preconditions guarantee row length == w and only expected cell types.
        for c in 0..w {
            let idx = r * w + c;
            if bytes[c] == b'#' {
                dist[idx] = 0;
                q.push_back(idx);
            }
        }
    }

    // Multi-source BFS over 4-neighborhood.
    while let Some(idx) = q.pop_front() {
        let d = dist[idx];
        let r = idx / w;
        let c = idx % w;

        // Up
        if r > 0 {
            let ni = idx - w;
            if dist[ni] < 0 {
                dist[ni] = d + 1;
                q.push_back(ni);
            }
        }
        // Down
        if r + 1 < h {
            let ni = idx + w;
            if dist[ni] < 0 {
                dist[ni] = d + 1;
                q.push_back(ni);
            }
        }
        // Left
        if c > 0 {
            let ni = idx - 1;
            if dist[ni] < 0 {
                dist[ni] = d + 1;
                q.push_back(ni);
            }
        }
        // Right
        if c + 1 < w {
            let ni = idx + 1;
            if dist[ni] < 0 {
                dist[ni] = d + 1;
                q.push_back(ni);
            }
        }
    }

    // Compute maximum distance. Preconditions guarantee at least one source exists,
    // so all cells should be reachable and have non-negative distances.
    let mut ans: i32 = 0;
    for &d in &dist {
        if d > ans {
            ans = max(ans, d);
        }
    }

    print!("{ans}\n");
}
