use std::io::{self, Read};

const NO_PENDING: i64 = i64::MIN; // Sentinel for "no pending update"; see spec's sentinel assumption.

#[derive(Clone, Copy)]
struct Node {
    sum: i64,  // Sum over the segment
    lazy: i64, // If != NO_PENDING, represents a pending assignment of the whole segment to a value v
}

struct SegTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        // 4*n is a common safe bound; handle n=0 by still allocating a small vector.
        let size = if n == 0 { 1 } else { 4 * n };
        let tree = vec![
            Node {
                sum: 0,
                lazy: NO_PENDING
            };
            size
        ];
        Self { n, tree }
    }

    fn apply(&mut self, idx: usize, l: usize, r: usize, v: i64) {
        let len = (r - l + 1) as i64;
        // Signed 64-bit arithmetic; overflow is intentionally unchecked (wraps in release).
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
        self.update_range_rec(1, 0, self.n - 1, ql, qr, v);
    }

    fn update_range_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize, v: i64) {
        if ql <= l && r <= qr {
            self.apply(idx, l, r, v);
            return;
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        if ql <= mid {
            self.update_range_rec(idx * 2, l, mid, ql, qr, v);
        }
        if qr > mid {
            self.update_range_rec(idx * 2 + 1, mid + 1, r, ql, qr, v);
        }
        self.tree[idx].sum = self.tree[idx * 2]
            .sum
            .wrapping_add(self.tree[idx * 2 + 1].sum);
    }

    fn query_range(&mut self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.query_range_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_range_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr {
            return self.tree[idx].sum;
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        let mut res: i64 = 0;
        if ql <= mid {
            res = res.wrapping_add(self.query_range_rec(idx * 2, l, mid, ql, qr));
        }
        if qr > mid {
            res = res.wrapping_add(self.query_range_rec(idx * 2 + 1, mid + 1, r, ql, qr));
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
        let t_opt = it.next();
        if t_opt.is_none() {
            break; // Input is assumed well-formed by spec; this is a minimal guard.
        }
        let typ: i32 = t_opt.unwrap().parse().unwrap();
        let start: usize = it.next().unwrap().parse().unwrap();
        let end: usize = it.next().unwrap().parse().unwrap();

        if typ == 0 {
            let x_i32: i32 = it.next().unwrap().parse().unwrap();
            let x = x_i32 as i64;
            st.update_range(start, end, x);
        } else if typ == 1 {
            let ans = st.query_range(start, end);
            out.push_str(&format!("{}\n", ans));
        } else {
            // Unrecognized operation: ignored (no output, no state change).
        }
    }

    print!("{}", out);
}
