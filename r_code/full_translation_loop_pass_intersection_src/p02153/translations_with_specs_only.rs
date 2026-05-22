use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const MAX: usize = 1005; // Fixed internal maximum grid size (must be >= H,W with margin for hazard expansion)

fn main() {
    // Read all input as whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();
    let a_in: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Internal storage is 1-based indexed for the grid area [1..=H][1..=W].
    // The specification states hazard expansion does not do explicit boundary checks;
    // we emulate that by using a fixed MAXxMAX storage and writing neighbors directly.
    // Valid inputs must ensure these writes stay within [0..MAX).
    let mut grid = vec![vec![b'.'; MAX]; MAX];
    let mut dangerous = vec![vec![false; MAX]; MAX];

    let mut sr: usize = 0;
    let mut sc: usize = 0;
    let mut gr: usize = 0;
    let mut gc: usize = 0;

    for i in 1..=h {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes();
        for j in 1..=w {
            let ch = if j - 1 < bytes.len() { bytes[j - 1] } else { b'.' };
            grid[i][j] = ch;
            if ch == b's' {
                sr = i;
                sc = j;
            } else if ch == b'g' {
                gr = i;
                gc = j;
            }
        }
    }

    // Derived hazard (dangerous) marking: a cell is dangerous if it is '*'
    // or any of its 8 neighbors. No explicit boundary checks are performed here.
    for i in 1..=h {
        for j in 1..=w {
            if grid[i][j] == b'*' {
                for di in -1i32..=1 {
                    for dj in -1i32..=1 {
                        let ni = (i as i32 + di) as usize;
                        let nj = (j as i32 + dj) as usize;
                        dangerous[ni][nj] = true;
                    }
                }
            }
        }
    }

    // Dijkstra-style search with finalization on pop.
    let inf: i64 = i64::MAX / 4;
    let mut dist = vec![vec![inf; MAX]; MAX];
    let mut finalized = vec![vec![false; MAX]; MAX];
    let mut pq: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();

    dist[sr][sc] = 0;
    pq.push((Reverse(0), sr, sc));

    // Movement deltas.
    const ORTHO: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const DIAG: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    while let Some((Reverse(d), r, c)) = pq.pop() {
        if finalized[r][c] {
            continue;
        }
        finalized[r][c] = true;

        if r == gr && c == gc {
            println!("{}", d);
            return;
        }

        let cur_danger = dangerous[r][c];

        // Determine candidate moves based on whether current cell is dangerous.
        // If not dangerous: consider 8 directions, but diagonal moves are only allowed
        // when destination is a wall.
        // If dangerous: consider only 4 orthogonal moves, and additionally forbid
        // moves into wall destinations.
        let mut consider = Vec::with_capacity(8);
        consider.extend_from_slice(&ORTHO);
        if !cur_danger {
            consider.extend_from_slice(&DIAG);
        }

        for (dr, dc) in consider {
            let nr_i32 = r as i32 + dr;
            let nc_i32 = c as i32 + dc;
            if nr_i32 < 1 || nr_i32 > h as i32 || nc_i32 < 1 || nc_i32 > w as i32 {
                continue;
            }
            let nr = nr_i32 as usize;
            let nc = nc_i32 as usize;

            let is_wall = grid[nr][nc] == b'#';
            let is_diag = dr != 0 && dc != 0;

            // Additional constraints:
            // - If current is dangerous and destination is wall: not allowed.
            if cur_danger && is_wall {
                continue;
            }
            // - Diagonal moves only allowed when destination is a wall.
            if is_diag && !is_wall {
                continue;
            }

            // Step cost.
            let step = if is_diag {
                // Diagonal
                if is_wall {
                    2 * a_in + b
                } else {
                    2 * a_in
                }
            } else {
                // Orthogonal
                if is_wall {
                    a_in + b
                } else {
                    a_in
                }
            };

            // Accumulated cost in signed 64-bit; specification assumes it fits.
            let nd = d + step;
            if nd < dist[nr][nc] {
                dist[nr][nc] = nd;
                pq.push((Reverse(nd), nr, nc));
            }
        }
    }

    println!("INF");
}
