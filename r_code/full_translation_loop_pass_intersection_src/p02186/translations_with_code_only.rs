use std::cmp::{max, min};
use std::io::{self, Read};

type Ll = i64;

const INF: Ll = ((1i64 << 62) - (1i64 << 31));

#[derive(Clone, Copy, Debug)]
struct Sayouso {
    k: Ll,
}
#[derive(Clone, Copy, Debug)]
struct Atai {
    m: Ll,
    mmax: Ll, // corresponds to C field `M`
    x: Ll,
}
#[derive(Clone, Copy, Debug)]
struct Node {
    t: Sayouso,
    x: Atai,
}

const SID: Sayouso = Sayouso { k: 0 };
const AID: Atai = Atai {
    m: INF,
    mmax: -INF,
    x: 0,
};

fn xx(a: Atai, b: Atai) -> Atai {
    // ret.m = min(x.m, y.m)
    // ret.M = max(x.M, y.M)
    // ret.x = max(y.M - x.m, max(x.x, y.x))
    let m = min(a.m, b.m);
    let mmax = max(a.mmax, b.mmax);
    let x = max(b.mmax - a.m, max(a.x, b.x));
    Atai { m, mmax, x }
}

fn tx(t: Sayouso, a: Atai) -> Atai {
    // add t.k to m and M, keep x
    Atai {
        m: a.m + t.k,
        mmax: a.mmax + t.k,
        x: a.x,
    }
}

fn tt(s: Sayouso, t: Sayouso) -> Sayouso {
    // compose: S then T in the C code's order (just addition)
    Sayouso { k: s.k + t.k }
}

struct LazySeg {
    n: usize,        // power-of-two size
    nodes: Vec<Node>, // 0..2*n-1, we use indices like the C code (1-based internal)
}

impl LazySeg {
    fn new(n: usize) -> Self {
        // C: calloc(2*n, sizeof(node)) then sets leaves [n..2n) to aid/sid,
        // and builds internal nodes.
        let mut nodes = vec![
            Node {
                t: SID,
                x: AID
            };
            2 * n
        ];
        // leaves are nodes[n..2n)
        for i in 0..n {
            nodes[n + i].x = AID;
            nodes[n + i].t = SID;
        }
        for i in (1..n).rev() {
            let left = nodes[2 * i].x;
            let right = nodes[2 * i + 1].x;
            nodes[i].x = xx(left, right);
            nodes[i].t = SID;
        }
        Self { n, nodes }
    }

    fn init(&mut self) {
        // C: for i=n-1..1: x = xx(children)
        for i in (1..self.n).rev() {
            let left = self.nodes[2 * i].x;
            let right = self.nodes[2 * i + 1].x;
            self.nodes[i].x = xx(left, right);
        }
    }

    fn update(&mut self, l: usize, r: usize, t: Sayouso) {
        self.update_sub(l, r, t, 1, 0, self.n);
    }

    fn update_sub(&mut self, l: usize, r: usize, t: Sayouso, i: usize, cl: usize, cr: usize) {
        if l >= r {
            return;
        }
        if cr <= l || r <= cl {
            return;
        }
        if l <= cl && cr <= r {
            self.nodes[i].t = tt(t, self.nodes[i].t);
            return;
        }

        // lazy propagation
        let cur_t = self.nodes[i].t;
        let li = 2 * i;
        let ri = 2 * i + 1;
        self.nodes[li].t = tt(cur_t, self.nodes[li].t);
        self.nodes[ri].t = tt(cur_t, self.nodes[ri].t);

        let cm = (cl + cr) / 2;
        self.update_sub(l, r, t, li, cl, cm);
        self.update_sub(l, r, t, ri, cm, cr);

        // update self from children (applying their pending tags to their x)
        let left = tx(self.nodes[li].t, self.nodes[li].x);
        let right = tx(self.nodes[ri].t, self.nodes[ri].x);
        self.nodes[i].x = xx(left, right);
        self.nodes[i].t = SID;
    }

    fn calc(&mut self, l: usize, r: usize) -> Atai {
        self.calc_sub(l, r, 1, 0, self.n)
    }

    fn calc_sub(&mut self, l: usize, r: usize, i: usize, cl: usize, cr: usize) -> Atai {
        if l >= r {
            return AID;
        }
        if l <= cl && cr <= r {
            return tx(self.nodes[i].t, self.nodes[i].x);
        }

        let cm = (cl + cr) / 2;

        // lazy propagation (C mutates the tree during query)
        let cur_t = self.nodes[i].t;
        let li = 2 * i;
        let ri = 2 * i + 1;
        self.nodes[li].t = tt(cur_t, self.nodes[li].t);
        self.nodes[ri].t = tt(cur_t, self.nodes[ri].t);
        self.nodes[i].x = tx(cur_t, self.nodes[i].x);
        self.nodes[i].t = SID;

        if r <= cm {
            return self.calc_sub(l, r, li, cl, cm);
        }
        if cm <= l {
            return self.calc_sub(l, r, ri, cm, cr);
        }
        let a = self.calc_sub(l, r, li, cl, cm);
        let b = self.calc_sub(l, r, ri, cm, cr);
        xx(a, b)
    }
}

fn main() {
    // Read all input, parse as integers like scanf would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let q: i32 = it.next().unwrap().parse().unwrap();

    let mut seg = LazySeg::new(1 << 17);

    // lseg[0].x = {0,0,0} where lseg points to leaves
    // leaves start at index seg.n
    seg.nodes[seg.n + 0].x = Atai { m: 0, mmax: 0, x: 0 };

    let mut s: Ll = 0;
    for i in 0..n {
        let t: Ll = it.next().unwrap().parse().unwrap();
        s += t;
        seg.nodes[seg.n + (i + 1)].x = Atai {
            m: s,
            mmax: s,
            x: 0,
        };
    }

    seg.init();

    let mut out = String::new();
    out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));

    let mut qq = q;
    while qq > 0 {
        qq -= 1;
        let i_in: Ll = it.next().unwrap().parse().unwrap();
        let t: Ll = it.next().unwrap().parse().unwrap();

        // C uses ll i and calls lsegcalc(i-1,i) even when i==0 (negative index),
        // which is undefined behavior. We cannot reproduce UB in safe Rust.
        // We emulate the most plausible intent by clamping to 0 for the left query.
        let i = if i_in < 0 { 0usize } else { i_in as usize };

        let cur_m = seg.calc(i, i + 1).m;
        let prev_m = if i == 0 {
            seg.calc(0, 0).m // empty range -> AID.m == INF, consistent with our calc
        } else {
            seg.calc(i - 1, i).m
        };

        let tag = Sayouso { k: t - cur_m + prev_m };
        seg.update(i, n + 1, tag);
        out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));
    }

    print!("{out}");
}
