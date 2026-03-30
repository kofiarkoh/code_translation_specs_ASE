use std::io::{self, BufRead};

const MAX: usize = 602;
const MAX_ROWS: usize = 60;
const MAX_COLS: usize = 12;

#[derive(Clone)]
struct Component {
    used: bool,
    m: usize,
    mx: usize,
    xl: isize,
    xr: isize,
    par: isize,
    ch: isize,
    sib: isize,
}

impl Component {
    fn new() -> Self {
        Component {
            used: false,
            m: 0,
            mx: 0,
            xl: -1,
            xr: -1,
            par: -1,
            ch: 0,
            sib: 0,
        }
    }
}

struct UnionFind {
    parent: [usize; MAX],
    rank: [usize; MAX],
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: (0..MAX).collect::<Vec<_>>().try_into().unwrap(),
            rank: [0; MAX],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
                if self.rank[root_x] == self.rank[root_y] {
                    self.rank[root_y] += 1;
                }
            }
        }
    }
}

fn mktree(r: usize, c: usize, this: usize, h: usize, map: &Vec<Vec<char>>, tbl: &mut Vec<Component>, rel: &Vec<Vec<usize>>, tr: &Vec<isize>) {
    if r == h - 1 {
        if map[r][c] == '.' {
            return;
        }
        if tbl[this].xl < 0 {
            tbl[this].xl = c as isize;
        } else if c as isize < tbl[this].xl {
            tbl[this].xl = c as isize;
        }
        if c as isize > tbl[this].xr {
            tbl[this].xr = c as isize;
        }
        return;
    }
    if map[r + 1][c] == '.' {
        return;
    }

    let k = rel[r + 1][c];
    let p = tr[k];
    if p == this as isize {
        return;
    }

    if tbl[this].xl < 0 {
        tbl[this].xl = c as isize;
    } else if c as isize < tbl[this].xl {
        tbl[this].xl = c as isize;
    }
    if c as isize > tbl[this].xr {
        tbl[this].xr = c as isize;
    }
    if tbl[this].par != -1 {
        return;
    }

    let s = tbl[p as usize].ch;
    if s == 0 {
        tbl[p as usize].ch = this as isize;
    } else {
        let mut current = s;
        while current != 0 {
            current = tbl[current as usize].sib;
        }
        tbl[current as usize].sib = this as isize;
    }
    tbl[this].par = p;
}

fn check(this: usize, tbl: &mut Vec<Component>) -> bool {
    if tbl[this].used {
        return true;
    }
    let ch = tbl[this].ch;
    if ch != 0 && !check(ch as usize, tbl) {
        return false;
    }

    let mut sib = tbl[this].sib;
    while sib != 0 {
        if !check(sib as usize, tbl) {
            return false;
        }
        sib = tbl[sib as usize].sib;
    }

    let x = 0.5 + tbl[this].mx as f64 / tbl[this].m as f64;
    if x <= tbl[this].xl as f64 || tbl[this].xr as f64 + 1.0 <= x {
        return false;
    }

    let par = tbl[this].par;
    tbl[par as usize].m += tbl[this].m;
    tbl[par as usize].mx += tbl[this].mx;
    tbl[this].used = true;
    true
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut map = vec![vec!['.'; MAX_COLS]; MAX_ROWS];
    let mut rel = vec![vec![0; MAX_COLS]; MAX_ROWS];
    let mut uf = UnionFind::new();
    let mut tbl = vec![Component::new(); 100];
    let mut tr = vec![-1; MAX];

    while let Some(line) = lines.next() {
        let dimensions: Vec<usize> = line.unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let (w, h) = (dimensions[0], dimensions[1]);
        if w == 0 || h == 0 {
            break;
        }

        for r in 0..h {
            let row = lines.next().unwrap().unwrap();
            for c in 0..w {
                map[r][c] = row.chars().nth(c).unwrap();
                let i = r * w + c;
                rel[r][c] = i;
                uf.parent[i] = i;
            }
        }

        for r in 0..h {
            for c in 0..w {
                if map[r][c] == '.' {
                    continue;
                }
                for (dr, dc) in &[(0, 1isize), (1, 0isize), (0, -1isize), (-1, 0isize)] {
                    let r2 = (r as isize + dr) as usize;
                    let c2 = (c as isize + dc) as usize;
                    if r2 < h && c2 < w && map[r2][c2] == map[r][c] {
                        uf.union(rel[r][c], rel[r2][c2]);
                    }
                }
            }
        }

        let mut sz = 1;
        for r in (0..h).rev() {
            for c in 0..w {
                if map[r][c] == '.' {
                    continue;
                }
                let k = rel[r][c] = uf.find(rel[r][c]);
                let i = if tr[k] < 0 {
                    tr[k] = sz;
                    sz += 1;
                    tbl[tr[k] as usize].xl = -1;
                    tr[k]
                } else {
                    tr[k]
                };
                tbl[i as usize].m += 1;
                tbl[i as usize].mx += c;
                mktree(r, c, i as usize, h, &map, &mut tbl, &rel, &tr);
            }
        }
        tbl[1].par = 1;

        println!("{}", if check(1, &mut tbl) { "STABLE" } else { "UNSTABLE" });
    }
}
