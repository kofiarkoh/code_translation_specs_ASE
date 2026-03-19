use std::io::{self, Read};

const MAX: usize = 1002;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct PP {
    x: i32,
    y: i32,
}
#[derive(Copy, Clone, Default)]
struct SEG {
    s: PP,
    e: PP,
}
#[derive(Copy, Clone)]
struct T {
    n: u8,          // number of segments in this component (<= 5 in this problem's usage)
    u: [SEG; 5],    // stored segments
    p: u8,          // count of endpoints with degree > 1
}
impl Default for T {
    fn default() -> Self {
        Self {
            n: 0,
            u: [SEG::default(); 5],
            p: 0,
        }
    }
}

/* UNION-FIND library */
#[derive(Clone)]
struct UnionFind {
    id: [i32; MAX],
    size: [i32; MAX],
}
impl UnionFind {
    fn new() -> Self {
        Self {
            id: [0; MAX],
            size: [0; MAX],
        }
    }
    fn init(&mut self, n: usize) {
        for i in 0..n {
            self.id[i] = i as i32;
            self.size[i] = 1;
        }
    }
    fn root(&mut self, mut i: i32) -> i32 {
        // Path compression by halving, matching the C code.
        while i != self.id[i as usize] {
            let parent = self.id[i as usize];
            let grand = self.id[parent as usize];
            self.id[i as usize] = grand;
            i = self.id[i as usize];
        }
        i
    }
    fn unite(&mut self, p: i32, q: i32) {
        let mut i = self.root(p);
        let mut j = self.root(q);
        if i == j {
            return;
        }
        let si = self.size[i as usize];
        let sj = self.size[j as usize];
        if si < sj {
            self.id[i as usize] = j;
            self.size[j as usize] += si;
        } else {
            self.id[j as usize] = i;
            self.size[i as usize] += sj;
        }
    }
}

fn vset(x: i32, y: i32) -> PP {
    PP { x, y }
}
fn vsub(p1: PP, p2: PP) -> PP {
    PP {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}
fn vmul(p1: PP, p2: PP) -> PP {
    // Complex multiplication (x + yi) * (x + yi)
    PP {
        x: p1.x * p2.x - p1.y * p2.y,
        y: p1.x * p2.y + p1.y * p2.x,
    }
}
fn dot(a: PP, b: PP) -> i32 {
    a.x * b.x + a.y * b.y
}

fn p_on_seg(p: PP, u: SEG) -> bool {
    let mut x1 = u.s.x;
    let mut y1 = u.s.y;
    let mut x2 = u.e.x;
    let mut y2 = u.e.y;
    if p == u.s || p == u.e {
        return false;
    }
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
    }
    x1 <= p.x
        && p.x <= x2
        && ((y1 <= y2 && y1 <= p.y && p.y <= y2) || (y1 > y2 && y2 <= p.y && p.y <= y1))
        && (p.y - y1) * (x2 - x1) == (y2 - y1) * (p.x - x1)
}

fn check256(id: &T, f: &[[u8; MAX]; MAX], ans: &mut [i32; 10]) {
    let mut p0 = [PP::default(); 2];
    let mut p1 = [PP::default(); 2];
    let mut p2 = [PP::default(); 2];

    let mut j: usize = 0;
    for i in 0..(id.n as usize) {
        let seg = id.u[i];
        if f[seg.s.x as usize][seg.s.y as usize] == 1 {
            p0[j] = seg.s;
            p1[j] = seg.e;
            j += 1;
        } else if f[seg.e.x as usize][seg.e.y as usize] == 1 {
            p0[j] = seg.e;
            p1[j] = seg.s;
            j += 1;
        }
    }

    for i in 0..(id.n as usize) {
        let seg = id.u[i];
        if seg.s != p0[0] && seg.e == p1[0] {
            p2[0] = seg.s;
        } else if seg.e != p0[0] && seg.s == p1[0] {
            p2[0] = seg.e;
        }
        if seg.s != p0[1] && seg.e == p1[1] {
            p2[1] = seg.s;
        } else if seg.e != p0[1] && seg.s == p1[1] {
            p2[1] = seg.e;
        }
    }

    let p = vmul(vsub(p0[0], p1[0]), vset(0, 1));
    let k1 = dot(vsub(p2[0], p1[0]), p);

    let p = vmul(vsub(p0[1], p1[1]), vset(0, 1));
    let k2 = dot(vsub(p2[1], p1[1]), p);

    if k1 > 0 && k2 > 0 {
        ans[2] += 1;
    } else if k1 < 0 && k2 < 0 {
        ans[5] += 1;
    } else {
        ans[6] += 1;
    }
}

