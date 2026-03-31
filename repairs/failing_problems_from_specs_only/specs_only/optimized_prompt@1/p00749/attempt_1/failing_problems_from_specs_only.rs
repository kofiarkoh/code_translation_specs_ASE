use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Node {
    parent: usize,        // 0 means unset during construction
    children: Vec<usize>, // in attachment order
    m: i64,               // mass
    mx: i64,              // horizontal moment (sum of column indices)
    xl: i32,              // support interval min (inclusive)
    xr: i32,              // support interval max (inclusive)
    processed: bool,      // aggregated upward already
}

fn main() {
    // Read entire stdin and tokenize like scanf("%d %d %s ...")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();
        let h: i32 = it.next().unwrap().parse().unwrap();

        if w <= 0 {
            break;
        }

        let wu = w as usize;
        let hu = h as usize;

        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(hu);
        for _ in 0..hu {
            let s = it.next().unwrap_or("");
            // Spec: row contains at least w characters; no validation performed.
            let bytes = s.as_bytes();
            let mut row = vec![b'.'; wu];
            for c in 0..wu {
                row[c] = bytes[c];
            }
            grid.push(row);
        }

        // Component formation: 4-neighborhood, same character, excluding '.'
        let mut comp = vec![vec![-1i32; wu]; hu];
        let mut comp_char: Vec<u8> = Vec::new();
        let mut comp_m: Vec<i64> = Vec::new();
        let mut comp_mx: Vec<i64> = Vec::new();

        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
        let mut cid: i32 = 0;

        for r in 0..hu {
            for c in 0..wu {
                if grid[r][c] == b'.' || comp[r][c] != -1 {
                    continue;
                }
                let ch = grid[r][c];
                let this_id = cid;
                cid += 1;

                comp_char.push(ch);
                comp_m.push(0);
                comp_mx.push(0);

                let mut q = VecDeque::new();
                q.push_back((r, c));
                comp[r][c] = this_id;

                while let Some((rr, cc)) = q.pop_front() {
                    comp_m[this_id as usize] += 1;
                    comp_mx[this_id as usize] += cc as i64;

                    for (dr, dc) in dirs {
                        let nr = rr as i32 + dr;
                        let nc = cc as i32 + dc;
                        if nr < 0 || nr >= h || nc < 0 || nc >= w {
                            continue;
                        }
                        let nru = nr as usize;
                        let ncu = nc as usize;
                        if comp[nru][ncu] != -1 {
                            continue;
                        }
                        if grid[nru][ncu] != ch {
                            continue;
                        }
                        comp[nru][ncu] = this_id;
                        q.push_back((nru, ncu));
                    }
                }
            }
        }

        // Build scan-order node IDs and support structure.
        // Internal node IDs are 1-based; 0 means "not assigned".
        let ncomp = cid as usize;
        let mut comp_to_node = vec![0usize; ncomp];

        // nodes[0] is unused sentinel to keep 1-based indexing.
        let mut nodes: Vec<Node> = Vec::with_capacity(ncomp + 1);
        nodes.push(Node {
            parent: 0,
            children: Vec::new(),
            m: 0,
            mx: 0,
            xl: 0,
            xr: -1,
            processed: false,
        });

        let mut next_node_id: usize = 1;

        // Helper to create node on first encounter in scan.
        let mut ensure_node = |comp_id: usize, nodes: &mut Vec<Node>, comp_to_node: &mut [usize]| -> usize {
            if comp_to_node[comp_id] != 0 {
                return comp_to_node[comp_id];
            }
            let nid = nodes.len(); // since nodes[0] exists, this equals next 1-based id
            comp_to_node[comp_id] = nid;
            nodes.push(Node {
                parent: 0,
                children: Vec::new(),
                m: comp_m[comp_id],
                mx: comp_mx[comp_id],
                xl: i32::MAX,
                xr: i32::MIN,
                processed: false,
            });
            nid
        };

        // Bottom-up scan: rows h-1..0, cols 0..w-1
        for rr in (0..hu).rev() {
            for cc in 0..wu {
                if grid[rr][cc] == b'.' {
                    continue;
                }
                let comp_id = comp[rr][cc] as usize;
                let nid = ensure_node(comp_id, &mut nodes, &mut comp_to_node);

                // Support footprint update:
                // - bottom row always contributes
                // - otherwise contributes only if cell below is occupied (any char)
                let supported = if rr + 1 >= hu {
                    true
                } else {
                    grid[rr + 1][cc] != b'.'
                };
                if supported {
                    let col = cc as i32;
                    if col < nodes[nid].xl {
                        nodes[nid].xl = col;
                    }
                    if col > nodes[nid].xr {
                        nodes[nid].xr = col;
                    }
                }

                // Parent/child relation via vertical adjacency, first-parent-wins
                if rr + 1 < hu && grid[rr + 1][cc] != b'.' {
                    let below_comp = comp[rr + 1][cc] as usize;
                    let pid = ensure_node(below_comp, &mut nodes, &mut comp_to_node);

                    if nodes[nid].parent == 0 {
                        nodes[nid].parent = pid;
                        nodes[pid].children.push(nid);
                    }
                }

                // Track next_node_id implicitly via nodes.len(); kept for clarity with spec.
                next_node_id = nodes.len();
                let _ = next_node_id;
            }
        }

        // Spec: root is internal node ID 1 and is forcibly set to be its own parent.
        // Spec assumes at least one occupied cell; otherwise behavior undefined.
        if nodes.len() > 1 {
            nodes[1].parent = 1;
        }

        // Recursive stability check (depth-first over children in stored order).
        fn dfs(u: usize, nodes: &mut [Node]) -> bool {
            // Check all descendants first
            let children = nodes[u].children.clone(); // preserve order; avoid borrow issues
            for v in children {
                if !dfs(v, nodes) {
                    return false;
                }
            }

            // Compute center of mass: x = 0.5 + (mx / m) using real division.
            // Stability: unstable if x <= xl or x >= xr + 1 (open interval (xl, xr+1)).
            // Note: xl/xr may remain extreme if no supported cells were recorded; spec does not define.
            let m = nodes[u].m as f64;
            let mx = nodes[u].mx as f64;
            let x = 0.5f64 + (mx / m);

            let xl = nodes[u].xl as f64;
            let xr1 = (nodes[u].xr as f64) + 1.0;

            if x <= xl || x >= xr1 {
                return false;
            }

            // Aggregate into parent if not already processed.
            if !nodes[u].processed {
                nodes[u].processed = true;
                let p = nodes[u].parent;
                if p != 0 && p != u {
                    let um = nodes[u].m;
                    let umx = nodes[u].mx;
                    nodes[p].m += um;
                    nodes[p].mx += umx;
                }
            }

            true
        }

        let stable = if nodes.len() > 1 { dfs(1, &mut nodes) } else { false };

        if stable {
            out.push_str("STABLE\n");
        } else {
            out.push_str("UNSTABLE\n");
        }
    }

    print!("{out}");
}
