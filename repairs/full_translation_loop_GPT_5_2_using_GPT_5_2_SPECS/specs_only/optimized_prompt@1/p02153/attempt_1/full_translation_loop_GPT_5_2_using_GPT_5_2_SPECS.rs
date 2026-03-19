use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    let base_cost: u64 = it.next().unwrap().parse().unwrap();
    let penalty_cost: u64 = it.next().unwrap().parse().unwrap();

    // Read grid rows (each row is a token; spec says no spaces inside rows)
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    for i in 0..h {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes().to_vec();
        // Spec assumes exactly width characters; no explicit error handling.
        grid.push(bytes);
        for j in 0..w {
            let c = grid[i][j] as char;
            // Marker characters are not specified; choose the narrowest common interpretation.
            if c == 'S' {
                start = (i, j);
            } else if c == 'G' {
                goal = (i, j);
            }
        }
    }

    // Hazard map: marked hazardous if cell is a hazard source or any of its 8 neighbors is.
    // Hazard source marker character is not specified; choose 'B' as the narrowest reasonable.
    let mut hazard = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] as char == 'B' {
                // Spec precondition: hazard sources are not on the outermost border.
                // We still bounds-check to keep Rust safe; this preserves behavior for valid inputs.
                for di in [-1isize, 0, 1] {
                    for dj in [-1isize, 0, 1] {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && nj >= 0 && (ni as usize) < h && (nj as usize) < w {
                            hazard[ni as usize][nj as usize] = true;
                        }
                    }
                }
            }
        }
    }

    // Wall marker character is not specified; choose '#'.
    let is_wall = |c: u8| -> bool { c as char == '#' };

    // Dijkstra-like shortest path with a visited set that finalizes each cell at most once.
    // This matches the spec invariant: once visited, not reconsidered for expansion.
    let inf: u64 = u64::MAX / 4;
    let mut dist = vec![vec![inf; w]; h];
    let mut visited = vec![vec![false; w]; h];
    let mut pq: BinaryHeap<(Reverse<u64>, usize, usize)> = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    pq.push((Reverse(0), start.0, start.1));

    // 8-direction movement
    let dirs: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    while let Some((Reverse(d), i, j)) = pq.pop() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        if (i, j) == goal {
            println!("{}", d);
            return;
        }

        for &(di, dj) in dirs {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || nj < 0 || (ni as usize) >= h || (nj as usize) >= w {
                continue;
            }
            let niu = ni as usize;
            let nju = nj as usize;

            // Path validity constraint:
            // "A path is considered valid only if it never performs a move from a hazardous cell into a wall cell."
            if hazard[i][j] && is_wall(grid[niu][nju]) {
                continue;
            }

            // Move cost depends on orthogonal vs diagonal and whether destination is a wall.
            // Spec: non-negative cost determined solely by those factors.
            let orth = di == 0 || dj == 0;
            let mut step = if orth { base_cost } else { base_cost.saturating_mul(2) };
            if is_wall(grid[niu][nju]) {
                step = step.saturating_add(penalty_cost);
            }

            let nd = d.saturating_add(step);
            if !visited[niu][nju] && nd < dist[niu][nju] {
                dist[niu][nju] = nd;
                pq.push((Reverse(nd), niu, nju));
            }
        }
    }

    // Infinity/unreachability indicator string is not specified; choose "INF".
    println!("INF");
}
