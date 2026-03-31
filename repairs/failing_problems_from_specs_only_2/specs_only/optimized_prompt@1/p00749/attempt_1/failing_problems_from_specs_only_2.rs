use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

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

        let w_usize = w as usize;
        let h_usize = h as usize;

        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h_usize);
        for _ in 0..h_usize {
            let s = it.next().unwrap();
            // Spec: row contains at least w characters; no validation performed.
            grid.push(s.as_bytes()[0..w_usize].to_vec());
        }

        // Component labeling by 4-neighborhood with same character (excluding '.').
        let mut comp: Vec<Vec<i32>> = vec![vec![-1; w_usize]; h_usize];
        let mut comp_char: Vec<u8> = Vec::new();
        let mut comp_m: Vec<i64> = Vec::new();
        let mut comp_mx: Vec<i64> = Vec::new();

        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
        let mut cid: i32 = 0;

        for r in 0..h_usize {
            for c in 0..w_usize {
                let ch = grid[r][c];
                if ch == b'.' || comp[r][c] != -1 {
                    continue;
                }
                let my_id = cid;
                cid += 1;

                comp_char.push(ch);
                comp_m.push(0);
                comp_mx.push(0);

                let mut q = VecDeque::new();
                q.push_back((r, c));
                comp[r][c] = my_id;

                while let Some((rr, cc)) = q.pop_front() {
                    comp_m[my_id as usize] += 1;
                    comp_mx[my_id as usize] += cc as i64;

                    for (dr, dc) in dirs {
                        let nr = rr as i32 + dr;
                        let nc = cc as i32 + dc;
                        if nr < 0 || nc < 0 || nr >= h || nc >= w {
                            continue;
                        }
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if comp[nr][nc] != -1 {
                            continue;
                        }
                        if grid[nr][nc] == ch {
                            comp[nr][nc] = my_id;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        let ncomp = cid as usize;

        // Node IDs assigned in bottom-up scan order when first encountered.
        // node_id_of_comp: component index -> node id (1..=nnode), 0 if not assigned yet.
        let mut node_id_of_comp: Vec<usize> = vec![0; ncomp];
        let mut comp_of_node: Vec<usize> = vec![0]; // 1-based; comp_of_node[0] unused
        let mut nnode: usize = 0;

        // Per-node data (1-based).
        let mut parent: Vec<usize> = vec![0]; // parent[node]
        let mut children: Vec<Vec<usize>> = vec![Vec::new()]; // children[node]
        let mut xl: Vec<i32> = vec![0];
        let mut xr: Vec<i32> = vec![0];
        let mut has_support: Vec<bool> = vec![false];

        // Per-node mass/moment (aggregated during stability check).
        let mut m: Vec<i64> = vec![0];
        let mut mx: Vec<i64> = vec![0];

        // Helper to create a node for a component if needed.
        let mut ensure_node = |comp_idx: usize,
                               node_id_of_comp: &mut Vec<usize>,
                               comp_of_node: &mut Vec<usize>,
                               parent: &mut Vec<usize>,
                               children: &mut Vec<Vec<usize>>,
                               xl: &mut Vec<i32>,
                               xr: &mut Vec<i32>,
                               has_support: &mut Vec<bool>,
                               m: &mut Vec<i64>,
                               mx: &mut Vec<i64>,
                               nnode: &mut usize| {
            if node_id_of_comp[comp_idx] == 0 {
                *nnode += 1;
                let nid = *nnode;
                node_id_of_comp[comp_idx] = nid;

                comp_of_node.push(comp_idx);
                parent.push(0);
                children.push(Vec::new());
                xl.push(0);
                xr.push(0);
                has_support.push(false);

                // Initialize node mass/moment from component totals.
                m.push(comp_m[comp_idx]);
                mx.push(comp_mx[comp_idx]);
            }
            node_id_of_comp[comp_idx]
        };

        // Bottom-up scan: build support footprint and single-parent structure.
        for rr in (0..h_usize).rev() {
            for cc in 0..w_usize {
                if grid[rr][cc] == b'.' {
                    continue;
                }
                let comp_idx = comp[rr][cc] as usize;
                let nid = ensure_node(
                    comp_idx,
                    &mut node_id_of_comp,
                    &mut comp_of_node,
                    &mut parent,
                    &mut children,
                    &mut xl,
                    &mut xr,
                    &mut has_support,
                    &mut m,
                    &mut mx,
                    &mut nnode,
                );

                // Support footprint contribution.
                let supported = if rr == h_usize - 1 {
                    true
                } else {
                    grid[rr + 1][cc] != b'.'
                };
                if supported {
                    if !has_support[nid] {
                        has_support[nid] = true;
                        xl[nid] = cc as i32;
                        xr[nid] = cc as i32;
                    } else {
                        if (cc as i32) < xl[nid] {
                            xl[nid] = cc as i32;
                        }
                        if (cc as i32) > xr[nid] {
                            xr[nid] = cc as i32;
                        }
                    }
                }

                // Parent/child attachment via vertical adjacency, first-parent-wins.
                if rr != h_usize - 1 && grid[rr + 1][cc] != b'.' {
                    let below_comp_idx = comp[rr + 1][cc] as usize;
                    let below_nid = ensure_node(
                        below_comp_idx,
                        &mut node_id_of_comp,
                        &mut comp_of_node,
                        &mut parent,
                        &mut children,
                        &mut xl,
                        &mut xr,
                        &mut has_support,
                        &mut m,
                        &mut mx,
                        &mut nnode,
                    );

                    if parent[nid] == 0 {
                        parent[nid] = below_nid;
                        children[below_nid].push(nid);
                    }
                }
            }
        }

        // Root selection: node ID 1 is forced to be its own parent.
        // Spec assumes at least one occupied cell; otherwise behavior is undefined.
        if nnode >= 1 {
            parent[1] = 1;
        }

        // Stability check: DFS over child links from root (node 1).
        // After validating descendants, check node's center of mass strictly inside (xl, xr+1).
        // If stable, aggregate node's m/mx into its parent once.
        let mut processed: Vec<bool> = vec![false; nnode + 1];

        fn dfs(
            u: usize,
            children: &Vec<Vec<usize>>,
            xl: &Vec<i32>,
            xr: &Vec<i32>,
            has_support: &Vec<bool>,
            parent: &Vec<usize>,
            m: &mut Vec<i64>,
            mx: &mut Vec<i64>,
            processed: &mut Vec<bool>,
        ) -> bool {
            for &v in &children[u] {
                if !dfs(v, children, xl, xr, has_support, parent, m, mx, processed) {
                    return false;
                }
            }

            // If a component never accumulated any supported column, its footprint is undefined.
            // Narrowest faithful behavior: treat as unstable (cannot satisfy strict containment).
            if !has_support[u] {
                return false;
            }

            let mu = m[u];
            let mxu = mx[u];

            // x = 0.5 + (mx / m) using real division.
            let x = 0.5f64 + (mxu as f64) / (mu as f64);

            // Unstable if x <= xl or x >= xr + 1.
            let left = xl[u] as f64;
            let right = (xr[u] + 1) as f64;
            if x <= left || x >= right {
                return false;
            }

            if !processed[u] {
                processed[u] = true;
                let p = parent[u];
                if p != 0 && p != u {
                    m[p] += m[u];
                    mx[p] += mx[u];
                }
            }
            true
        }

        let stable = if nnode >= 1 {
            dfs(
                1,
                &children,
                &xl,
                &xr,
                &has_support,
                &parent,
                &mut m,
                &mut mx,
                &mut processed,
            )
        } else {
            // Spec says this case is undefined; choose a narrow, deterministic output.
            false
        };

        if stable {
            println!("STABLE");
        } else {
            println!("UNSTABLE");
        }
    }
}
