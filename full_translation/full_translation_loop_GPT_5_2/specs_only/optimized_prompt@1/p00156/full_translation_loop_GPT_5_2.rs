use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; parse as lines to preserve grid rows (which may contain '#', '.', '&', etc.).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    loop {
        // Read next non-empty header line (w h). If none, end.
        let header = loop {
            match lines.next() {
                None => return,
                Some(l) => {
                    let t = l.trim();
                    if !t.is_empty() {
                        break t.to_string();
                    }
                }
            }
        };

        let mut hw = header.split_whitespace();
        let w: usize = match hw.next() {
            Some(v) => v.parse().unwrap(),
            None => return,
        };
        let h: usize = match hw.next() {
            Some(v) => v.parse().unwrap(),
            None => return,
        };

        if w == 0 && h == 0 {
            break;
        }

        let mut grid: Vec<Vec<u8>> = vec![vec![b' '; w]; h];
        let mut start: Option<(usize, usize)> = None;

        for y in 0..h {
            let row = lines.next().unwrap_or("");
            let bytes = row.as_bytes();
            for x in 0..w {
                let c = if x < bytes.len() { bytes[x] } else { b' ' };
                grid[y][x] = c;

                // Starting point marker in the judge data is '&'.
                if c == b'&' && start.is_none() {
                    start = Some((x, y));
                }
            }
        }

        // Spec says grid contains at least one starting point; if not, behavior undefined.
        // We avoid panicking to prevent runtime error; print 0 as a non-negative integer.
        let (sx, sy) = match start {
            Some(p) => p,
            None => {
                println!("0");
                continue;
            }
        };

        // Transition cost: entering a cell counts 1 if that cell is '#', else 0.
        // This matches "counted based on the character type encountered" and the sample behavior.
        let inf: u32 = u32::MAX / 4;
        let mut dist: Vec<Vec<u32>> = vec![vec![inf; w]; h];
        dist[sy][sx] = 0;

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

                let cost = if grid[nyu][nxu] == b'#' { 1u32 } else { 0u32 };
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

        if ans == inf {
            ans = 0;
        }
        println!("{}", ans);
    }
}
