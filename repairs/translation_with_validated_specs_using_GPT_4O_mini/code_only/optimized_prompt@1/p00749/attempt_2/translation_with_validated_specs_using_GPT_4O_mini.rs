use std::io::{self, BufRead};

const MAX: usize = 602;
const MAX_TBL: usize = 100;

struct UnionFind {
    p: [usize; MAX],
    rank: [usize; MAX],
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            p: (0..MAX).collect::<Vec<_>>().try_into().unwrap(),
            rank: [0; MAX],
        }
    }

    fn make_set(&mut self, x: usize) {
        self.p[x] = x;
        self.rank[x] = 0;
    }

    fn find_set(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find_set(self.p[x]);
        }
        self.p[x]
    }

    fn union_set(&mut self, x: usize, y: usize) {
        let root_x = self.find_set(x);
        let root_y = self.find_set(y);
        self.link(root_x, root_y);
    }

    fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.p[y] = x;
        } else {
            self.p[x] = y;
            if self.rank[x] == self.rank[y] {
                self.rank[y] += 1;
            }
        }
    }
}

struct Node {
    used: bool,
    m: usize,
    mx: usize,
    xl: isize,
    xr: isize,
    par: usize,
    ch: usize,
    sib: usize,
}

struct Tree {
    tbl: [Node; MAX_TBL],
    sz: usize,
}

impl Tree {
    fn new() -> Self {
        Tree {
            tbl: [Node {
                used: false,
                m: 0,
                mx: 0,
                xl: -1,
                xr: -1,
                par: 0,
                ch: 0,
                sib: 0,
            }; MAX_TBL],
            sz: 0,
        }
    }

    fn mktree(&mut self, r: usize, c: usize, this: usize, h: usize, map: &Vec<Vec<char>>, rel: &Vec<Vec<usize>>, tr: &Vec<isize>) {
        if r == h - 1 {
            if map[r][c] == '.' {
                return;
            }
            if self.tbl[this].xl < 0 {
                self.tbl[this].xl = c as isize;
            } else if c as isize < self.tbl[this].xl {
                self.tbl[this].xl = c as isize;
            }
            if c as isize > self.tbl[this].xr {
                self.tbl[this].xr = c as isize;
            }
            return;
        }
        if map[r + 1][c] == '.' {
            return;
        }

        let k = rel[r + 1][c];
        let p = tr[k];

        if p as usize == this {
            return;
        }

        if self.tbl[this].xl < 0 {
            self.tbl[this].xl = c as isize;
        } else if c as isize < self.tbl[this].xl {
            self.tbl[this].xl = c as isize;
        }
        if c as isize > self.tbl[this].xr {
            self.tbl[this].xr = c as isize;
        }
        if self.tbl[this].par != 0 {
            return;
        }

        let s = self.tbl[p as usize].ch;
        if s == 0 {
            self.tbl[p as usize].ch = this;
        } else {
            let mut s = s;
            while self.tbl[s].sib != 0 {
                s = self.tbl[s].sib;
            }
            self.tbl[s].sib = this;
        }
        self.tbl[this].par = p as usize;
    }

    fn check(&mut self, this: usize) -> bool {
        if self.tbl[this].used {
            return true;
        }
        let ch = self.tbl[this].ch;
        if ch != 0 && !self.check(ch) {
            return false;
        }

        let mut sib = self.tbl[this].sib;
        while sib != 0 {
            if !self.check(sib) {
                return false;
            }
            sib = self.tbl[sib].sib;
        }

        let x = 0.5 + self.tbl[this].mx as f64 / self.tbl[this].m as f64;
        if x <= self.tbl[this].xl as f64 || self.tbl[this].xr as f64 + 1.0 <= x {
            return false;
        }

        let par = self.tbl[this].par;
        self.tbl[par].m += self.tbl[this].m;
        self.tbl[par].mx += self.tbl[this].mx;
        self.tbl[this].used = true;
        true
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut union_find = UnionFind::new();
    let mut map: Vec<Vec<char>>;
    let mut rel: Vec<Vec<usize>>;
    let mut tr: Vec<isize>;
    let mut tree = Tree::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace();
        let w: usize = dimensions.next().unwrap().parse().unwrap();
        let h: usize = dimensions.next().unwrap().parse().unwrap();

        if w == 0 {
            break;
        }

        map = vec![vec![' '; w]; h];
        rel = vec![vec![0; w]; h];
        tr = vec![-1; MAX];

        for r in 0..h {
            let row = lines.next().unwrap().unwrap();
            for c in 0..w {
                map[r][c] = row.chars().nth(c).unwrap();
                let i = r * w + c;
                union_find.make_set(i);
                rel[r][c] = i;
            }
        }

        let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for r in 0..h {
            for c in 0..w {
                if map[r][c] == '.' {
                    continue;
                }
                for &(dr, dc) in &mv {
                    let r2 = r as isize + dr;
                    let c2 = c as isize + dc;
                    if r2 >= 0 && r2 < h as isize && c2 >= 0 && c2 < w as isize {
                        if map[r2 as usize][c2 as usize] == map[r][c] {
                            union_find.union_set(rel[r][c], rel[r2 as usize][c2 as usize]);
                        }
                    }
                }
            }
        }

        tree.sz = 1;
        for r in (0..h).rev() {
            for c in 0..w {
                if map[r][c] == '.' {
                    continue;
                }
                let k = rel[r][c] = union_find.find_set(rel[r][c]);
                let i = if tr[k] < 0 {
                    tr[k] = tree.sz as isize;
                    tree.sz += 1;
                    let tp = &mut tree.tbl[tr[k] as usize];
                    tp.xl = -1;
                    tr[k]
                } else {
                    tr[k]
                };
                let tp = &mut tree.tbl[i as usize];
                tp.m += 1;
                tp.mx += c;
                tree.mktree(r, c, i as usize, h, &map, &rel, &tr);
            }
        }
        tree.tbl[1].par = 1;

        println!("{}", if tree.check(1) { "STABLE" } else { "UNSTABLE" });
    }
}
