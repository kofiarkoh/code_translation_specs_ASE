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
    M: Ll,
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
    M: -INF,
    x: 0,
};

fn xx(a: Atai, b: Atai) -> Atai {
    // ret.m = min(a.m, b.m)
    // ret.M = max(a.M, b.M)
    // ret.x = max(b.M - a.m, max(a.x, b.x))
    Atai {
        m: min(a.m, b.m),
        M: max(a.M, b.M),
        x: max(b.M - a.m, max(a.x, b.x)),
    }
}

fn tx(t: Sayouso, x: Atai) -> Atai {
    // Apply lazy add to min/max; x unchanged.
    Atai {
        m: x.m + t.k,
        M: x.M + t.k,
        x: x.x,
    }
}

fn tt(s: Sayouso, t: Sayouso) -> Sayouso {
    // Compose: add amounts
    Sayouso { k: s.k + t.k }
}

struct LazySeg {
    n: usize,        // power-of-two size
    nodes: Vec<Node>, // 1-indexed segment tree in [1, 2*n)
}

impl LazySeg {
    fn new(n: usize) -> Self {
        // C uses calloc(2*n) and then sets leaves [n..2n) to aid/sid,
        // and internal nodes computed from children.
        let mut nodes = vec![
            Node {
                t: SID,
                x: AID
            };
            2 * n
        ];
        // leaves are already AID/SID; build internal nodes
        for i in (1..n).rev() {
            let left = nodes[2 * i].x;
            let right = nodes[2 * i + 1].x;
            nodes[i].x = xx(left, right);
            nodes[i].t = SID;
        }
        Self { n, nodes }
    }

    fn init(&mut self) {
        // Recompute internal nodes from current leaves (like lseginit()).
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

        // push lazy to children
        let cur_t = self.nodes[i].t;
        let li = 2 * i;
        let ri = 2 * i + 1;
        self.nodes[li].t = tt(cur_t, self.nodes[li].t);
        self.nodes[ri].t = tt(cur_t, self.nodes[ri].t);

        let cm = (cl + cr) / 2;
        self.update_sub(l, r, t, li, cl, cm);
        self.update_sub(l, r, t, ri, cm, cr);

        // pull
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

        // push lazy (C mutates tree during query)
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
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut q: i32 = it.next().unwrap().parse().unwrap();

    // lseguse(1<<17)
    let mut seg = LazySeg::new(1usize << 17);

    // lseg[0].x = {0,0,0}
    seg.nodes[seg.n + 0].x = Atai { m: 0, M: 0, x: 0 };

    let mut s: Ll = 0;
    for i in 0..n {
        let t: Ll = it.next().unwrap().parse().unwrap();
        s += t;
        seg.nodes[seg.n + (i + 1)].x = Atai { m: s, M: s, x: 0 };
    }

    seg.init();

    let mut out = String::new();
    out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));

    while q > 0 {
        q -= 1;
        let i_in: Ll = it.next().unwrap().parse().unwrap();
        let t: Ll = it.next().unwrap().parse().unwrap();

        // C uses ll i; and calls lsegcalc(i,i+1) and lsegcalc(i-1,i).
        // Indices are assumed valid by the C program; we mirror that.
        let i = i_in as usize;

        let mi = seg.calc(i, i + 1).m;
        let mim1 = seg.calc(i - 1, i).m;
        let delta = t - mi + mim1;
        let upd = Sayouso { k: delta };

        seg.update(i, n + 1, upd);
        out.push_str(&format!("{}\n", seg.calc(0, n + 1).x));
    }

    print!("{out}");
}
