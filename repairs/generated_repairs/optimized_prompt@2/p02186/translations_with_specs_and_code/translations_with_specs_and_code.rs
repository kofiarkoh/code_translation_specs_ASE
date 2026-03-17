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

#[derive(Clone)]
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
        let size = 1 << (32 - (n as f64).log(2.0).ceil() as usize);
        let mut nodes = vec![Node {
            T: Sayouso { k: 0 },
            x: Atai { m: i64::MAX, M: i64::MIN, x: 0 },
        }; 2 * size];

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

    fn combine_transforms(S: Sayouso, T: Sayouso) -> Sayouso {
        Sayouso { k: S.k + T.k }
    }

    fn update_range(&mut self, l: usize, r: usize, T: Sayouso) {
        self.update_range_sub(l, r, T, 1, 0, self.size);
    }

    fn update_range_sub(&mut self, l: usize, r: usize, T: Sayouso, i: usize, cl: usize, cr: usize) {
        if l >= r {
            return;
        }
        if cr <= l || r <= cl {
            return;
        }
        if l <= cl && cr <= r {
            self.nodes[i].T = Self::combine_transforms(T, self.nodes[i].T);
            return;
        }

        self.nodes[2 * i].T = Self::combine_transforms(self.nodes[i].T, self.nodes[2 * i].T);
        self.nodes[2 * i + 1].T = Self::combine_transforms(self.nodes[i].T, self.nodes[2 * i + 1].T);

        let cm = (cl + cr) / 2;
        self.update_range_sub(l, r, T, 2 * i, cl, cm);
        self.update_range_sub(l, r, T, 2 * i + 1, cm, cr);

        self.nodes[i].x = Self::combine(
            Self::apply_transform(self.nodes[2 * i].T, self.nodes[2 * i].x),
            Self::apply_transform(self.nodes[2 * i + 1].T, self.nodes[2 * i + 1].x),
        );
        self.nodes[i].T = Sayouso { k: 0 };
    }

    fn query_range(&mut self, l: usize, r: usize) -> Atai {
        self.query_range_sub(l, r, 1, 0, self.size)
    }

    fn query_range_sub(&mut self, l: usize, r: usize, i: usize, cl: usize, cr: usize) -> Atai {
        if l >= r {
            return Atai { m: i64::MAX, M: i64::MIN, x: 0 };
        }
        if l <= cl && cr <= r {
            return Self::apply_transform(self.nodes[i].T, self.nodes[i].x);
        }

        let cm = (cl + cr) / 2;

        self.nodes[2 * i].T = Self::combine_transforms(self.nodes[i].T, self.nodes[2 * i].T);
        self.nodes[2 * i + 1].T = Self::combine_transforms(self.nodes[i].T, self.nodes[2 * i + 1].T);
        self.nodes[i].x = Self::apply_transform(self.nodes[i].T, self.nodes[i].x);
        self.nodes[i].T = Sayouso { k: 0 };

        let left_result = self.query_range_sub(l, r, 2 * i, cl, cm);
        let right_result = self.query_range_sub(l, r, 2 * i + 1, cm, cr);
        Self::combine(left_result, right_result)
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
        segment_tree.nodes[i + 1].x = Atai { m: s, M: s, x: 0 };
    }

    for i in (1..segment_tree.size).rev() {
        segment_tree.nodes[i].x = LazySegmentTree::combine(segment_tree.nodes[2 * i].x, segment_tree.nodes[2 * i + 1].x);
    }

    println!("{}", segment_tree.query_range(0, n + 1).x);

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_parts = query_line.split_whitespace();
        let i: usize = query_parts.next().unwrap().parse().unwrap();
        let t: i64 = query_parts.next().unwrap().parse().unwrap();

        let current_max = segment_tree.query_range(i, i + 1).m;
        let previous_max = segment_tree.query_range(i.wrapping_sub(1), i).m;
        let T = Sayouso { k: t - current_max + previous_max };
        segment_tree.update_range(i, n + 1, T);
        println!("{}", segment_tree.query_range(0, n + 1).x);
    }
}
