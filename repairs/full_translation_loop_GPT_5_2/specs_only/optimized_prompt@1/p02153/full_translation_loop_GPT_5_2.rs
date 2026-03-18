use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();
    let cost_normal: i64 = it.next().unwrap().parse().unwrap();
    let cost_bomb: i64 = it.next().unwrap().parse().unwrap();

    // Character mapping inferred from sample:
    // 's' = start, 'g' = goal, '*' = bomb, '#' = obstacle, '.' = normal (and any other non-# is traversable).
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;

    for r in 0..h {
        let row_str = it.next().unwrap_or("");
        let mut row = vec![b' '; w];
        let bytes = row_str.as_bytes();
        for c in 0..w {
            let ch = *bytes.get(c).unwrap_or(&b' ');
            row[c] = ch;
            if ch == b's' {
                start = Some((r, c));
            } else if ch == b'g' {
                goal = Some((r, c));
            }
        }
        grid.push(row);
    }

    // If start/goal are missing, behavior is undefined by spec; print unreachable to avoid panic.
    let (sr, sc) = match start {
        Some(v) => v,
        None => {
            println!("unreachable");
            return;
        }
    };
    let (gr, gc) = match goal {
        Some(v) => v,
        None => {
            println!("unreachable");
            return;
        }
    };

    let dirs = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    // Exceptional behavior per spec:
    // "If the starting position is surrounded by bombs, the program will return that the target is unreachable."
    // Interpret bombs as '*' and "surrounded" as all in-bounds 4-neighbors are bombs, with at least one neighbor.
    let mut any_neighbor = false;
    let mut all_neighbors_bombs = true;
    for (dr, dc) in dirs {
        let nr = sr as isize + dr;
        let nc = sc as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            any_neighbor = true;
            if grid[nr as usize][nc as usize] != b'*' {
                all_neighbors_bombs = false;
            }
        }
    }
    if any_neighbor && all_neighbors_bombs {
        println!("unreachable");
        return;
    }

    // Dijkstra: cost to move into a cell depends on the destination cell type.
    // Entering '*' costs cost_bomb, entering any other traversable cell costs cost_normal.
    // '#' is impassable.
    let n = h * w;
    let mut dist = vec![i64::MAX; n];
    let idx = |r: usize, c: usize| -> usize { r * w + c };

    let mut pq: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
    dist[idx(sr, sc)] = 0;
    pq.push((Reverse(0), sr, sc));

    while let Some((Reverse(d), r, c)) = pq.pop() {
        if d != dist[idx(r, c)] {
            continue;
        }
        if (r, c) == (gr, gc) {
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
            if cell == b'#' {
                continue;
            }

            let step = if cell == b'*' { cost_bomb } else { cost_normal };

            // Spec doesn't define overflow; avoid panic and keep behavior deterministic.
            let nd = match d.checked_add(step) {
                Some(v) => v,
                None => {
                    if step >= 0 {
                        i64::MAX
                    } else {
                        i64::MIN
                    }
                }
            };

            let id = idx(nr, nc);
            if nd < dist[id] {
                dist[id] = nd;
                pq.push((Reverse(nd), nr, nc));
            }
        }
    }

    println!("unreachable");
}
