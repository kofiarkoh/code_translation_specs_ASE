use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Sayouso {
    k: i64,
}

#[derive(Clone, Copy)]
struct Atai {
    m: i64,
    M: i64,
    x: i64,
}

#[derive(Clone)] // Added Clone derive for Node
struct Node {
    T: Sayouso,
    x: Atai,
}

struct LazySegmentTree {
    nodes: Vec<Node>,
    size: usize,
}

impl LazySegmentTree {
    fn new(n: usize) -> Self {
        let mut nodes = vec![Node { T: Sayouso { k: 0 }, x: Atai { m: i64::MAX, M: i64::MIN, x: 0 } }; 2 * n];
        let size = n;

        for i in 0..size {
            nodes[size + i].x = Atai { m: 0, M: 0, x: 0 }; // Initialize leaves
        }

        for i in (1..size).rev() {
            nodes[i].x = Self::combine(nodes[2 * i].x, nodes[2 * i + 1].x);
            nodes[i].T = Sayouso { k: 0 };
        }

        LazySegmentTree { nodes, size }
    }

    fn combine(x: Atai, y: Atai) -> Atai {
        Atai {
            m: x.m.min(y.m),
            M: x.M.max(y.M),
            x: (y.M - x.m).max(x.x).max(y.x),
        }
    }

    fn apply_transform(T: Sayouso, x: Atai) -> Atai {
        Atai {
            m: x.m + T.k,
            M: x.M + T.k,
            x: x.x,
        }
    }

    fn combine_transform(S: Sayouso, T: Sayouso) -> Sayouso {
        Sayouso { k: S.k + T.k }
    }

    fn update_range(&mut self, l: usize, r: usize, T: Sayouso, i: usize, cl: usize, cr: usize) {
        if l >= r {
            return;
        }
        if cr <= l || r <= cl {
            return;
        }
        if l <= cl && cr <= r {
            self.nodes[i].T = Self::combine_transform(T, self.nodes[i].T);
            return;
        }

        self.nodes[2 * i].T = Self::combine_transform(self.nodes[i].T, self.nodes[2 * i].T);
        self.nodes[2 * i + 1].T = Self::combine_transform(self.nodes[i].T, self.nodes[2 * i + 1].T);

        let cm = (cl + cr) / 2;
        self.update_range(l, r, T, 2 * i, cl, cm);
        self.update_range(l, r, T, 2 * i + 1, cm, cr);

        self.nodes[i].x = Self::combine(
            Self::apply_transform(self.nodes[2 * i].T, self.nodes[2 * i].x),
            Self::apply_transform(self.nodes[2 * i + 1].T, self.nodes[2 * i + 1].x),
        );
        self.nodes[i].T = Sayouso { k: 0 };
    }

    fn query_range(&mut self, l: usize, r: usize, i: usize, cl: usize, cr: usize) -> Atai {
        if l >= r {
            return Atai { m: i64::MAX, M: i64::MIN, x: 0 };
        }
        if l <= cl && cr <= r {
            return Self::apply_transform(self.nodes[i].T, self.nodes[i].x);
        }

        let cm = (cl + cr) / 2;

        self.nodes[2 * i].T = Self::combine_transform(self.nodes[i].T, self.nodes[2 * i].T);
        self.nodes[2 * i + 1].T = Self::combine_transform(self.nodes[i].T, self.nodes[2 * i + 1].T);
        self.nodes[i].x = Self::apply_transform(self.nodes[i].T, self.nodes[i].x);
        self.nodes[i].T = Sayouso { k: 0 };

        if r <= cm {
            self.query_range(l, r, 2 * i, cl, cm)
        } else if cm <= l {
            self.query_range(l, r, 2 * i + 1, cm, cr)
        } else {
            Self::combine(
                self.query_range(l, r, 2 * i, cl, cm),
                self.query_range(l, r, 2 * i + 1, cm, cr),
            )
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut segment_tree = LazySegmentTree::new(n);
    let mut s = 0;

    for i in 0..n {
        let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        s += t;
        segment_tree.nodes[i + segment_tree.size].x = Atai { m: s, M: s, x: 0 };
    }

    for i in (1..segment_tree.size).rev() {
        segment_tree.nodes[i].x = LazySegmentTree::combine(segment_tree.nodes[2 * i].x, segment_tree.nodes[2 * i + 1].x);
    }

    println!("{}", segment_tree.query_range(0, n + 1, 1, 0, segment_tree.size).x);

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let i: usize = parts.next().unwrap().parse().unwrap();
        let t: i64 = parts.next().unwrap().parse().unwrap();

        let T = Sayouso { k: t - segment_tree.query_range(i, i + 1, 1, 0, segment_tree.size).m + segment_tree.query_range(i - 1, i, 1, 0, segment_tree.size).m };
        segment_tree.update_range(i, n + 1, T, 1, 0, segment_tree.size);
        println!("{}", segment_tree.query_range(0, n + 1, 1, 0, segment_tree.size).x);
    }
}
