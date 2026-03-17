// AOJ 1168: Off Balance
// 2017.12.22 bal4u@uu

use std::io::{self, BufRead};

const MAX: usize = 602;
const MAX_TBL: usize = 100;
const MAX_MAP: usize = 60;

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

    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.p[root_y] = root_x;
            } else {
                self.p[root_x] = root_y;
                if self.rank[root_x] == self.rank[root_y] {
                    self.rank[root_y] += 1;
                }
            }
        }
    }
}

#[derive(Default)]
struct TreeNode {
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
    nodes: [TreeNode; MAX_TBL],
    sz: usize,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: [TreeNode::default(); MAX_TBL],
            sz: 0,
        }
    }

    fn mktree(&mut self, r: usize, c: usize, this: usize, h: usize, map: &Vec<Vec<char>>, rel: &Vec<Vec<usize>>, tr: &Vec<isize>) {
        if r == h - 1 {
            if map[r][c] == '.' {
                return;
            }
            if self.nodes[this].xl < 0 {
                self.nodes[this].xl = c as isize;
            } else if c as isize < self.nodes[this].xl {
                self.nodes[this].xl = c as isize;
            }
            if c as isize > self.nodes[this].xr {
                self.nodes[this].xr = c as isize;
            }
            return;
        }
        if map[r + 1][c] == '.' {
            return;
        }

        let k = rel[r + 1][c];
        let p = tr[k] as usize;
        if p == this {
            return;
        }

        if self.nodes[this].xl < 0 {
            self.nodes[this].xl = c as isize;
        } else if c as isize < self.nodes[this].xl {
            self.nodes[this].xl = c as isize;
        }
        if c as isize > self.nodes[this].xr {
            self.nodes[this].xr = c as isize;
        }
        if self.nodes[this].par != 0 {
            return;
        }

        let s = self.nodes[p].ch;
        if s == 0 {
            self.nodes[p].ch = this;
        } else {
            let mut current = s;
            while self.nodes[current].sib != 0 {
                current = self.nodes[current].sib;
            }
            self.nodes[current].sib = this;
        }
        self.nodes[this].par = p;
    }

    fn check(&mut self, this: usize) -> bool {
        if self.nodes[this].used {
            return true;
        }
        let ch = self.nodes[this].ch;
        if ch != 0 && !self.check(ch) {
            return false;
        }

        let mut sib = self.nodes[this].sib;
        while sib != 0 {
            if !self.check(sib) {
                return false;
            }
            sib = self.nodes[sib].sib;
        }

        let x = 0.5 + self.nodes[this].mx as f64 / self.nodes[this].m as f64;
        if x <= self.nodes[this].xl as f64 || self.nodes[this].xr as f64 + 1.0 <= x {
            return false;
        }

        let par = self.nodes[this].par;
        self.nodes[par].m += self.nodes[this].m;
        self.nodes[par].mx += self.nodes[this].mx;
        self.nodes[this].used = true;
        true
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut union_find = UnionFind::new();
    let mut map = vec![vec![' '; 12]; MAX_MAP];
    let mut rel = vec![vec![0; 10]; MAX_MAP];
    let mut tr = vec![-1; MAX];
    let mut tree = Tree::new();
    let mv: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let w: usize = iter.next().unwrap().parse().unwrap();
        let h: usize = iter.next().unwrap().parse().unwrap();

        if w == 0 {
            break;
        }

        for r in 0..h {
            let line = lines.next().unwrap().unwrap();
            for c in 0..w {
                map[r][c] = line.chars().nth(c).unwrap();
                let i = r * w + c;
                union_find.p[i] = i;
                rel[r][c] = i;
            }
        }

        for r in 0..h {
            for c in 0..w {
                if map[r][c] == '.' {
                    continue;
                }
                for i in 0..4 {
                    let r2 = (r as isize + mv[i].0) as usize;
                    let c2 = (c as isize + mv[i].1) as usize;
                    if r2 < h && c2 < w {
                        if map[r2][c2] == map[r][c] {
                            union_find.union(rel[r][c], rel[r2][c2]);
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
                let k = rel[r][c];
                let k = union_find.find(k);
                let i = if tr[k] < 0 {
                    let idx = tree.sz;
                    tr[k] = idx as isize;
                    tree.sz += 1;
                    tree.nodes[idx].xl = -1;
                    idx
                } else {
                    tr[k] as usize
                };
                tree.nodes[i].m += 1;
                tree.nodes[i].mx += c;
                tree.mktree(r, c, i, h, &map, &rel, &tr);
            }
        }
        tree.nodes[1].par = 1;

        println!("{}", if tree.check(1) { "STABLE" } else { "UNSTABLE" });
    }
}
