use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

const MAX_HW: usize = 103;
const MAX_LIM: usize = 10;
const INF: i32 = 1_000_000_000;

#[derive(Clone, Copy)]
enum TurnCost {
    NoCost,
    UseR1,
    UseR2,
    Disallowed,
}

fn dir_idx(dr: i32, dc: i32) -> Option<usize> {
    match (dr, dc) {
        (-1, 0) => Some(0), // up
        (0, 1) => Some(1),  // right
        (1, 0) => Some(2),  // down
        (0, -1) => Some(3), // left
        _ => None,
    }
}

// Fixed turn-allowance rules are not specified in detail.
// Narrowest reasonable interpretation consistent with the spec:
// - All direction changes are permitted.
// - Resource consumption depends only on the kind of turn:
//   * straight: no cost
//   * left turn: consumes resource1
//   * right turn: consumes resource2
//   * U-turn: disallowed
fn turn_cost(cur_dir: usize, next_dir: usize) -> TurnCost {
    if cur_dir == next_dir {
        return TurnCost::NoCost;
    }
    let diff = (next_dir + 4 - cur_dir) % 4;
    match diff {
        1 => TurnCost::UseR2,      // right
        3 => TurnCost::UseR1,      // left
        2 => TurnCost::Disallowed, // U-turn
        _ => TurnCost::Disallowed,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();
    let lim: usize = it.next().unwrap().parse().unwrap();

    // The spec states fixed internal maxima; behavior beyond is not inferable.
    // We still allocate based on input but keep indexing safe.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    let mut start = None;
    let mut goal = None;

    for r in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        for c in 0..w {
            let ch = row[c];
            if ch == b'S' {
                start = Some((r, c));
            } else if ch == b'G' {
                goal = Some((r, c));
            }
        }
        grid.push(row);
    }

    let (sr, sc) = start.unwrap();
    let (gr, gc) = goal.unwrap();

    // Blocked marker is not specified; narrowest reasonable interpretation:
    // treat '#' as blocked, everything else traversable (including S and G).
    let is_blocked = |r: usize, c: usize| -> bool { grid[r][c] == b'#' };

    // Initial move: one cell downward from start, facing down.
    let init_r = sr as i32 + 1;
    let init_c = sc as i32;

    if init_r < 0 || init_r >= h as i32 || init_c < 0 || init_c >= w as i32 {
        print!("-1\n");
        return;
    }
    let ir = init_r as usize;
    let ic = init_c as usize;
    if is_blocked(ir, ic) {
        print!("-1\n");
        return;
    }

    // dist[r][c][dir][r1][r2] = min total usage (r1+r2) to reach that state.
    // We store as i16/i32; limits are small.
    let limp1 = lim + 1;
    let mut dist = vec![INF; h * w * 4 * limp1 * limp1];
    let idx = |r: usize, c: usize, dir: usize, r1: usize, r2: usize| -> usize {
        ((((r * w + c) * 4 + dir) * limp1 + r1) * limp1 + r2)
    };

    let init_dir = 2usize; // down
    dist[idx(ir, ic, init_dir, 0, 0)] = 0;

    // Dijkstra over (r1+r2) since each move adds 0 or 1 to exactly one resource.
    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize, usize, usize, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), ir, ic, init_dir, 0, 0));

    let drs = [-1i32, 0, 1, 0];
    let dcs = [0i32, 1, 0, -1];

    let mut best_goal = INF;

    while let Some((Reverse(cost), r, c, dir, r1, r2)) = pq.pop() {
        if cost != dist[idx(r, c, dir, r1, r2)] {
            continue;
        }
        if cost > 20 {
            // Spec only cares about answers <= 20; pruning is safe for output semantics.
            continue;
        }
        if (r, c) == (gr, gc) {
            if cost < best_goal {
                best_goal = cost;
            }
            // Continue to ensure we find the minimum (though Dijkstra guarantees first is min).
            continue;
        }

        for ndir in 0..4 {
            let nr_i = r as i32 + drs[ndir];
            let nc_i = c as i32 + dcs[ndir];
            if nr_i < 0 || nr_i >= h as i32 || nc_i < 0 || nc_i >= w as i32 {
                continue;
            }
            let nr = nr_i as usize;
            let nc = nc_i as usize;
            if is_blocked(nr, nc) {
                continue;
            }

            let tc = turn_cost(dir, ndir);
            let (nr1, nr2, add) = match tc {
                TurnCost::NoCost => (r1, r2, 0i32),
                TurnCost::UseR1 => {
                    if r1 >= lim {
                        continue;
                    }
                    (r1 + 1, r2, 1i32)
                }
                TurnCost::UseR2 => {
                    if r2 >= lim {
                        continue;
                    }
                    (r1, r2 + 1, 1i32)
                }
                TurnCost::Disallowed => continue,
            };

            // "Do not consume resources on moves originating from the start cell
            // (resource-consuming moves from the start are disallowed)."
            // Since the initial state is already one cell below start, we only need to
            // enforce this if we ever stand on the start cell.
            if (r, c) == (sr, sc) && add != 0 {
                continue;
            }

            let ncost = cost + add;
            if ncost > 20 {
                continue;
            }
            let di = idx(nr, nc, ndir, nr1, nr2);
            if ncost < dist[di] {
                dist[di] = ncost;
                pq.push((Reverse(ncost), nr, nc, ndir, nr1, nr2));
            }
        }
    }

    if best_goal <= 20 {
        print!("{}\n", best_goal);
    } else {
        print!("-1\n");
    }

    // Note: The spec mentions fixed-size internal storage and queue overflow behavior
    // being not inferable; this implementation uses dynamic storage and thus does not
    // emulate such overflow behavior.
    let _ = (MAX_HW, MAX_LIM); // keep constants referenced
}
