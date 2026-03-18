use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    // Parse height and width; on invalid input, return error code.
    let h: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let w: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Enforce maximum grid size constraint (narrow interpretation: treat as invalid input).
    if h > 512 || w > 512 {
        process::exit(1);
    }

    // Read grid rows; each row must have exactly w characters.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = match it.next() {
            Some(s) => s.as_bytes().to_vec(),
            None => process::exit(1),
        };
        if row.len() != w {
            process::exit(1);
        }
        grid.push(row);
    }

    // Grouping: connected components under 4-neighborhood where adjacent cells must be different
    // to be connected (i.e., edges exist only between cells with different characters).
    //
    // For each component, count occurrences of each character; then count pairs of cells with
    // different characters within the same component:
    // sum over components of (total_pairs - same_char_pairs) = sum over components of
    // (n*(n-1)/2 - sum_c cnt[c]*(cnt[c]-1)/2).
    //
    // This matches "pairs of cells from different groups" under the implied bipartite-style
    // grouping used by DFS on alternating characters.
    let mut visited = vec![vec![false; w]; h];
    let mut ans: u128 = 0;

    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }

            // BFS/DFS to collect one component.
            let mut q = VecDeque::new();
            visited[i][j] = true;
            q.push_back((i, j));

            // Count per byte value (0..=255) to avoid assuming character set.
            let mut cnt = [0u32; 256];
            let mut n: u64 = 0;

            while let Some((x, y)) = q.pop_front() {
                let ch = grid[x][y];
                cnt[ch as usize] += 1;
                n += 1;

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
                    // Edge exists only if characters differ.
                    if grid[nxu][nyu] != ch {
                        visited[nxu][nyu] = true;
                        q.push_back((nxu, nyu));
                    }
                }
            }

            let total_pairs: u128 = (n as u128) * ((n - 1) as u128) / 2;
            let mut same_pairs: u128 = 0;
            for &c in &cnt {
                let cc = c as u128;
                same_pairs += cc * (cc - 1) / 2;
            }
            ans += total_pairs - same_pairs;
        }
    }

    // Output a single integer.
    println!("{}", ans);
}
