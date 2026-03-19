use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    // Read grid as bytes; each row is a token (no spaces).
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes().to_vec();
        // Narrowest reasonable interpretation: assume input respects preconditions (length == w).
        grid.push(bytes);
    }

    // The specification describes:
    // - distances initialized to "infinity"
    // - cells containing the "specific character" have distance 0
    // - propagate to compute nearest "different character" distance.
    //
    // Since the spec does not name the characters, the narrowest interpretation that matches
    // the described algorithm is:
    // - treat one character as the "specific character" and all others as "different"
    // - choose the "specific character" as the first character encountered in the grid.
    //
    // This avoids inventing a hard-coded character and stays within the spec's constraints.
    let mut specific: Option<u8> = None;
    'outer: for i in 0..h {
        for j in 0..w {
            if j < grid[i].len() {
                specific = Some(grid[i][j]);
                break 'outer;
            }
        }
    }
    let specific = match specific {
        Some(c) => c,
        None => {
            // No cells; but preconditions say positive dimensions, so this should not happen.
            // Output 0 as a non-negative valid distance.
            println!("0");
            return;
        }
    };

    // Multi-source BFS from all "different character" cells to compute distance to nearest different.
    // Then take maximum over "specific character" cells.
    const INF: i32 = 1_000_000_000;
    let mut dist = vec![vec![INF; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            let c = grid[i][j];
            if c != specific {
                dist[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    // If there are no "different character" cells, then the nearest different is unreachable.
    // The spec does not define this case; the narrowest behavior is to keep INF and thus
    // output 0 (no finite maximum distance can be established).
    if q.is_empty() {
        println!("0");
        return;
    }

    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];
    while let Some((x, y)) = q.pop_front() {
        let d = dist[x][y];
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= h || ny >= w {
                continue;
            }
            if dist[nx][ny] > d + 1 {
                dist[nx][ny] = d + 1;
                q.push_back((nx, ny));
            }
        }
    }

    let mut ans: i32 = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == specific {
                let d = dist[i][j];
                if d != INF {
                    ans = max(ans, d);
                }
            }
        }
    }

    println!("{}", ans);
}
