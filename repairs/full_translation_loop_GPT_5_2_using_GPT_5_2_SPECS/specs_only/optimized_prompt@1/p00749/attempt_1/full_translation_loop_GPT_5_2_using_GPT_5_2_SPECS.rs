use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Comp {
    ch: u8,
    mass: i64,
    moment: i64, // sum of column indices of own cells
    // support span based on "bottom-contact columns"
    contact_min: i32,
    contact_max: i32,
    // support relations: this component supports these components above it
    children: Vec<usize>,
    // aggregated values including supported components
    agg_mass: i64,
    agg_moment: i64,
    stable: bool,
}

fn main() {
    // Read entire stdin; parse as whitespace-separated tokens.
    // Grid rows are tokens; precondition says each row has at least width chars.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: i32 = match w_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let _h_token = it.next();
        if _h_token.is_none() {
            break;
        }
        let h: i32 = match _h_token.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if w <= 0 {
            break; // termination condition; no output for this line
        }
        if h <= 0 {
            // Not specified; treat as malformed; no reliable behavior.
            // Narrowest reasonable: stop processing.
            break;
        }

        let width = w as usize;
        let height = h as usize;

        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(height);
        for _ in 0..height {
            let row = match it.next() {
                Some(s) => s.as_bytes().to_vec(),
                None => Vec::new(),
            };
            grid.push(row);
        }

        // Component labeling: connected via 4-neighborhood AND same character (non-dot).
        let mut comp_id = vec![vec![usize::MAX; width]; height];
        let mut comps: Vec<Comp> = Vec::new();

        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];

        for r in 0..height {
            for c in 0..width {
                let ch = *grid[r].get(c).unwrap_or(&b'.');
                if ch == b'.' {
                    continue;
                }
                if comp_id[r][c] != usize::MAX {
                    continue;
                }

                let id = comps.len();
                comps.push(Comp {
                    ch,
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
                        if nch == ch {
                            comp_id[nr][nc] = id;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        // Compute contact span for each component:
        // "minimum and maximum column indices among its cells that have solid support directly beneath them
        // (or, for bottom-row cells, among its bottom-row cells)."
        //
        // Interpret "solid support directly beneath" as: the cell below is solid (any non-dot),
        // regardless of character/component.
        for r in 0..height {
            for c in 0..width {
                let id = comp_id[r][c];
                if id == usize::MAX {
                    continue;
                }
                let is_bottom_row = r + 1 == height;
                let supported_below = if is_bottom_row {
                    true
                } else {
                    let below = *grid[r + 1].get(c).unwrap_or(&b'.');
                    below != b'.'
                };
                if supported_below {
                    let col = c as i32;
                    if col < comps[id].contact_min {
                        comps[id].contact_min = col;
                    }
                    if col > comps[id].contact_max {
                        comps[id].contact_max = col;
                    }
                }
            }
        }

        // Derive support relations between components:
        // A component A supports component B if any cell of B has a solid cell directly below it
        // that belongs to A. This creates edges A -> B (supporter -> supported).
        //
        // Multiple contacts still yield a single edge.
        let n = comps.len();
        let mut has_parent = vec![false; n];
        let mut edge_seen = vec![vec![false; n]; n]; // bounded by implementation; spec implies fixed-size
        for r in 0..height.saturating_sub(1) {
            for c in 0..width {
                let above_id = comp_id[r][c];
                if above_id == usize::MAX {
                    continue;
                }
                let below_id = comp_id[r + 1][c];
                if below_id == usize::MAX {
                    continue;
                }
                if above_id == below_id {
                    continue;
                }
                // below supports above
                if !edge_seen[below_id][above_id] {
                    edge_seen[below_id][above_id] = true;
                    comps[below_id].children.push(above_id);
                    has_parent[above_id] = true;
                }
            }
        }

        // Find roots (components with no parent). Spec assumes single connected structure;
        // if multiple roots exist, behavior is not guaranteed. Narrowest reasonable: require all roots stable.
        let roots: Vec<usize> = (0..n).filter(|&i| !has_parent[i]).collect();

        // Post-order traversal to aggregate mass/moment and check stability.
        // Use DFS with explicit stack to avoid recursion limits.
        let mut visited = vec![0u8; n]; // 0=unseen,1=entered,2=done
        let mut order: Vec<usize> = Vec::with_capacity(n);

        for &root in &roots {
            let mut stack: Vec<usize> = vec![root];
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
        // If there are components not reachable from any root (cycle or disconnected parent marking),
        // include them as separate roots to keep behavior defined.
        for v in 0..n {
            if visited[v] == 0 {
                let mut stack: Vec<usize> = vec![v];
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

        // Process in post-order (children before parent).
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

            // Stability rule:
            // center_of_mass = average column position of all solid cells in this component plus all it supports,
            // with a half-cell offset. That is: COM = (moment/mass) + 0.5.
            //
            // Must lie strictly within (leftmost_contact, rightmost_contact + 1).
            // Avoid floating point: check strict inequalities:
            // left < (moment/mass + 0.5) < right+1
            // Multiply by 2*mass:
            // 2*mass*left < 2*moment + mass
            // 2*moment + mass < 2*mass*(right+1)
            //
            // If contact span is missing (no supported/bottom cells), treat as unstable (narrowest).
            if comps[v].contact_min == i32::MAX || comps[v].contact_max == i32::MIN || agg_mass <= 0 {
                ok = false;
            } else {
                let left = comps[v].contact_min as i64;
                let right_plus_1 = (comps[v].contact_max as i64) + 1;

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

        let all_stable = if n == 0 {
            // No solid cells: spec doesn't define; narrowest: stable (nothing to violate).
            true
        } else {
            // Require all roots stable; if multiple roots exist, this is the narrowest global verdict.
            roots.iter().all(|&r| comps[r].stable)
        };

        out.push_str(if all_stable { "STABLE\n" } else { "UNSTABLE\n" });
    }

    print!("{out}");
}
