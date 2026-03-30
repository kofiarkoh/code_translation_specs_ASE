use std::cmp::Ordering;
use std::io::{self, BufRead};

const MAX_N: usize = 1_000_000;
const NIL: i32 = -1;

#[derive(Debug, Copy, Clone)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Node {
    location: usize,
    l: i32,
    r: i32,
}

struct KDTree {
    p: [Point; MAX_N],
    t: [Node; MAX_N],
    pid: [i32; MAX_N],
    n: usize,
    np: usize,
    n_pid: usize,
}

impl KDTree {
    fn new() -> Self {
        KDTree {
            p: [Point { id: 0, x: 0, y: 0 }; MAX_N],
            t: [Node { location: 0, l: NIL, r: NIL }; MAX_N],
            pid: [0; MAX_N],
            n: 0,
            np: 0,
            n_pid: 0,
        }
    }

    fn cmpx_func(l: &Point, r: &Point) -> Ordering {
        l.x.cmp(&r.x)
    }

    fn cmpy_func(l: &Point, r: &Point) -> Ordering {
        l.y.cmp(&r.y)
    }

    fn make_kd_tree(&mut self, l: usize, r: usize, depth: usize) -> i32 {
        if l >= r {
            return NIL;
        }

        if r - l >= 2 {
            if depth % 2 == 0 {
                self.p[l..r].sort_unstable_by(KDTree::cmpx_func);
            } else {
                self.p[l..r].sort_unstable_by(KDTree::cmpy_func);
            }
        }

        let mid = (l + r) / 2;
        let s = self.np;
        self.np += 1;
        self.t[s].location = mid;
        self.t[s].l = self.make_kd_tree(l, mid, depth + 1);
        self.t[s].r = self.make_kd_tree(mid + 1, r, depth + 1);
        s as i32
    }

    fn find(&mut self, v: usize, sx: i32, tx: i32, sy: i32, ty: i32, depth: usize) {
        let x = self.p[self.t[v].location].x;
        let y = self.p[self.t[v].location].y;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            self.pid[self.n_pid] = self.p[self.t[v].location].id as i32;
            self.n_pid += 1;
        }

        if depth % 2 == 0 {
            if self.t[v].l != NIL {
                if sx <= x {
                    self.find(self.t[v].l as usize, sx, tx, sy, ty, depth + 1);
                }
            }
            if self.t[v].r != NIL {
                if x <= tx {
                    self.find(self.t[v].r as usize, sx, tx, sy, ty, depth + 1);
                }
            }
        } else {
            if self.t[v].l != NIL {
                if sy <= y {
                    self.find(self.t[v].l as usize, sx, tx, sy, ty, depth + 1);
                }
            }
            if self.t[v].r != NIL {
                if y <= ty {
                    self.find(self.t[v].r as usize, sx, tx, sy, ty, depth + 1);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut kd_tree = KDTree::new();
    kd_tree.n = lines.next().unwrap().unwrap().parse().unwrap();
    
    for i in 0..kd_tree.n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        kd_tree.p[i] = Point { id: i, x, y };
        kd_tree.t[i].l = NIL;
        kd_tree.t[i].r = NIL;
    }

    kd_tree.np = 0;
    let root = kd_tree.make_kd_tree(0, kd_tree.n, 0);

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut bounds = line.split_whitespace();
        let sx: i32 = bounds.next().unwrap().parse().unwrap();
        let tx: i32 = bounds.next().unwrap().parse().unwrap();
        let sy: i32 = bounds.next().unwrap().parse().unwrap();
        let ty: i32 = bounds.next().unwrap().parse().unwrap();
        kd_tree.n_pid = 0;
        kd_tree.find(root as usize, sx, tx, sy, ty, 0);

        let pid_slice = &mut kd_tree.pid[..kd_tree.n_pid];
        pid_slice.sort_unstable();

        for &id in pid_slice {
            println!("{}", id);
        }
        println!();
    }
}
