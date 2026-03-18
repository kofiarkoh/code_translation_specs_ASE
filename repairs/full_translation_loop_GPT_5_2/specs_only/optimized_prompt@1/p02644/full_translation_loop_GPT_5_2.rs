use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Per spec: invalid inputs => undefined behavior, so we unwrap directly.
    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    // Coordinates are assumed to be within bounds; indexing base is not specified.
    // Narrowest reasonable interpretation for grid problems: 1-based input coordinates.
    let sx_in: isize = it.next().unwrap().parse().unwrap();
    let sy_in: isize = it.next().unwrap().parse().unwrap();
    let tx_in: isize = it.next().unwrap().parse().unwrap();
    let ty_in: isize = it.next().unwrap().parse().unwrap();

    let sx = (sx_in - 1) as usize;
    let sy = (sy_in - 1) as usize;
    let tx = (tx_in - 1) as usize;
    let ty = (ty_in - 1) as usize;

    // Read grid: each row is a token (no spaces). Valid characters are not specified;
    // we only treat '#' as obstacle and everything else as passable.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        grid.push(row);
    }

    // Distance array: -1 means unvisited/unreachable.
    let mut dist = vec![vec![-1i32; w]; h];
    let mut q = VecDeque::<(usize, usize)>::new();

    dist[sx][sy] = 0;
    q.push_back((sx, sy));

    // BFS with constrained "ray" moves up to k in 4 directions, stopping at obstacles.
    // Invariant: dist holds the minimum moves discovered so far; queue processes in discovery order.
    let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((x, y)) = q.pop_front() {
        let dcur = dist[x][y];

        for (dx, dy) in dirs {
            for step in 1..=k {
                let nx_i = x as isize + dx * step as isize;
                let ny_i = y as isize + dy * step as isize;

                if nx_i < 0 || ny_i < 0 || nx_i >= h as isize || ny_i >= w as isize {
                    break;
                }
                let nx = nx_i as usize;
                let ny = ny_i as usize;

                if grid[nx][ny] == b'#' {
                    break;
                }

                if dist[nx][ny] == -1 {
                    dist[nx][ny] = dcur + 1;
                    q.push_back((nx, ny));
                } else {
                    // If we already reached (nx, ny) in <= dcur+1 moves, then continuing further
                    // in this direction from (x, y) cannot improve anything beyond this point
                    // under BFS layering, so we stop scanning this ray.
                    if dist[nx][ny] <= dcur + 1 {
                        break;
                    } else {
                        // This case is unlikely under standard BFS, but to preserve the invariant
                        // "dist maintains minimum moves", we allow improvement and continue.
                        dist[nx][ny] = dcur + 1;
                        q.push_back((nx, ny));
                    }
                }
            }
        }
    }

    println!("{}", dist[tx][ty]);
}
