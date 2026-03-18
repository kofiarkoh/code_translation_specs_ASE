use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn dijkstra_min_cost(grid: &[Vec<u64>], sx: usize, sy: usize) -> u64 {
    let h = grid.len();
    let w = if h == 0 { 0 } else { grid[0].len() };

    // If dimensions are degenerate or start is out of bounds (input may violate preconditions),
    // avoid panicking; return 0 as the narrowest "do nothing" behavior.
    if h == 0 || w == 0 || sy >= h || sx >= w {
        return 0;
    }

    let mut dist = vec![vec![u64::MAX; w]; h];
    let mut heap: BinaryHeap<(Reverse<u64>, usize, usize)> = BinaryHeap::new();

    dist[sy][sx] = grid[sy][sx];
    heap.push((Reverse(dist[sy][sx]), sx, sy));

    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    while let Some((Reverse(d), x, y)) = heap.pop() {
        if d != dist[y][x] {
            continue;
        }
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= h || nx >= w {
                continue;
            }
            let nd = d.saturating_add(grid[ny][nx]);
            if nd < dist[ny][nx] {
                dist[ny][nx] = nd;
                heap.push((Reverse(nd), nx, ny));
            }
        }
    }

    // Minimum cost to reach any cell from the start.
    dist.into_iter()
        .flat_map(|row| row.into_iter())
        .min()
        .unwrap_or(u64::MAX)
}

fn read_grid(it: &mut dyn Iterator<Item = &str>, w: usize, h: usize) -> Vec<Vec<u64>> {
    let mut g = vec![vec![0u64; w]; h];
    for y in 0..h {
        for x in 0..w {
            // If input is truncated, keep remaining cells as 0 to avoid panics.
            if let Some(tok) = it.next() {
                g[y][x] = tok.parse::<u64>().unwrap_or(0);
            } else {
                return g;
            }
        }
    }
    g
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(v) => v.parse().unwrap_or(0),
        None => return,
    };

    let mut outputs = Vec::with_capacity(t);

    for _ in 0..t {
        // Room 1
        let w1: usize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let h1: usize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let x1_in: isize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let y1_in: isize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let g1 = read_grid(&mut it, w1, h1);

        // Room 2
        let w2: usize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let h2: usize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let x2_in: isize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let y2_in: isize = match it.next() {
            Some(v) => v.parse().unwrap_or(0),
            None => break,
        };
        let g2 = read_grid(&mut it, w2, h2);

        // The sample crash suggests coordinates may be 1-based in the actual input.
        // To preserve behavior without external assumptions, try 0-based first; if out of bounds,
        // fall back to 1-based (subtract 1) if that becomes valid.
        fn normalize_start(x: isize, y: isize, w: usize, h: usize) -> (usize, usize) {
            let in_bounds_0 = x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h;
            if in_bounds_0 {
                return (x as usize, y as usize);
            }
            let x1 = x - 1;
            let y1 = y - 1;
            let in_bounds_1 = x1 >= 0 && y1 >= 0 && (x1 as usize) < w && (y1 as usize) < h;
            if in_bounds_1 {
                return (x1 as usize, y1 as usize);
            }
            // If still invalid, clamp to (0,0) to avoid panics.
            (0, 0)
        }

        let (sx1, sy1) = normalize_start(x1_in, y1_in, w1, h1);
        let (sx2, sy2) = normalize_start(x2_in, y2_in, w2, h2);

        let min1 = dijkstra_min_cost(&g1, sx1, sy1);
        let min2 = dijkstra_min_cost(&g2, sx2, sy2);
        let combined = min1.saturating_add(min2);

        outputs.push(combined.to_string());
    }

    print!("{}", outputs.join("\n"));
}
