use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification provides no malformed-input handling guarantees.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let sr1: usize = it.next().unwrap().parse().unwrap();
    let sc1: usize = it.next().unwrap().parse().unwrap();
    let gr1: usize = it.next().unwrap().parse().unwrap();
    let gc1: usize = it.next().unwrap().parse().unwrap();

    // Convert 1-based to 0-based.
    let sr = sr1 - 1;
    let sc = sc1 - 1;
    let gr = gr1 - 1;
    let gc = gc1 - 1;

    // Grid: '.' is passable; any other char is blocked.
    // Each row provides at least width characters (precondition).
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        grid.push(row);
    }

    // Distances: -1 means unreachable.
    let mut dist = vec![vec![-1i32; w]; h];

    // Start is reachable in 0 moves unconditionally, even if blocked.
    dist[sr][sc] = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sr, sc));

    let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((r, c)) = q.pop_front() {
        let d = dist[r][c];
        // Expand in 4 directions, up to K steps, stopping at first blocked/out-of-bounds.
        for (dr, dc) in dirs {
            for step in 1..=k {
                let nr = r as isize + dr * step as isize;
                let nc = c as isize + dc * step as isize;
                if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                    break;
                }
                let ur = nr as usize;
                let uc = nc as usize;

                // Cannot pass through blocked cells; stop at first blocked cell.
                if grid[ur][uc] != b'.' {
                    break;
                }

                // If already reached with a strictly smaller distance, further steps in this
                // direction from the current cell cannot improve (they would be >= that).
                if dist[ur][uc] != -1 && dist[ur][uc] < d + 1 {
                    break;
                }

                // If unvisited, set distance and enqueue.
                if dist[ur][uc] == -1 {
                    dist[ur][uc] = d + 1;
                    q.push_back((ur, uc));
                } else {
                    // dist[ur][uc] == d+1: already discovered at same minimal distance;
                    // continue scanning further cells in this direction.
                    dist[ur][uc] = min(dist[ur][uc], d + 1);
                }
            }
        }
    }

    println!("{}", dist[gr][gc]);
}
