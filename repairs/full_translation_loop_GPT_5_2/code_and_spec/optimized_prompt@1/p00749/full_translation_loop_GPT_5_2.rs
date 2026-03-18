use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Node {
    used: bool,
    m: i32,
    mx: i32,
    xl: i32,
    xr: i32,
    par: usize,
    ch: usize,
    sib: usize,
}

struct UnionFind {
    p: Vec<usize>,
    rank: Vec<u8>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = Vec::with_capacity(n);
        for i in 0..n {
            p.push(i);
        }
        Self {
            p,
            rank: vec![0; n],
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
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x == y {
            return;
        }
        if self.rank[x] > self.rank[y] {
            self.p[y] = x;
        } else {
            self.p[x] = y;
            if self.rank[x] == self.rank[y] {
                self.rank[y] = self.rank[y].saturating_add(1);
            }
        }
    }
}

fn mktree(
    r: usize,
    c: usize,
    this: usize,
    w: usize,
    h: usize,
    map: &[Vec<u8>],
    rel: &[Vec<usize>],
    tr: &[isize],
    tbl: &mut [Node],
) {
    if r == h - 1 {
        if map[r][c] == b'.' {
            return;
        }
        if tbl[this].xl < 0 {
            tbl[this].xl = c as i32;
        } else if (c as i32) < tbl[this].xl {
            tbl[this].xl = c as i32;
        }
        if (c as i32) > tbl[this].xr {
            tbl[this].xr = c as i32;
        }
        return;
    }
    if map[r + 1][c] == b'.' {
        return;
    }

    let k = rel[r + 1][c];
    let p = tr[k];
    if p < 0 {
        // In the original C, this cannot happen because the scan is bottom-up,
        // so any component below has already been assigned a tree index.
        return;
    }
    let p = p as usize;

    if p == this {
        return;
    }

    if tbl[this].xl < 0 {
        tbl[this].xl = c as i32;
    } else if (c as i32) < tbl[this].xl {
        tbl[this].xl = c as i32;
    }
    if (c as i32) > tbl[this].xr {
        tbl[this].xr = c as i32;
    }
    if tbl[this].par != 0 {
        return;
    }

    let mut s = tbl[p].ch;
    if s == 0 {
        tbl[p].ch = this;
    } else {
        while tbl[s].sib != 0 {
            s = tbl[s].sib;
        }
        tbl[s].sib = this;
    }
    tbl[this].par = p;

    let _ = w; // kept to mirror signature intent; unused in logic
}

fn check(this: usize, tbl: &mut [Node]) -> bool {
    if tbl[this].used {
        return true;
    }

    let ch = tbl[this].ch;
    if ch != 0 && !check(ch, tbl) {
        return false;
    }

    let mut sib = tbl[this].sib;
    while sib != 0 {
        if !check(sib, tbl) {
            return false;
        }
        sib = tbl[sib].sib;
    }

    // x = 0.5 + mx/m (as double)
    // Unstable if x <= xl or xr+1 <= x
    // Note: m should be > 0 for any real component; root (1) is a component too.
    let x = 0.5f64 + (tbl[this].mx as f64) / (tbl[this].m as f64);
    if x <= (tbl[this].xl as f64) || ((tbl[this].xr + 1) as f64) <= x {
        return false;
    }

    let par = tbl[this].par;
    tbl[par].m += tbl[this].m;
    tbl[par].mx += tbl[this].mx;
    tbl[this].used = true;
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
        let w: i32 = match w_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // per spec: invalid dims => stop processing, no output
        };
        let h: i32 = match it.next() {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        if w <= 0 {
            break;
        }
        if h <= 0 {
            // Spec says invalid dims => do not process further and no output for that case.
            break;
        }
        let w = w as usize;
        let h = h as usize;

        let mut map: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let row = match it.next() {
                Some(s) => s.as_bytes().to_vec(),
                None => return,
            };
            // Assume well-formed input; if shorter/longer, we still keep bytes as given.
            map.push(row);
        }

        let n = w * h;
        let mut uf = UnionFind::new(n);

        let mut rel: Vec<Vec<usize>> = vec![vec![0; w]; h];
        for r in 0..h {
            for c in 0..w {
                rel[r][c] = r * w + c;
            }
        }

        let mv = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
        for r in 0..h {
            for c in 0..w {
                if map[r][c] == b'.' {
                    continue;
                }
                for (dr, dc) in mv {
                    let r2 = r as i32 + dr;
                    let c2 = c as i32 + dc;
                    if r2 >= 0 && r2 < h as i32 && c2 >= 0 && c2 < w as i32 {
                        let r2u = r2 as usize;
                        let c2u = c2 as usize;
                        if map[r2u][c2u] == map[r][c] {
                            let a = rel[r][c];
                            let b = rel[r2u][c2u];
                            uf.union(a, b);
                        }
                    }
                }
            }
        }

        // Tree building
        // C uses tbl[100] and tr[MAX=602]. We'll size dynamically.
        let mut tbl: Vec<Node> = vec![Node::default(); n + 2]; // indices start at 1
        let mut tr: Vec<isize> = vec![-1; n];

        let mut sz: usize = 1; // next free index; root component will become 1 if first seen
        for r in (0..h).rev() {
            for c in 0..w {
                if map[r][c] == b'.' {
                    continue;
                }
                let k = uf.find(rel[r][c]);
                rel[r][c] = k;

                let idx: usize;
                if tr[k] < 0 {
                    idx = sz;
                    tr[k] = idx as isize;
                    sz += 1;
                    tbl[idx].xl = -1;
                    tbl[idx].xr = 0;
                } else {
                    idx = tr[k] as usize;
                }

                tbl[idx].m += 1;
                tbl[idx].mx += c as i32;

                mktree(r, c, idx, w, h, &map, &rel, &tr, &mut tbl);
            }
        }

        // Root's parent is itself (as in C).
        if tbl.len() > 1 {
            tbl[1].par = 1;
        }

        let stable = check(1, &mut tbl);
        out.push_str(if stable { "STABLE\n" } else { "UNSTABLE\n" });
    }

    print!("{out}");
}
