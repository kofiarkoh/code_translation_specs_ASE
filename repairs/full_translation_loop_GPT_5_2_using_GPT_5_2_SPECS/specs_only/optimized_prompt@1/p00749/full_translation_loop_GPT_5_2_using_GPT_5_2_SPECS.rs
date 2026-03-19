use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Comp {
    mass: i64,
    moment: i64, // sum of column indices of own cells
    contact_min: i32,
    contact_max: i32,
    children: Vec<usize>, // supporter -> supported
    agg_mass: i64,
    agg_moment: i64,
    stable: bool,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_tok = match it.next() {
            Some(t) => t,
            None => break,
        };
        let w: i32 = match w_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let h_tok = match it.next() {
            Some(t) => t,
            None => break,
        };
        let h: i32 = match h_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if w <= 0 {
            break;
        }
        if h <= 0 {
            break;
        }

        let width = w as usize;
        let height = h as usize;

        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(height);
        for _ in 0..height {
            grid.push(it.next().unwrap_or("").as_bytes().to_vec());
        }

        // Components: 4-neighborhood connectivity of any solid cell (non-dot).
        let mut comp_id = vec![vec![usize::MAX; width]; height];
        let mut comps: Vec<Comp> = Vec::new();
        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];

        for r in 0..height {
            for c in 0..width {
                let ch = *grid[r].get(c).unwrap_or(&b'.');
                if ch == b'.' || comp_id[r][c] != usize::MAX {
                    continue;
                }

                let id = comps.len();
                comps.push(Comp {
                    mass: 0,
                    moment: 0,
                    contact_min: i32::MAX,
                    contact_max: i32::MIN,
                    children: Vec::new(),
                    agg_mass: 0,
                    agg_moment: 0,
                    stable: true,
                });

                let mut q = VecDeque::new();
                q.push_back((r, c));
                comp_id[r][c] = id;

                while let Some((rr, cc)) = q.pop_front() {
                    comps[id].mass += 1;
                    comps[id].moment += cc as i64;

                    for (dr, dc) in dirs {
                        let nr = rr as i32 + dr;
                        let nc = cc as i32 + dc;
                        if nr < 0 || nc < 0 || nr >= height as i32 || nc >= width as i32 {
                            continue;
                        }
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if comp_id[nr][nc] != usize::MAX {
                            continue;
                        }
                        let nch = *grid[nr].get(nc).unwrap_or(&b'.');
                        if nch != b'.' {
                            comp_id[nr][nc] = id;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        let n = comps.len();

        // Contact span:
        // - If a component touches the bottom row: use its bottom-row cells.
        // - Otherwise: use its cells that have a solid cell directly beneath.
        let mut touches_bottom = vec![false; n];
        if height > 0 {
            for c in 0..width {
                let id = comp_id[height - 1][c];
                if id != usize::MAX {
                    touches_bottom[id] = true;
                }
            }
            for c in 0..width {
                let id = comp_id[height - 1][c];
                if id == usize::MAX {
                    continue;
                }
                let col = c as i32;
                comps[id].contact_min = comps[id].contact_min.min(col);
                comps[id].contact_max = comps[id].contact_max.max(col);
            }
        }
        for r in 0..height.saturating_sub(1) {
            for c in 0..width {
                let id = comp_id[r][c];
                if id == usize::MAX || touches_bottom[id] {
                    continue;
                }
                let below = *grid[r + 1].get(c).unwrap_or(&b'.');
                if below != b'.' {
                    let col = c as i32;
                    comps[id].contact_min = comps[id].contact_min.min(col);
                    comps[id].contact_max = comps[id].contact_max.max(col);
                }
            }
        }

        // Support relations: below component supports above component.
        let mut has_parent = vec![false; n];
        let mut edge_seen = vec![vec![false; n]; n]; // spec implies bounded sizes
        for r in 0..height.saturating_sub(1) {
            for c in 0..width {
                let above = comp_id[r][c];
                let below = comp_id[r + 1][c];
                if above == usize::MAX || below == usize::MAX || above == below {
                    continue;
                }
                if !edge_seen[below][above] {
                    edge_seen[below][above] = true;
                    comps[below].children.push(above);
                    has_parent[above] = true;
                }
            }
        }

        let roots: Vec<usize> = (0..n).filter(|&i| !has_parent[i]).collect();

        // Post-order traversal.
        let mut visited = vec![0u8; n]; // 0=unseen,1=entered,2=done
        let mut order: Vec<usize> = Vec::with_capacity(n);

        for &root in &roots {
            let mut stack = vec![root];
            while let Some(v) = stack.pop() {
                if visited[v] == 0 {
                    visited[v] = 1;
                    stack.push(v);
                    for &ch in &comps[v].children {
                        if visited[ch] == 0 {
                            stack.push(ch);
                        }
                    }
                } else if visited[v] == 1 {
                    visited[v] = 2;
                    order.push(v);
                }
            }
        }
        for v in 0..n {
            if visited[v] == 0 {
                let mut stack = vec![v];
                while let Some(x) = stack.pop() {
                    if visited[x] == 0 {
                        visited[x] = 1;
                        stack.push(x);
                        for &ch in &comps[x].children {
                            if visited[ch] == 0 {
                                stack.push(ch);
                            }
                        }
                    } else if visited[x] == 1 {
                        visited[x] = 2;
                        order.push(x);
                    }
                }
            }
        }

        for &v in &order {
            let mut agg_mass = comps[v].mass;
            let mut agg_moment = comps[v].moment;
            let mut ok = true;

            for &ch in &comps[v].children {
                agg_mass += comps[ch].agg_mass;
                agg_moment += comps[ch].agg_moment;
                if !comps[ch].stable {
                    ok = false;
                }
            }

            comps[v].agg_mass = agg_mass;
            comps[v].agg_moment = agg_moment;

            // Center-of-mass rule:
            // COM = (moment/mass) + 0.5, must satisfy:
            // contact_min < COM < contact_max + 1
            //
            // The sample requires that equality at the boundary counts as UNSTABLE.
            // Use integer form:
            // 2*mass*contact_min < 2*moment + mass < 2*mass*(contact_max+1)
            if comps[v].contact_min == i32::MAX || comps[v].contact_max == i32::MIN || agg_mass <= 0 {
                ok = false;
            } else {
                let left = comps[v].contact_min as i64;
                let right_plus_1 = comps[v].contact_max as i64 + 1;

                let lhs = 2i64
                    .saturating_mul(agg_mass)
                    .saturating_mul(left);
                let mid = 2i64
                    .saturating_mul(agg_moment)
                    .saturating_add(agg_mass);
                let rhs = 2i64
                    .saturating_mul(agg_mass)
                    .saturating_mul(right_plus_1);

                if !(lhs < mid && mid < rhs) {
                    ok = false;
                }
            }

            comps[v].stable = ok;
        }

        // Global verdict: all roots must be stable.
        let all_stable = if n == 0 {
            true
        } else {
            roots.iter().all(|&r| comps[r].stable)
        };

        out.push_str(if all_stable { "STABLE\n" } else { "UNSTABLE\n" });
    }

    print!("{out}");
}
