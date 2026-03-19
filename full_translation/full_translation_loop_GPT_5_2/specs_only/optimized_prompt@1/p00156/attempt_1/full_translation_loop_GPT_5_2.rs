use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: usize = w_opt.unwrap().parse().unwrap();
        let h: usize = it.next().unwrap().parse().unwrap();

        if w == 0 && h == 0 {
            break;
        }

        // Grid size constraint is stated as max 101x101; we do not enforce it beyond allocation.
        let mut grid: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
        let mut start: Option<(usize, usize)> = None;

        for y in 0..h {
            // The specification says "grid contents" but not the exact formatting.
            // Narrow interpretation: each row is provided as a token (no spaces) of length w.
            let row = it.next().unwrap_or("");
            let bytes = row.as_bytes();
            for x in 0..w {
                let c = if x < bytes.len() { bytes[x] } else { b' ' };
                grid[y][x] = c;

                // The specification does not name the starting character.
                // Narrowest reasonable interpretation: the start is marked by 'S'.
                if c == b'S' && start.is_none() {
                    start = Some((x, y));
                }
            }
        }

        // If no start exists, behavior is undefined per spec; we avoid inventing behavior.
        let (sx, sy) = start.unwrap();

        // We model "transition count ... based on the character type encountered" as:
        // moving to a neighbor costs 0 if the character is the same, else 1.
        // This matches "character comparisons" and yields a non-negative integer.
        let inf: u32 = u32::MAX / 4;
        let mut dist: Vec<Vec<u32>> = vec![vec![inf; w]; h];
        dist[sy][sx] = 0;

        // Dijkstra with 0/1 edge weights (could be 0-1 BFS, but heap is simple and safe).
        let mut pq: BinaryHeap<(Reverse<u32>, usize, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), sx, sy));

        let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

        while let Some((Reverse(d), x, y)) = pq.pop() {
            if d != dist[y][x] {
                continue;
            }
            for (dx, dy) in dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    continue;
                }
                let nxu = nx as usize;
                let nyu = ny as usize;

                let cost = if grid[nyu][nxu] == grid[y][x] { 0u32 } else { 1u32 };
                let nd = d.saturating_add(cost);
                if nd < dist[nyu][nxu] {
                    dist[nyu][nxu] = nd;
                    pq.push((Reverse(nd), nxu, nyu));
                }
            }
        }

        // Minimum distance to any edge cell.
        let mut ans = inf;
        for x in 0..w {
            ans = ans.min(dist[0][x]);
            ans = ans.min(dist[h - 1][x]);
        }
        for y in 0..h {
            ans = ans.min(dist[y][0]);
            ans = ans.min(dist[y][w - 1]);
        }

        // Postcondition: output is a non-negative integer.
        // If unreachable (should not happen in an open grid), we still print the computed value.
        println!("{}", ans);
    }
}
