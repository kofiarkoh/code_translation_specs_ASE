use std::io::{self, Read};

const INF: i64 = ((1i64 << 62) - (1i64 << 31));

#[derive(Clone, Copy, Debug)]
struct Lazy {
    k: i64,
}
const SID: Lazy = Lazy { k: 0 };

#[derive(Clone, Copy, Debug)]
struct Val {
    m: i64, // minimum prefix in segment
    M: i64, // maximum prefix in segment
    x: i64, // maximum difference (max subarray sum) within segment
}
const AID: Val = Val {
    m: INF,
    M: -INF,
    x: 0,
};

#[derive(Clone, Copy, Debug)]
struct Node {
    t: Lazy,
    x: Val,
}

fn combine(a: Val, b: Val) -> Val {
    // Merge two segments: min, max, and best cross difference.
    let m = a.m.min(b.m);
    let M = a.M.max(b.M);
    let x = (b.M - a.m).max(a.x.max(b.x));
    Val { m, M, x }
}

fn apply(t: Lazy, v: Val) -> Val {
    // Add t.k to all prefix sums in this segment; x unchanged.
    Val {
        m: v.m + t.k,
        M: v.M + t.k,
        x: v.x,
    }
}

fn compose(a: Lazy, b: Lazy) -> Lazy {
    // Apply a then b (both are additions), so add.
    Lazy { k: a.k + b.k }
}

struct LazySegTree {
    n: usize,        // power-of-two size
    nodes: Vec<Node>, // 1-indexed tree nodes, size 2*n
}

impl LazySegTree {
    fn new(n_pow2: usize) -> Self {
        let mut nodes = vec![
            Node {
                t: SID,
                x: AID
            };
            2 * n_pow2
        ];
        // Initialize internal nodes from leaves (all AID initially).
        for i in (1..n_pow2).rev() {
            nodes[i].x = combine(nodes[2 * i].x, nodes[2 * i + 1].x);
            nodes[i].t = SID;
        }
        Self { n: n_pow2, nodes }
    }

    fn set_leaf(&mut self, idx: usize, v: Val) {
        self.nodes[self.n + idx].x = v;
    }

    fn init(&mut self) {
        for i in (1..self.n).rev() {
            self.nodes[i].x = combine(self.nodes[2 * i].x, self.nodes[2 * i + 1].x);
            self.nodes[i].t = SID;
        }
    }

    fn push(&mut self, i: usize) {
        let t = self.nodes[i].t;
        if t.k != 0 {
            let l = 2 * i;
            let r = 2 * i + 1;
            self.nodes[l].t = compose(t, self.nodes[l].t);
            self.nodes[r].t = compose(t, self.nodes[r].t);
            self.nodes[i].x = apply(t, self.nodes[i].x);
            self.nodes[i].t = SID;
        }
    }

    fn update_range(&mut self, l: usize, r: usize, t: Lazy) {
        self.update_sub(l, r, t, 1, 0, self.n);
    }

    fn update_sub(&mut self, l: usize, r: usize, t: Lazy, i: usize, cl: usize, cr: usize) {
        if l >= r || cr <= l || r <= cl {
            return;
        }
        if l <= cl && cr <= r {
            self.nodes[i].t = compose(t, self.nodes[i].t);
            return;
        }
        // propagate pending tag before going down
        self.push(i);
        let cm = (cl + cr) / 2;
        self.update_sub(l, r, t, 2 * i, cl, cm);
        self.update_sub(l, r, t, 2 * i + 1, cm, cr);
        let left = apply(self.nodes[2 * i].t, self.nodes[2 * i].x);
        let right = apply(self.nodes[2 * i + 1].t, self.nodes[2 * i + 1].x);
        self.nodes[i].x = combine(left, right);
        self.nodes[i].t = SID;
    }

    fn query_range(&mut self, l: usize, r: usize) -> Val {
        self.query_sub(l, r, 1, 0, self.n)
    }

    fn query_sub(&mut self, l: usize, r: usize, i: usize, cl: usize, cr: usize) -> Val {
        if l >= r {
            return AID;
        }
        if l <= cl && cr <= r {
            return apply(self.nodes[i].t, self.nodes[i].x);
        }
        self.push(i);
        let cm = (cl + cr) / 2;
        if r <= cm {
            return self.query_sub(l, r, 2 * i, cl, cm);
        }
        if cm <= l {
            return self.query_sub(l, r, 2 * i + 1, cm, cr);
        }
        let a = self.query_sub(l, r, 2 * i, cl, cm);
        let b = self.query_sub(l, r, 2 * i + 1, cm, cr);
        combine(a, b)
    }
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity as in C: 1<<17 (131072)
    // Spec precondition ensures n+1 fits.
    let mut seg = LazySegTree::new(1 << 17);

    // Build prefix sums P[0..n], stored as leaves 0..n (length n+1).
    // Leaves beyond n are left as AID and never queried/updated.
    seg.set_leaf(0, Val { m: 0, M: 0, x: 0 });
    let mut pref: i64 = 0;
    for i in 0..n {
        let a: i64 = it.next().unwrap().parse().unwrap();
        pref += a;
        seg.set_leaf(i + 1, Val { m: pref, M: pref, x: 0 });
    }
    seg.init();

    let mut out = String::new();
    out.push_str(&format!("{}\n", seg.query_range(0, n + 1).x));

    for _ in 0..q {
        let idx1: usize = it.next().unwrap().parse().unwrap(); // 1-based
        let t_new: i64 = it.next().unwrap().parse().unwrap();

        // Compute delta to add to all prefix sums P[k] for k >= idx1.
        // Using the same formula as C:
        // delta = new_value - old_value, where old_value = P[i]-P[i-1]
        let p_i = seg.query_range(idx1, idx1 + 1).m;
        let p_im1 = seg.query_range(idx1 - 1, idx1).m;
        let old_val = p_i - p_im1;
        let delta = t_new - old_val;

        seg.update_range(idx1, n + 1, Lazy { k: delta });
        out.push_str(&format!("{}\n", seg.query_range(0, n + 1).x));
    }

    print!("{out}");
}
