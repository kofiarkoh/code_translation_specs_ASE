use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Input format (inferred from failing sample):
    // H W K
    // H lines of grid ('.', '#', 'S', 'G')
    //
    // "maximum number of allowed moves in two distinct directions":
    // With only one integer K provided, interpret as a single limit on a special move type.
    // Narrowest consistent model: normal 4-neighbor moves are free/unlimited, and there is a
    // second move type "break through a wall" usable at most K times (common BFS-with-breaks).
    //
    // Output: minimum number of moves (edges) from S to G, else -1.
    //
    // Exceptional behavior from spec:
    // - If start is at bottom edge OR adjacent to obstacle => output failure (-1).
    //   Apply adjacency in 4-neighborhood.
    // - If computed minimum moves exceed a predefined threshold => output failure (-1).
    //   Threshold unspecified; use a conservative bound based on grid size.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(x) => match x.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("-1");
                return;
            }
        },
        None => return,
    };
    let w: usize = match it.next() {
        Some(x) => match x.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("-1");
                return;
            }
        },
        None => {
            println!("-1");
            return;
        }
    };
    let k: usize = match it.next() {
        Some(x) => match x.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("-1");
                return;
            }
        },
        None => {
            println!("-1");
            return;
        }
    };

    if h == 0 || w == 0 {
        println!("-1");
        return;
    }

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;

    for r in 0..h {
        let row_s = match it.next() {
            Some(s) => s,
            None => {
                println!("-1");
                return;
            }
        };
        let row = row_s.as_bytes().to_vec();
        if row.len() != w {
            println!("-1");
            return;
        }
        for c in 0..w {
            match row[c] {
                b'S' => {
                    if start.is_some() {
                        println!("-1");
                        return;
                    }
                    start = Some((r, c));
                }
                b'G' => {
                    if goal.is_some() {
                        println!("-1");
                        return;
                    }
                    goal = Some((r, c));
                }
                b'.' | b'#' => {}
                _ => {
                    println!("-1");
                    return;
                }
            }
        }
        grid.push(row);
    }

    let (sr, sc) = match start {
        Some(p) => p,
        None => {
            println!("-1");
            return;
        }
    };
    let (gr, gc) = match goal {
        Some(p) => p,
        None => {
            println!("-1");
            return;
        }
    };

    // Exceptional behavior: start at bottom edge => failure.
    if sr + 1 == h {
        println!("-1");
        return;
    }

    // Exceptional behavior: start adjacent to obstacle => failure (4-neighborhood).
    let dirs = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in dirs {
        let nr = sr as isize + dr;
        let nc = sc as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            if grid[nr as usize][nc as usize] == b'#' {
                println!("-1");
                return;
            }
        }
    }

    // BFS over (r, c, breaks_used). breaks_used <= k.
    // Move to '.'/'S'/'G' without increasing breaks_used.
    // Move to '#' by increasing breaks_used by 1 (if allowed).
    //
    // This matches "two distinct directions" as two distinct move types.
    let states = h.saturating_mul(w).saturating_mul(k.saturating_add(1));
    if states == 0 {
        println!("-1");
        return;
    }

    // Fixed memory limit: keep a conservative cap to avoid OOM.
    const MAX_STATES: usize = 10_000_000;
    if states > MAX_STATES {
        println!("-1");
        return;
    }

    let idx = |r: usize, c: usize, b: usize, w: usize, k: usize| -> usize {
        (r * w + c) * (k + 1) + b
    };

    let mut dist: Vec<u32> = vec![u32::MAX; states];
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();

    dist[idx(sr, sc, 0, w, k)] = 0;
    q.push_back((sr, sc, 0));

    // Threshold unspecified; choose a conservative bound based on grid size.
    // If the true answer is larger, spec says to output failure.
    let threshold: u32 = (h.saturating_mul(w).saturating_mul(4)) as u32;

    while let Some((r, c, b_used)) = q.pop_front() {
        let d = dist[idx(r, c, b_used, w, k)];
        if d == u32::MAX {
            continue;
        }
        if d > threshold {
            println!("-1");
            return;
        }
        if r == gr && c == gc {
            println!("{}", d);
            return;
        }

        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            let cell = grid[nr][nc];
            let (nb, passable) = if cell == b'#' {
                if b_used >= k {
                    (b_used, false)
                } else {
                    (b_used + 1, true)
                }
            } else {
                (b_used, true)
            };
            if !passable {
                continue;
            }

            let nd = d.saturating_add(1);
            let nindex = idx(nr, nc, nb, w, k);
            if nd < dist[nindex] {
                dist[nindex] = nd;
                q.push_back((nr, nc, nb));
            }
        }
    }

    println!("-1");
}
