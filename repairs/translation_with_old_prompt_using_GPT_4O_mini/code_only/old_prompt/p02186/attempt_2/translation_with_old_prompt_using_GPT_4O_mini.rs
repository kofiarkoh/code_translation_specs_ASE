use std::io::{self, BufRead};
use std::cmp::{min, max};

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

struct Node {
    T: Sayouso,
    x: Atai,
}

struct LazySegmentTree {
    lsegN: Vec<Node>,
    lsegNUM: usize,
}

impl LazySegmentTree {
    fn new(n: usize) -> Self {
        let mut lsegN = vec![Node { T: Sayouso { k: 0 }, x: Atai { m: INF, M: -INF, x: 0 } }; 2 * n];
        let lsegNUM = n;

        for i in 0..lsegNUM {
            lsegN[i].x = Atai { m: INF, M: -INF, x: 0 };
            lsegN[i].T = Sayouso { k: 0 };
        }
        for i in (1..lsegNUM).rev() {
            lsegN[i].x = Self::xx(lsegN[2 * i].x, lsegN[2 * i + 1].x);
            lsegN[i].T = Sayouso { k: 0 };
        }
        LazySegmentTree { lsegN, lsegNUM }
    }

    fn xx(x: Atai, y: Atai) -> Atai {
        Atai {
            m: min(x.m, y.m),
            M: max(x.M, y.M),
            x: max(y.M - x.m, max(x.x, y.x)),
        }
    }

    fn tx(T: Sayouso, x: Atai) -> Atai {
        Atai {
            m: x.m + T.k,
            M: x.M + T.k,
            x: x.x,
        }
    }

    fn tt(S: Sayouso, T: Sayouso) -> Sayouso {
        Sayouso { k: S.k + T.k }
    }

    fn lsegupdate(&mut self, l: i64, r: i64, T: Sayouso) {
        self.lsegupdatesub(l, r, T, 1, 0, self.lsegNUM as i64);
    }

    fn lsegupdatesub(&mut self, l: i64, r: i64, T: Sayouso, i: i64, cl: i64, cr: i64) {
        if l >= r {
            return;
        }
        if cr <= l || r <= cl {
            return;
        }
        if l <= cl && cr <= r {
            self.lsegN[i as usize].T = Self::tt(T, self.lsegN[i as usize].T);
            return;
        }
        self.lsegN[(2 * i) as usize].T = Self::tt(self.lsegN[i as usize].T, self.lsegN[(2 * i) as usize].T);
        self.lsegN[(2 * i + 1) as usize].T = Self::tt(self.lsegN[i as usize].T, self.lsegN[(2 * i + 1) as usize].T);
        let cm = (cl + cr) / 2;
        self.lsegupdatesub(l, r, T, 2 * i, cl, cm);
        self.lsegupdatesub(l, r, T, 2 * i + 1, cm, cr);
        self.lsegN[i as usize].x = Self::xx(Self::tx(self.lsegN[(2 * i) as usize].T, self.lsegN[(2 * i) as usize].x), Self::tx(self.lsegN[(2 * i + 1) as usize].T, self.lsegN[(2 * i + 1) as usize].x));
        self.lsegN[i as usize].T = Sayouso { k: 0 };
    }

    fn lsegcalc(&mut self, l: i64, r: i64) -> Atai {
        self.lsegcalcsub(l, r, 1, 0, self.lsegNUM as i64)
    }

    fn lsegcalcsub(&mut self, l: i64, r: i64, i: i64, cl: i64, cr: i64) -> Atai {
        if l >= r {
            return Atai { m: INF, M: -INF, x: 0 };
        }
        if l <= cl && cr <= r {
            return Self::tx(self.lsegN[i as usize].T, self.lsegN[i as usize].x);
        }

        let cm = (cl + cr) / 2;
        self.lsegN[(2 * i) as usize].T = Self::tt(self.lsegN[i as usize].T, self.lsegN[(2 * i) as usize].T);
        self.lsegN[(2 * i + 1) as usize].T = Self::tt(self.lsegN[i as usize].T, self.lsegN[(2 * i + 1) as usize].T);
        self.lsegN[i as usize].x = Self::tx(self.lsegN[i as usize].T, self.lsegN[i as usize].x);
        self.lsegN[i as usize].T = Sayouso { k: 0 };

        if r <= cm {
            return self.lsegcalcsub(l, r, 2 * i, cl, cm);
        }
        if cm <= l {
            return self.lsegcalcsub(l, r, 2 * i + 1, cm, cr);
        }
        Self::xx(self.lsegcalcsub(l, r, 2 * i, cl, cm), self.lsegcalcsub(l, r, 2 * i + 1, cm, cr))
    }
}

const INF: i64 = (1 << 62) - (1 << 31);

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut lseg_tree = LazySegmentTree::new(1 << 17);
    lseg_tree.lsegN[0].x = Atai { m: 0, M: 0, x: 0 };

    let mut s = 0;
    for i in 0..n {
        let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        s += t;
        lseg_tree.lsegN[i + 1].x = Atai { m: s, M: s, x: 0 };
    }
    lseg_tree.lsegupdatesub(0, n as i64 + 1, Sayouso { k: 0 }, 1, 0, lseg_tree.lsegNUM as i64);

    println!("{}", lseg_tree.lsegcalc(0, n as i64 + 1).x);
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let i: i64 = parts.next().unwrap().parse().unwrap();
        let t: i64 = parts.next().unwrap().parse().unwrap();
        let T = Sayouso { k: t - lseg_tree.lsegcalc(i, i + 1).m + lseg_tree.lsegcalc(i - 1, i).m };
        lseg_tree.lsegupdate(i, n as i64 + 1, T);
        println!("{}", lseg_tree.lsegcalc(0, n as i64 + 1).x);
    }
}
