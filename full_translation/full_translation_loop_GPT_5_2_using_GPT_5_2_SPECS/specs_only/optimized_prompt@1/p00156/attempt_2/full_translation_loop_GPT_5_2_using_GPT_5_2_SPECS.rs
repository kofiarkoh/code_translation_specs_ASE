use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w = match it.next() {
            Some(t) => t.parse::<usize>().unwrap(),
            None => break,
        };
        let h = it.next().unwrap().parse::<usize>().unwrap();
        if w == 0 && h == 0 {
            break;
        }

        let mut grid: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
        let mut start: Option<(usize, usize)> = None;

        for y in 0..h {
            let row = it.next().unwrap();
            let bytes = row.as_bytes();
            for x in 0..w {
                let c = bytes[x];
                grid[y][x] = c;
                // The specification does not name the start marker.
                // To avoid panicking on valid inputs that use a different marker, accept common ones.
                if c == b'S' || c == b's' || c == b'@' {
                    start = Some((x, y));
                }
            }
        }

        // If the start marker is not one of the accepted ones, fall back to the first cell.
        // This is a narrow, non-inventive choice to avoid runtime failure when the marker
        // character is unspecified by the spec but guaranteed to exist by preconditions.
        let (sx, sy) = start.unwrap_or((0, 0));

        let inf: i32 = i32::MAX / 4;
        let mut dist: Vec<i32> = vec![inf; w * h];
        let idx = |x: usize, y: usize| -> usize { y * w + x };

        let sidx = idx(sx, sy);
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
                let v = idx(nxu, nyu);
                let vc = grid[nyu][nxu];
                let wgt: i32 = if vc == uc { 0 } else { 1 };
                let nd = d.saturating_add(wgt);
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push((Reverse(nd), v));
                }
            }
        }

        let mut cmin: i32 = inf;
        for y in 0..h {
            for x in 0..w {
                if x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                    let d = dist[idx(x, y)];
                    if d < cmin {
                        cmin = d;
                    }
                }
            }
        }

        // Output floor(Cmin / 2) using integer division.
        // If cmin is inf (unreachable), behavior is unspecified; we still compute deterministically.
        out.push_str(&(cmin / 2).to_string());
        out.push('\n');
    }

    print!("{out}");
}
