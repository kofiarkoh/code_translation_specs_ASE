use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse first four integers, then read the grid as raw lines to avoid
    // assumptions about whitespace formatting inside rows.
    let mut pos = 0usize;
    let bytes = input.as_bytes();

    fn skip_ws(bytes: &[u8], pos: &mut usize) {
        while *pos < bytes.len() && bytes[*pos].is_ascii_whitespace() {
            *pos += 1;
        }
    }
    fn read_i64(bytes: &[u8], pos: &mut usize) -> Option<i64> {
        skip_ws(bytes, pos);
        if *pos >= bytes.len() {
            return None;
        }
        let mut sign = 1i64;
        if bytes[*pos] == b'-' {
            sign = -1;
            *pos += 1;
        }
        let mut val: i64 = 0;
        let mut any = false;
        while *pos < bytes.len() {
            let c = bytes[*pos];
            if !c.is_ascii_digit() {
                break;
            }
            any = true;
            val = val.saturating_mul(10).saturating_add((c - b'0') as i64);
            *pos += 1;
        }
        if any { Some(val * sign) } else { None }
    }

    let h = match read_i64(bytes, &mut pos) {
        Some(v) if v > 0 => v as usize,
        _ => return,
    };
    let w = match read_i64(bytes, &mut pos) {
        Some(v) if v > 0 => v as usize,
        _ => return,
    };
    let cost_normal = match read_i64(bytes, &mut pos) {
        Some(v) => v,
        None => return,
    };
    let cost_bomb = match read_i64(bytes, &mut pos) {
        Some(v) => v,
        None => return,
    };

    // Read exactly h rows, each with at least w characters (excluding line breaks).
    // If a row is shorter, pad with spaces (undefined behavior per spec; this avoids panics).
    // Character mapping (narrow, common):
    // 'S' start, 'G' goal, 'B' bomb, '#' obstacle, others traversable normal.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;

    for r in 0..h {
        // Move to start of next non-newline content (but allow leading spaces as valid cells).
        // We only skip '\r' and '\n' here to preserve spaces as grid cells if present.
        while pos < bytes.len() && (bytes[pos] == b'\n' || bytes[pos] == b'\r') {
            pos += 1;
        }

        let mut row: Vec<u8> = Vec::with_capacity(w);
        while row.len() < w && pos < bytes.len() {
            let c = bytes[pos];
            if c == b'\n' || c == b'\r' {
                break;
            }
            row.push(c);
            pos += 1;
        }

        // Consume rest of line (if any) to align to next row.
        while pos < bytes.len() && bytes[pos] != b'\n' {
            if bytes[pos] == b'\r' {
                pos += 1;
                break;
            }
            pos += 1;
        }
        if pos < bytes.len() && bytes[pos] == b'\n' {
            pos += 1;
        }

        if row.len() < w {
            row.resize(w, b' ');
        }

        for c in 0..w {
            match row[c] {
                b'S' => start = Some((r, c)),
                b'G' => goal = Some((r, c)),
                _ => {}
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

    // Exceptional behavior per spec.
    // Interpret "surrounded by bombs" as: all in-bounds 4-neighbors are bombs, and there is at least one neighbor.
    let mut any_neighbor = false;
    let mut all_neighbors_bombs = true;
    for (dr, dc) in dirs {
        let nr = sr as isize + dr;
        let nc = sc as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            any_neighbor = true;
            if grid[nr as usize][nc as usize] != b'B' {
                all_neighbors_bombs = false;
            }
        }
    }
    if any_neighbor && all_neighbors_bombs {
        println!("unreachable");
        return;
    }

    // Dijkstra: cost to move into a cell depends on the destination cell type.
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

            let step = if cell == b'B' { cost_bomb } else { cost_normal };

            // Avoid panic on overflow; spec doesn't define overflow behavior.
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
