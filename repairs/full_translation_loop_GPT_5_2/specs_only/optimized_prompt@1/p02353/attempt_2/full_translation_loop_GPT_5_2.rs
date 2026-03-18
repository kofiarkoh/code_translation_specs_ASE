use std::io::{self, Read};

#[derive(Clone, Debug)]
struct SegTree {
    n: usize,
    tree: Vec<i64>, // segment sums
    lazy: Vec<i64>, // pending range-add values
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = 4usize.saturating_mul(n.max(1));
        Self {
            n,
            tree: vec![0i64; size],
            lazy: vec![0i64; size],
        }
    }

    fn push(&mut self, idx: usize, l: usize, r: usize) {
        let pending = self.lazy[idx];
        if pending == 0 || l == r {
            return;
        }
        let mid = (l + r) / 2;
        let left = idx * 2;
        let right = idx * 2 + 1;

        let left_len = (mid - l + 1) as i64;
        let right_len = (r - mid) as i64;

        self.tree[left] += pending * left_len;
        self.lazy[left] += pending;

        self.tree[right] += pending * right_len;
        self.lazy[right] += pending;

        self.lazy[idx] = 0;
    }

    fn range_add(&mut self, ql: usize, qr: usize, val: i64) {
        if self.n == 0 {
            return;
        }
        self.range_add_rec(1, 0, self.n - 1, ql, qr, val);
    }

    fn range_add_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize, val: i64) {
        if ql > r || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            let len = (r - l + 1) as i64;
            self.tree[idx] += val * len;
            self.lazy[idx] += val;
            return;
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        self.range_add_rec(idx * 2, l, mid, ql, qr, val);
        self.range_add_rec(idx * 2 + 1, mid + 1, r, ql, qr, val);
        self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
    }

    fn range_sum(&mut self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 {
            return 0;
        }
        self.range_sum_rec(1, 0, self.n - 1, ql, qr)
    }

    fn range_sum_rec(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql > r || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[idx];
        }
        self.push(idx, l, r);
        let mid = (l + r) / 2;
        self.range_sum_rec(idx * 2, l, mid, ql, qr)
            + self.range_sum_rec(idx * 2 + 1, mid + 1, r, ql, qr)
    }
}

fn main() {
    // The sample indicates a different query format than previously assumed:
    // Each query begins with an operation code:
    //   0 l r   : output sum on [l, r]
    //   1 l r   : range add +1 on [l, r]
    //   2 l r   : range add -1 on [l, r]
    //   3 x     : point add +1 at index x
    //
    // This is the narrowest interpretation consistent with the failing sample:
    // - Query "0 0 4" produced output.
    // - Query "3" is a single integer line, so op=3 must take one argument.
    // - Query "1 1 3" and "2 0 4" have three integers, so ops 1/2 take l r.
    //
    // Indices are treated as 0-based inclusive, matching the sample.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let q: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut st = SegTree::new(n);
    let mut out = String::new();

    for _ in 0..q {
        let op: i32 = it.next().unwrap().parse().unwrap();
        match op {
            0 => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                let l = a.min(b);
                let r = a.max(b);
                let ans = st.range_sum(l, r);
                out.push_str(&ans.to_string());
                out.push('\n');
            }
            1 => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                let l = a.min(b);
                let r = a.max(b);
                st.range_add(l, r, 1);
            }
            2 => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                let l = a.min(b);
                let r = a.max(b);
                st.range_add(l, r, -1);
            }
            3 => {
                let x: usize = it.next().unwrap().parse().unwrap();
                st.range_add(x, x, 1);
            }
            _ => {
                // Behavior for invalid input is undefined by the specification.
                // We choose to do nothing for unknown opcodes.
            }
        }
    }

    print!("{out}");
}
