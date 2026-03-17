use std::io::{self, BufRead};

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    location: usize,
    l: Option<usize>,
    r: Option<usize>,
}

struct KDTree {
    points: Vec<Point>,
    nodes: Vec<Node>,
}

impl KDTree {
    fn new(points: Vec<Point>) -> Self {
        let mut tree = KDTree {
            points,
            nodes: Vec::new(),
        };
        tree.make_kd_tree(0, tree.points.len(), 0);
        tree
    }

    fn make_kd_tree(&mut self, l: usize, r: usize, depth: usize) {
        if l >= r {
            return;
        }

        let mid = (l + r) / 2;
        if depth % 2 == 0 {
            self.points[l..r].sort_by(|a, b| a.x.cmp(&b.x));
        } else {
            self.points[l..r].sort_by(|a, b| a.y.cmp(&b.y));
        }

        let node = Node {
            location: mid,
            l: if mid > l { Some(self.nodes.len() + 1) } else { None },
            r: if mid + 1 < r { Some(self.nodes.len() + 2) } else { None },
        };
        self.nodes.push(node);

        self.make_kd_tree(l, mid, depth + 1);
        self.make_kd_tree(mid + 1, r, depth + 1);
    }

    fn find(&self, v: usize, sx: i32, tx: i32, sy: i32, ty: i32, depth: usize, pid: &mut Vec<usize>) {
        let x = self.points[self.nodes[v].location].x;
        let y = self.points[self.nodes[v].location].y;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            pid.push(self.points[self.nodes[v].location].id);
        }

        if depth % 2 == 0 {
            if let Some(left) = self.nodes[v].l {
                if sx <= x {
                    self.find(left, sx, tx, sy, ty, depth + 1, pid);
                }
            }
            if let Some(right) = self.nodes[v].r {
                if x < tx { // Changed from x <= tx to x < tx
                    self.find(right, sx, tx, sy, ty, depth + 1, pid);
                }
            }
        } else {
            if let Some(left) = self.nodes[v].l {
                if sy <= y {
                    self.find(left, sx, tx, sy, ty, depth + 1, pid);
                }
            }
            if let Some(right) = self.nodes[v].r {
                if y < ty { // Changed from y <= ty to y < ty
                    self.find(right, sx, tx, sy, ty, depth + 1, pid);
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
        let query: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (sx, tx, sy, ty) = (query[0], query[1], query[2], query[3]);

        let mut pid = Vec::new();
        kd_tree.find(0, sx, tx, sy, ty, 0, &mut pid);
        pid.sort_unstable();

        for id in pid {
            println!("{}", id);
        }
        println!(); // Blank line after each query's results
    }
}
