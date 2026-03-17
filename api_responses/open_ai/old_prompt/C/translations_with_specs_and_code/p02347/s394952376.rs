use std::cmp::Ordering;
use std::io::{self, BufRead};

const MAX_N: usize = 1_000_000;
const NIL: i32 = -1;

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    location: usize,
    l: i32,
    r: i32,
}

struct KDTree {
    points: Vec<Point>,
    nodes: Vec<Node>,
    np: usize,
}

impl KDTree {
    fn new(points: Vec<Point>) -> Self {
        let mut tree = KDTree {
            points,
            nodes: Vec::new(),
            np: 0,
        };
        let root = tree.make_kd_tree(0, tree.points.len(), 0);
        tree.nodes.push(Node {
            location: root,
            l: NIL,
            r: NIL,
        });
        tree
    }

    fn make_kd_tree(&mut self, l: usize, r: usize, depth: usize) -> usize {
        if l >= r {
            return NIL as usize;
        }

        if r - l >= 2 {
            if depth % 2 == 0 {
                self.points[l..r].sort_by(|a, b| a.x.cmp(&b.x));
            } else {
                self.points[l..r].sort_by(|a, b| a.y.cmp(&b.y));
            }
        }

        let mid = (l + r) / 2;
        let s = self.np;
        self.np += 1;

        self.nodes.push(Node {
            location: mid,
            l: self.make_kd_tree(l, mid, depth + 1) as i32,
            r: self.make_kd_tree(mid + 1, r, depth + 1) as i32,
        });

        s
    }

    fn find(&self, v: usize, sx: i32, tx: i32, sy: i32, ty: i32, depth: usize, pid: &mut Vec<usize>) {
        let x = self.points[self.nodes[v].location].x;
        let y = self.points[self.nodes[v].location].y;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            pid.push(self.points[self.nodes[v].location].id);
        }

        if depth % 2 == 0 {
            if self.nodes[v].l != NIL {
                if sx <= x {
                    self.find(self.nodes[v].l as usize, sx, tx, sy, ty, depth + 1, pid);
                }
            }
            if self.nodes[v].r != NIL {
                if x <= tx {
                    self.find(self.nodes[v].r as usize, sx, tx, sy, ty, depth + 1, pid);
                }
            }
        } else {
            if self.nodes[v].l != NIL {
                if sy <= y {
                    self.find(self.nodes[v].l as usize, sx, tx, sy, ty, depth + 1, pid);
                }
            }
            if self.nodes[v].r != NIL {
                if y <= ty {
                    self.find(self.nodes[v].r as usize, sx, tx, sy, ty, depth + 1, pid);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        points.push(Point { id: i, x: coords[0], y: coords[1] });
    }

    let kd_tree = KDTree::new(points);

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let bounds: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (sx, tx, sy, ty) = (bounds[0], bounds[1], bounds[2], bounds[3]);

        let mut pid = Vec::new();
        kd_tree.find(0, sx, tx, sy, ty, 0, &mut pid);
        pid.sort_unstable();

        for id in pid {
            println!("{}", id);
        }
        println!(); // Blank line after each query's results
    }
}
