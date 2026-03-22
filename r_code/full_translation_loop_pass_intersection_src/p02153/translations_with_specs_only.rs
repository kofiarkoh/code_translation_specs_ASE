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
    let base_cost: u64 = it.next().unwrap().parse().unwrap();
    let penalty_cost: u64 = it.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;

    for i in 0..h {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes().to_vec();
        grid.push(bytes);
        for j in 0..w {
            let c = grid[i][j] as char;
            // The actual markers in the dataset appear to be lowercase: 's' and 'g'.
            if c == 's' {
                start = Some((i, j));
            } else if c == 'g' {
                goal = Some((i, j));
            }
        }
    }

    let start = start.unwrap_or((0, 0));
    let goal = goal.unwrap_or((0, 0));

    // Hazard source marker appears to be '*'.
    let mut hazard = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] as char == '*' {
                // Spec precondition: hazard sources are not on the outermost border.
                // Keep bounds checks for safety; for valid inputs this matches the intended marking.
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

    let is_wall = |c: u8| -> bool { c as char == '#' };

    let inf: u64 = u64::MAX / 4;
    let mut dist = vec![vec![inf; w]; h];
    let mut visited = vec![vec![false; w]; h];
    let mut pq: BinaryHeap<(Reverse<u64>, usize, usize)> = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    pq.push((Reverse(0), start.0, start.1));

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

            // Validity constraint: cannot move from a hazardous cell into a wall cell.
            if hazard[i][j] && is_wall(grid[niu][nju]) {
                continue;
            }

            // Cost: orthogonal = base_cost, diagonal = 2*base_cost; entering a wall adds penalty_cost.
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

    // Unreachability indicator: use "INF" as a conventional infinity marker.
    println!("INF");
}
