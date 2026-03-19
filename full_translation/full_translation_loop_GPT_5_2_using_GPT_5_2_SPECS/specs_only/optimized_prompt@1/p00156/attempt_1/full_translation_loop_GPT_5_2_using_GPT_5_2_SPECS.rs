use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification assumes well-formed input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break; // No more input; unspecified, but exit quietly.
        }
        let w: usize = w_opt.unwrap().parse().unwrap();
        let h: usize = it.next().unwrap().parse().unwrap();
        if w == 0 && h == 0 {
            break;
        }

        // Grid is provided as characters; split_whitespace implies each row is a token.
        // Specification: input provides required number of grid characters.
        let mut grid: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
        let mut start: Option<(usize, usize)> = None;

        for y in 0..h {
            let row = it.next().unwrap();
            let bytes = row.as_bytes();
            // Narrow interpretation: each row token contains exactly w characters.
            for x in 0..w {
                let c = bytes[x];
                grid[y][x] = c;
                // Specification: exactly one designated start cell marked by a specific character.
                // The specific character is not stated; choose the narrowest common convention: 'S'.
                if c == b'S' {
                    start = Some((x, y));
                }
            }
        }

        let (sx, sy) = start.unwrap();

        // Dijkstra on 0/1 edge weights (0 if same char, 1 otherwise).
        // Use a binary heap for correctness without assuming a particular update strategy.
        let inf: i32 = i32::MAX / 4;
        let mut dist: Vec<i32> = vec![inf; w * h];
        let idx = |x: usize, y: usize, w: usize| -> usize { y * w + x };

        let sidx = idx(sx, sy, w);
        dist[sidx] = 0;

        let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), sidx));

        let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((Reverse(d), u)) = heap.pop() {
            if d != dist[u] {
                continue;
            }
            let ux = u % w;
            let uy = u / w;
            let uc = grid[uy][ux];

            for (dx, dy) in dirs {
                let nx = ux as isize + dx;
                let ny = uy as isize + dy;
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    continue;
                }
                let nxu = nx as usize;
                let nyu = ny as usize;
                let v = idx(nxu, nyu, w);
                let vc = grid[nyu][nxu];
                let wgt: i32 = if vc == uc { 0 } else { 1 };
                let nd = d.saturating_add(wgt);
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push((Reverse(nd), v));
                }
            }
        }

        // Minimum cost to any boundary cell.
        let mut cmin: i32 = inf;
        for y in 0..h {
            for x in 0..w {
                if x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                    let d = dist[idx(x, y, w)];
                    if d < cmin {
                        cmin = d;
                    }
                }
            }
        }

        // Specification: output floor(Cmin / 2) using integer division.
        // If unreachable boundary exists (cmin == inf), behavior is not inferable; we still compute.
        let ans = cmin / 2;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
