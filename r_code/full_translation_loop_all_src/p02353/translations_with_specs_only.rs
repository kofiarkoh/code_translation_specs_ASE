use std::io::{self, Read};

const NO_PENDING: i64 = i64::MIN; // Sentinel for "no pending update"; see spec's sentinel assumption.

#[derive(Clone, Copy)]
struct Node {
    sum: i64,
    lazy: i64, // pending assignment value, or NO_PENDING
}

struct SegTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = if n == 0 { 1 } else { 4 * n + 5 };
        Self {
            n,
            tree: vec![
                Node {
                    sum: 0,
                    lazy: NO_PENDING
                };
                size
            ],
        }
    }

    fn apply(&mut self, idx: usize, l: usize, r: usize, v: i64) {
        let len = (r - l + 1) as i64;
        self.tree[idx].sum = v.wrapping_mul(len);
        self.tree[idx].lazy = v;
    }

    fn push(&mut self, idx: usize, l: usize, r: usize) {
        let pending = self.tree[idx].lazy;
        if pending == NO_PENDING || l == r {
            return;
        }
        let mid = (l + r) / 2;
        let left = idx * 2;
        let right = idx * 2 + 1;
        self.apply(left, l, mid, pending);
        self.apply(right, mid + 1, r, pending);
        self.tree[idx].lazy = NO_PENDING;
    }

    fn update_range(&mut self, ql: usize, qr: usize, v: i64) {
        if self.n == 0 {
            return;
        }
        self.update_rec(1, 0, self.n - 1, ql, qr, v);
    }

    fn update_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if ql <= l && r <= qr {
            self.apply(idx, l, r, v);
            return;
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        if ql <= mid {
            self.update_rec(idx * 2, l, mid, ql, qr, v);
        }
        if qr > mid {
            self.update_rec(idx * 2 + 1, mid + 1, r, ql, qr, v);
        }
        self.tree[idx].sum = self.tree[idx * 2]
            .sum
            .wrapping_add(self.tree[idx * 2 + 1].sum);
    }

    fn query_range(&mut self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr {
            return self.tree[idx].sum;
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        let mut res = 0i64;
        if ql <= mid {
            res = res.wrapping_add(self.query_rec(idx * 2, l, mid, ql, qr));
        }
        if qr > mid {
            res = res.wrapping_add(self.query_rec(idx * 2 + 1, mid + 1, r, ql, qr));
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let queries: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut st = SegTree::new(num);
    let mut out = String::new();

    for _ in 0..queries {
        let typ: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let start: usize = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let end: usize = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };

        if typ == 0 {
            let x_i32: i32 = match it.next() {
                Some(s) => s.parse().unwrap(),
                None => break,
            };
            st.update_range(start, end, x_i32 as i64);
        } else if typ == 1 {
            let ans = st.query_range(start, end);
            out.push_str(&ans.to_string());
            out.push('\n');
        } else {
            // Unrecognized operation: ignored (no output, no state change).
            // Spec does not define extra trailing integers for such operations; we consume none.
        }
    }

    print!("{out}");
}
