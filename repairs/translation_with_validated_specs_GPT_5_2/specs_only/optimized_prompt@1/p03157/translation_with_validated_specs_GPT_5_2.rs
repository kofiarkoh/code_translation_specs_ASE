use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as bytes, then parse as whitespace-delimited tokens (C "word" style).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let h: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let w: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Read H rows; each must be a non-whitespace string of length at least W.
    // Characters beyond W-1 are ignored.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = match it.next() {
            Some(s) => s.as_bytes(),
            None => std::process::exit(1),
        };
        // Per spec, if row length < W, behavior is undefined; we do not add extra handling.
        let mut r = Vec::with_capacity(w);
        r.extend_from_slice(&row[..w]);
        grid.push(r);
    }

    let mut visited = vec![vec![false; w]; h];
    let mut ans: u64 = 0;

    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }

            // BFS/DFS over region defined by edges between different characters.
            let mut q = VecDeque::new();
            visited[i][j] = true;
            q.push_back((i, j));

            let mut dots: u64 = 0;
            let mut nondots: u64 = 0;

            while let Some((x, y)) = q.pop_front() {
                if grid[x][y] == b'.' {
                    dots = dots.wrapping_add(1);
                } else {
                    nondots = nondots.wrapping_add(1);
                }

                let cur = grid[x][y];
                for (dx, dy) in dirs {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || ny < 0 || nx >= h as isize || ny >= w as isize {
                        continue;
                    }
                    let nxu = nx as usize;
                    let nyu = ny as usize;

                    if visited[nxu][nyu] {
                        continue;
                    }
                    // Connectivity step allowed only if characters are different.
                    if grid[nxu][nyu] != cur {
                        visited[nxu][nyu] = true;
                        q.push_back((nxu, nyu));
                    }
                }
            }

            ans = ans.wrapping_add(dots.wrapping_mul(nondots));
        }
    }

    // Print as unsigned 64-bit decimal with newline (Rust's formatting is portable for u64).
    println!("{}", ans);
}
