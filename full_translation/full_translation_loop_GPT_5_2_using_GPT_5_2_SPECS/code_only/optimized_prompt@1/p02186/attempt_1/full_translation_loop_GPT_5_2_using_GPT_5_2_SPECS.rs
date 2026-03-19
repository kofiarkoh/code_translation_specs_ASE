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
    // ret.m = min(a.m, b.m)
    // ret.M = max(a.M, b.M)
    // ret.x = max(b.M - a.m, max(a.x, b.x))
    Atai {
        m: min(a.m, b.m),
        mmax: max(a.mmax, b.mmax),
        x: max(b.mmax - a.m, max(a.x, b.x)),
    }
}

fn tx(t: Sayouso, a: Atai) -> Atai {
    Atai {
        m: a.m + t.k,
        mmax: a.mmax + t.k,
        x: a.x,
    }
}

fn tt(s: Sayouso, t: Sayouso) -> Sayouso {
    Sayouso { k: s.k + t.k }
}

struct LazySeg {
    n: usize,        // power of two
    nodes: Vec<Node>, // 0..2*n
}

impl LazySeg {
    fn new(n: usize) -> Self {
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
            nodes[i].x = xx(nodes[2 * i].x, nodes[2 * i + 1].x);
            nodes[i].t = SID;
        }
        Self { n, nodes }
    }

    fn leaf_mut(&mut self, idx: usize) -> &mut Node {
        &mut self.nodes[self.n + idx]
    }

    fn init(&mut self) {
        for i in (1..self.n).rev() {
            self.nodes[i].x = xx(self.nodes[2 * i].x, self.nodes[2 * i + 1].x);
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

        // push lazy to children
        let lazy = self.nodes[i].t;
        self.nodes[2 * i].t = tt(lazy, self.nodes[2 * i].t);
        self.nodes[2 * i + 1].t = tt(lazy, self.nodes[2 * i + 1].t);

        let cm = (cl + cr) / 2;
        self.update_sub(l, r, t, 2 * i, cl, cm);
        self.update_sub(l, r, t, 2 * i + 1, cm, cr);

        let left = tx(self.nodes[2 * i].t, self.nodes[2 * i].x);
        let right = tx(self.nodes[2 * i + 1].t, self.nodes[2 * i + 1].x);
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

        // push lazy (C code mutates tree during query)
        let lazy = self.nodes[i].t;
        self.nodes[2 * i].t = tt(lazy, self.nodes[2 * i].t);
        self.nodes[2 * i + 1].t = tt(lazy, self.nodes[2 * i + 1].t);
        self.nodes[i].x = tx(lazy, self.nodes[i].x);
        self.nodes[i].t = SID;

        if r <= cm {
            return self.calc_sub(l, r, 2 * i, cl, cm);
        }
        if cm <= l {
            return self.calc_sub(l, r, 2 * i + 1, cm, cr);
        }
        let a = self.calc_sub(l, r, 2 * i, cl, cm);
        let b = self.calc_sub(l, r, 2 * i + 1, cm, cr);
        xx(a, b)
    }
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut q: i32 = it.next().unwrap().parse().unwrap();

    let mut seg = LazySeg::new(1usize << 17);

    // lseg[0].x = {0,0,0}
    seg.leaf_mut(0).x = Atai { m: 0, mmax: 0, x: 0 };

    let mut sum: Ll = 0;
    for i in 0..n {
        let t: Ll = it.next().unwrap().parse().unwrap();
        sum += t;
        seg.leaf_mut(i + 1).x = Atai {
            m: sum,
            mmax: sum,
            x: 0,
        };
    }
    seg.init();

    let mut out = String::new();
    out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));

    while q > 0 {
        q -= 1;
        let i_in: Ll = it.next().unwrap().parse().unwrap();
        let t: Ll = it.next().unwrap().parse().unwrap();
        let i = i_in as usize;

        // T = { t - calc(i,i+1).m + calc(i-1,i).m }
        let mi = seg.calc(i, i + 1).m;
        let miprev = seg.calc(i - 1, i).m;
        let delta = t - mi + miprev;
        let tnode = Sayouso { k: delta };

        seg.update(i, n + 1, tnode);
        out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));
    }

    print!("{out}");
}