fn check08(
    id: &T,
    n: usize,
    uf: &mut UnionFind,
    u: &[SEG; MAX],
    ans: &mut [i32; 10],
) {
    for i in 1..=n {
        if uf.root(i as i32) == i as i32 {
            for j in 0..(id.n as usize) {
                if p_on_seg(u[i].s, id.u[j]) {
                    ans[8] += 1;
                    return;
                }
                if p_on_seg(u[i].e, id.u[j]) {
                    ans[8] += 1;
                    return;
                }
            }
        }
    }
    ans[0] += 1;
}

fn check37(
    id: &T,
    n: usize,
    uf: &mut UnionFind,
    u: &[SEG; MAX],
    f: &[[u8; MAX]; MAX],
    ans: &mut [i32; 10],
) {
    for i in 0..(id.n as usize) {
        let seg = id.u[i];
        if f[seg.s.x as usize][seg.s.y as usize] > 1 && f[seg.e.x as usize][seg.e.y as usize] > 1
        {
            for j in 1..=n {
                if uf.root(j as i32) == j as i32 {
                    if p_on_seg(u[j].s, seg) {
                        ans[3] += 1;
                        return;
                    }
                    if p_on_seg(u[j].e, seg) {
                        ans[3] += 1;
                        return;
                    }
                }
            }
        }
    }
    ans[7] += 1;
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }
    fn next_u32(&mut self) -> Option<u32> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut n: u32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c & 0x0f) as u32;
            self.idx += 1;
        }
        Some(n)
    }
}

fn main() {
    let mut sc = Scanner::new();

    let mut uf = UnionFind::new();
    let mut t: [T; MAX] = std::array::from_fn(|_| T::default());
    let mut u: [SEG; MAX] = [SEG::default(); MAX];
    let mut f: [[u8; MAX]; MAX] = [[0u8; MAX]; MAX];
    let mut uid: [[i16; MAX]; MAX] = [[0i16; MAX]; MAX];
    let mut ans: [i32; 10] = [0; 10];

    let mut out = String::new();

    while let Some(nn) = sc.next_u32() {
        let n = nn as usize;
        if n == 0 {
            break;
        }

        ans = [0; 10];
        // Reset only what C resets: t[0..=n], and full f/uid.
        for i in 0..=n {
            t[i] = T::default();
        }
        f = [[0u8; MAX]; MAX];
        uid = [[0i16; MAX]; MAX];
        uf.init(n + 1);

        for i in 1..=n {
            let x1 = sc.next_u32().unwrap() as usize;
            let y1 = sc.next_u32().unwrap() as usize;
            let x2 = sc.next_u32().unwrap() as usize;
            let y2 = sc.next_u32().unwrap() as usize;

            u[i].s = PP {
                x: x1 as i32,
                y: y1 as i32,
            };
            u[i].e = PP {
                x: x2 as i32,
                y: y2 as i32,
            };

            if f[x1][y1] != 0 {
                uf.unite(uid[x1][y1] as i32, i as i32);
            } else {
                uid[x1][y1] = i as i16;
            }
            f[x1][y1] = f[x1][y1].wrapping_add(1);

            if f[x2][y2] != 0 {
                uf.unite(uid[x2][y2] as i32, i as i32);
            } else {
                uid[x2][y2] = i as i16;
            }
            f[x2][y2] = f[x2][y2].wrapping_add(1);
        }

        for i in 1..=n {
            let j = uf.root(i as i32) as usize;
            let k = t[j].n as usize;
            t[j].n = t[j].n.wrapping_add(1);
            if k < 5 {
                t[j].u[k] = u[i];
            }
            if f[u[i].s.x as usize][u[i].s.y as usize] > 1 {
                t[j].p = t[j].p.wrapping_add(1);
            }
            if f[u[i].e.x as usize][u[i].e.y as usize] > 1 {
                t[j].p = t[j].p.wrapping_add(1);
            }
        }

        let mut s: i32 = 0;
        for i in 1..=n {
            if t[i].n >= 2 {
                s += t[i].n as i32;
                if t[i].n == 2 && t[i].p == 2 {
                    ans[4] += 1;
                } else if t[i].n == 4 && t[i].p == 6 {
                    ans[9] += 1;
                } else if t[i].n == 4 && t[i].p == 8 {
                    check08(&t[i], n, &mut uf, &u, &mut ans);
                } else if t[i].n == 5 && t[i].p == 8 {
                    check256(&t[i], &f, &mut ans);
                } else if t[i].n == 3 && t[i].p == 4 {
                    check37(&t[i], n, &mut uf, &u, &f, &mut ans);
                }
            }
        }
        s += ans[4] + ans[3] + ans[8];
        ans[1] = n as i32 - s;

        out.push_str(&format!("{}", ans[0]));
        for i in 1..10 {
            out.push_str(&format!(" {}", ans[i]));
        }
        out.push('\n');
    }

    print!("{out}");
}
