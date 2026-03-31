use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Cell {
    ch: u8,
    comp_rep: usize, // union-find representative id (0..n-1), valid only if ch != b'.'
}

#[derive(Debug, Clone)]
struct Comp {
    // Intrinsic/aggregated properties (m, mx are mutated during evaluation)
    m: i64,
    mx: i64,
    xl: i32,
    xr: i32,

    // Support structure (single-parent)
    parent: Option<usize>, // internal component index (1-based)
    children: Vec<usize>,  // internal component indices (1-based), in append order

    // For evaluation bookkeeping
    visited: bool,
}

struct DSU {
    p: Vec<usize>,
    r: Vec<u8>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            r: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            let root = self.find(self.p[x]);
            self.p[x] = root;
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.r[ra] < self.r[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.p[rb] = ra;
        if self.r[ra] == self.r[rb] {
            self.r[ra] = self.r[ra].wrapping_add(1);
        }
    }
}

fn idx(r: usize, c: usize, w: usize) -> usize {
    r * w + c
}

fn dfs_check(u: usize, comps: &mut [Comp]) -> bool {
    // Post-order: process children first, then check u, then aggregate into parent.
    if comps[u].visited {
        return true;
    }
    comps[u].visited = true;

    // Clone children list to avoid borrow issues while mutating comps during recursion.
    let children = comps[u].children.clone();
    for &v in &children {
        if !dfs_check(v, comps) {
            return false;
        }
        // After successful child processing, aggregate child's mass/moment into parent.
        let (cm, cmx) = (comps[v].m, comps[v].mx);
        comps[u].m += cm;
        comps[u].mx += cmx;
    }

    // Balance check:
    // x = 0.5 + (mx / m) using real division.
    // Condition: xl < x < xr + 1 (strict).
    let m = comps[u].m;
    let mx = comps[u].mx;
    // m should be > 0 for any real component; if not, treat as failing narrowly.
    if m <= 0 {
        return false;
    }
    let x = 0.5f64 + (mx as f64) / (m as f64);
    let xl = comps[u].xl as f64;
    let xr1 = (comps[u].xr as f64) + 1.0;
    if !(xl < x && x < xr1) {
        return false;
    }

    true
}

fn main() {
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
        let _h_token = it.next();
        if _h_token.is_none() {
            break;
        }
        let h: i32 = _h_token.unwrap().parse().unwrap();

        if w <= 0 {
            break;
        }
        let (w_us, h_us) = (w as usize, h as usize);

        // Read grid rows (each token is a row with no spaces).
        let mut grid_bytes: Vec<Vec<u8>> = Vec::with_capacity(h_us);
        for _ in 0..h_us {
            let row = it.next().unwrap_or("");
            // Spec: each row provides at least w characters; take first w.
            let mut bytes = row.as_bytes().to_vec();
            if bytes.len() < w_us {
                // Narrowest behavior: pad with '.' if short (though spec says it won't be).
                bytes.resize(w_us, b'.');
            } else if bytes.len() > w_us {
                bytes.truncate(w_us);
            }
            grid_bytes.push(bytes);
        }

        let n = w_us * h_us;
        let mut dsu = DSU::new(n);

        // Build connectivity components by unioning orthogonally adjacent equal non-empty cells.
        for r in 0..h_us {
            for c in 0..w_us {
                let ch = grid_bytes[r][c];
                if ch == b'.' {
                    continue;
                }
                if c + 1 < w_us && grid_bytes[r][c + 1] == ch {
                    dsu.union(idx(r, c, w_us), idx(r, c + 1, w_us));
                }
                if r + 1 < h_us && grid_bytes[r + 1][c] == ch {
                    dsu.union(idx(r, c, w_us), idx(r + 1, c, w_us));
                }
            }
        }

        // Prepare cell table with representative ids.
        let mut cells: Vec<Cell> = Vec::with_capacity(n);
        for r in 0..h_us {
            for c in 0..w_us {
                let ch = grid_bytes[r][c];
                let rep = if ch == b'.' {
                    0
                } else {
                    dsu.find(idx(r, c, w_us))
                };
                cells.push(Cell { ch, comp_rep: rep });
            }
        }

        // Bottom-up scan to assign internal indices and build per-component properties and support links.
        let mut rep_to_internal: Vec<Option<usize>> = vec![None; n];
        // comps[0] unused to keep 1-based indexing.
        let mut comps: Vec<Comp> = vec![Comp {
            m: 0,
            mx: 0,
            xl: 0,
            xr: -1,
            parent: None,
            children: Vec::new(),
            visited: false,
        }];

        let mut next_internal: usize = 1;

        for rr in (0..h_us).rev() {
            for cc in 0..w_us {
                let cell = cells[idx(rr, cc, w_us)];
                if cell.ch == b'.' {
                    continue;
                }
                let rep = cell.comp_rep;
                let internal = match rep_to_internal[rep] {
                    Some(id) => id,
                    None => {
                        let id = next_internal;
                        next_internal += 1;
                        rep_to_internal[rep] = Some(id);
                        comps.push(Comp {
                            m: 0,
                            mx: 0,
                            xl: cc as i32,
                            xr: cc as i32,
                            parent: None,
                            children: Vec::new(),
                            visited: false,
                        });
                        id
                    }
                };

                // Accumulate intrinsic properties.
                {
                    let comp = &mut comps[internal];
                    comp.m += 1;
                    comp.mx += cc as i64;
                    let c_i32 = cc as i32;
                    if c_i32 < comp.xl {
                        comp.xl = c_i32;
                    }
                    if c_i32 > comp.xr {
                        comp.xr = c_i32;
                    }
                }

                // Support link construction with stop conditions.
                // If bottom row: no parent attempt.
                if rr == h_us - 1 {
                    continue;
                }
                // If already has parent: frozen.
                if comps[internal].parent.is_some() {
                    continue;
                }
                // Check below cell.
                let below = cells[idx(rr + 1, cc, w_us)];
                if below.ch == b'.' {
                    continue;
                }
                let below_rep = below.comp_rep;
                let below_internal = match rep_to_internal[below_rep] {
                    Some(id) => id,
                    None => {
                        // Because scan is bottom-up, the below component should have been seen already.
                        // Narrowest behavior: create it if not yet created (should not happen).
                        let id = next_internal;
                        next_internal += 1;
                        rep_to_internal[below_rep] = Some(id);
                        comps.push(Comp {
                            m: 0,
                            mx: 0,
                            xl: cc as i32,
                            xr: cc as i32,
                            parent: None,
                            children: Vec::new(),
                            visited: false,
                        });
                        id
                    }
                };
                if below_internal == internal {
                    continue;
                }

                // Assign parent and append to parent's child list.
                comps[internal].parent = Some(below_internal);
                comps[below_internal].children.push(internal);
            }
        }

        // If no components were discovered, there is no internal index 1.
        // Narrowest interpretation: evaluation cannot start => UNSTABLE.
        let stable = if comps.len() <= 1 {
            false
        } else {
            // Force root (component 1) to be its own parent for bookkeeping.
            comps[1].parent = Some(1);

            // Evaluate only reachable from root via children links.
            dfs_check(1, &mut comps)
        };

        if stable {
            out.push_str("STABLE\n");
        } else {
            out.push_str("UNSTABLE\n");
        }
    }

    print!("{out}");
}
