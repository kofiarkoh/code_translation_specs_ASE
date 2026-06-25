use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Node {
    min_pref: i64, // minimum prefix sum in this segment
    max_pref: i64, // maximum prefix sum in this segment
    best: i64,     // maximum difference P[b]-P[a] with a<b within this segment
}

impl Node {
    fn leaf(v: i64) -> Self {
        // Segment of length 1: only one prefix value, so no valid a<b inside.
        Node {
            min_pref: v,
            max_pref: v,
            best: 0,
        }
    }

    fn combine(left: Node, right: Node) -> Self {
        // For concatenated segment:
        // - min/max are straightforward.
        // - best is max of:
        //   * best entirely in left
        //   * best entirely in right
        //   * best crossing: max in right - min in left (a in left, b in right)
        let min_pref = left.min_pref.min(right.min_pref);
        let max_pref = left.max_pref.max(right.max_pref);
        let cross = right.max_pref - left.min_pref;
        let best = left.best.max(right.best).max(cross);
        Node {
            min_pref,
            max_pref,
            best,
        }
    }
}

struct SegTree {
    n: usize,        // number of leaves (prefix positions), equals n_input + 1
    size: usize,     // power-of-two >= n
    tree: Vec<Node>, // 1-indexed segment tree nodes
}

impl SegTree {
    fn new(values: &[i64]) -> Self {
        let n = values.len();
        let mut size = 1usize;
        while size < n {
            size <<= 1;
        }

        // Initialize all nodes as neutral leaves with value 0.
        // This is safe because we will build by setting actual leaves and combining.
        let mut tree = vec![Node::leaf(0); 2 * size];

        // Set leaves
        for i in 0..size {
            let v = if i < n { values[i] } else { 0 };
            tree[size + i] = Node::leaf(v);
        }

        // Build
        for idx in (1..size).rev() {
            tree[idx] = Node::combine(tree[idx << 1], tree[(idx << 1) | 1]);
        }

        SegTree { n, size, tree }
    }

    fn set(&mut self, pos: usize, val: i64) {
        // pos is 0-based within [0, n)
        let mut idx = self.size + pos;
        self.tree[idx] = Node::leaf(val);
        idx >>= 1;
        while idx >= 1 {
            self.tree[idx] = Node::combine(self.tree[idx << 1], self.tree[(idx << 1) | 1]);
            if idx == 1 {
                break;
            }
            idx >>= 1;
        }
    }

    fn best(&self) -> i64 {
        // Root best is the maximum subarray sum with empty subarray allowed (>=0).
        self.tree[1].best
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut a = vec![0i64; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // Build prefix sums P[0..n], P[0]=0, P[k]=sum_{j<k} a[j]
    let mut p = vec![0i64; n + 1];
    for k in 1..=n {
        p[k] = p[k - 1] + a[k - 1];
    }

    let mut st = SegTree::new(&p);

    let mut out = String::new();
    out.push_str(&format!("{}\n", st.best()));

    for _ in 0..q {
        let i1: usize = it.next().unwrap().parse().unwrap(); // 1-based
        let t: i64 = it.next().unwrap().parse().unwrap();
        let idx = i1 - 1; // 0-based in a

        let old = a[idx];
        a[idx] = t;
        let delta = t - old;

        // Update all prefix sums P[k] for k >= i1 (since P uses count of first k elements)
        // i1 is 1-based element index, so affected prefix indices are k in [i1..=n].
        for k in i1..=n {
            p[k] += delta;
            st.set(k, p[k]);
        }

        out.push_str(&format!("{}\n", st.best()));
    }

    print!("{out}");
}
