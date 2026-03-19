use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    cost: u64,
    x: usize,
    y: usize,
}

fn dijkstra_min_cost(grid: &[Vec<u64>], sx: usize, sy: usize) -> u64 {
    let h = grid.len();
    let w = if h == 0 { 0 } else { grid[0].len() };
    // Preconditions say positive dimensions and valid start; keep narrow behavior.
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
            // Authentication levels are non-negative; use saturating add to avoid panic on overflow.
            let nd = d.saturating_add(grid[ny][nx]);
            if nd < dist[ny][nx] {
                dist[ny][nx] = nd;
                heap.push((Reverse(nd), nx, ny));
            }
        }
    }

    // "minimum authentication level achievable by traversing" is ambiguous about destination.
    // Narrowest reasonable interpretation: minimum cost to reach any cell from the start.
    // Since costs are non-negative and include the start cell, the minimum is always the start cell.
    // However, to preserve the "considering valid paths" intent, we compute the minimum over all reachable cells.
    dist.into_iter()
        .flat_map(|row| row.into_iter())
        .min()
        .unwrap_or(u64::MAX)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = Vec::with_capacity(t);

    for _ in 0..t {
        // Room 1
        let w1: usize = it.next().unwrap().parse().unwrap();
        let h1: usize = it.next().unwrap().parse().unwrap();
        let sx1: usize = it.next().unwrap().parse().unwrap();
        let sy1: usize = it.next().unwrap().parse().unwrap();
        let mut g1 = vec![vec![0u64; w1]; h1];
        for y in 0..h1 {
            for x in 0..w1 {
                g1[y][x] = it.next().unwrap().parse::<u64>().unwrap();
            }
        }

        // Room 2
        let w2: usize = it.next().unwrap().parse().unwrap();
        let h2: usize = it.next().unwrap().parse().unwrap();
        let sx2: usize = it.next().unwrap().parse().unwrap();
        let sy2: usize = it.next().unwrap().parse().unwrap();
        let mut g2 = vec![vec![0u64; w2]; h2];
        for y in 0..h2 {
            for x in 0..w2 {
                g2[y][x] = it.next().unwrap().parse::<u64>().unwrap();
            }
        }

        let min1 = dijkstra_min_cost(&g1, sx1, sy1);
        let min2 = dijkstra_min_cost(&g2, sx2, sy2);
        let combined = min1.saturating_add(min2);

        outputs.push(combined.to_string());
    }

    print!("{}", outputs.join("\n"));
}
