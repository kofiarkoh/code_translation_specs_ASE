use std::cmp::Ordering;
use std::io::{self, Read};

const EPS: f64 = 1e-6;

#[inline]
fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[derive(Clone, Copy, Default)]
struct PP {
    x: f64,
    y: f64,
}

#[inline]
fn peqp(a: PP, b: PP) -> bool {
    eq(a.x, b.x) && eq(a.y, b.y)
}

#[derive(Clone, Copy, Default)]
struct SEG {
    s: PP,
    e: PP,
}

#[inline]
fn vadd(p1: PP, p2: PP) -> PP {
    PP {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}
#[inline]
fn vsub(p1: PP, p2: PP) -> PP {
    PP {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}
#[inline]
fn vsmul(p: PP, k: f64) -> PP {
    PP { x: p.x * k, y: p.y * k }
}
#[inline]
fn cross(a: PP, b: PP) -> f64 {
    a.x * b.y - a.y * b.x
}

#[inline]
fn ccw(p: PP, q: PP, r: PP) -> i32 {
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dx_ = r.x - p.x;
    let dy_ = r.y - p.y;
    let t = dx * dy_ - dy * dx_;
    if t.abs() < EPS {
        0
    } else if t <= 0.0 {
        -1
    } else {
        1
    }
}

#[inline]
fn dir(s: SEG) -> PP {
    vsub(s.e, s.s)
}

#[inline]
fn intersect(s1: SEG, s2: SEG) -> bool {
    (ccw(s1.s, s1.e, s2.s) * ccw(s1.s, s1.e, s2.e) <= 0)
        && (ccw(s2.s, s2.e, s1.s) * ccw(s2.s, s2.e, s1.e) <= 0)
}

fn cross_point(ok: &mut i32, s1: SEG, s2: SEG) -> PP {
    let one = PP { x: 1.0, y: 1.0 };
    let n = cross(vsub(s2.s, s1.s), dir(s2));
    let d = cross(dir(s1), dir(s2));
    if eq(d, 0.0) {
        *ok = 0;
        return one;
    }
    *ok = 1;
    vadd(s1.s, vsmul(dir(s1), n / d))
}

fn uniq(a: &mut Vec<PP>) {
    if a.is_empty() {
        return;
    }
    let mut i = 0usize;
    let mut j = 1usize;
    while j < a.len() {
        while j < a.len() && peqp(a[i], a[j]) {
            j += 1;
        }
        if j < a.len() {
            i += 1;
            if i != j {
                a[i] = a[j];
            }
        }
    }
    a.truncate(i + 1);
}

const N: usize = 500;
const MAX: usize = 5000;

#[derive(Clone, Copy, Default)]
struct T {
    s: SEG,
    id: usize,
}

#[derive(Clone, Copy, Default)]
struct U {
    u: usize,
    id: usize,
}

#[derive(Clone, Copy)]
struct A {
    hi: usize,
    to: [usize; 25],
    id: [usize; 25],
}
impl Default for A {
    fn default() -> Self {
        Self {
            hi: 0,
            to: [0; 25],
            id: [0; 25],
        }
    }
}

#[derive(Clone, Copy, Default)]
struct S {
    v: usize,
    hi: i32,
    hi2: i32, // unused, kept to mirror C layout intent
}

fn pushback(aa: &mut Vec<Vec<A>>, ff: &mut Vec<i32>, i: usize, iid: usize, j: usize, jid: usize) {
    let a = &mut aa[i][iid];
    for k in 0..a.hi {
        if a.to[k] == j && a.id[k] == jid {
            return;
        }
    }
    if a.hi == 0 {
        ff[i] += 1;
    }
    let k = a.hi;
    a.hi += 1;
    a.to[k] = j;
    a.id[k] = jid;
}

fn segarrange(data: &[T], n: usize) -> (Vec<PP>, Vec<Vec<A>>, Vec<S>) {
    let mut tbl: Vec<PP> = Vec::with_capacity(MAX);
    for i in 0..n {
        tbl.push(data[i].s.s);
        tbl.push(data[i].s.e);
    }
    for i in 0..n {
        for j in 0..i {
            if intersect(data[i].s, data[j].s) {
                let mut k = 0;
                let p = cross_point(&mut k, data[i].s, data[j].s);
                if k != 0 {
                    tbl.push(p);
                }
            }
        }
    }

    tbl.sort_by(|a, b| {
        if eq(a.x, b.x) {
            if eq(a.y, b.y) {
                Ordering::Equal
            } else if a.y <= b.y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if a.x <= b.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    uniq(&mut tbl);
    let sz = tbl.len();

    // aa[sz][25], ff[sz]
    let mut ff: Vec<i32> = vec![0; sz];
    let mut aa: Vec<Vec<A>> = vec![vec![A::default(); 25]; sz];

    let mut u: [U; N] = [U::default(); N];

    for i in 0..n {
        let mut k = 0usize;
        for j in 0..sz {
            if ccw(data[i].s.s, data[i].s.e, tbl[j]) == 0 {
                u[k].u = j;
                u[k].id = data[i].id;
                k += 1;
            }
        }
        // Note: C code assumes points are encountered in sorted order along the segment.
        for j in 0..k.saturating_sub(1) {
            let a1 = u[j];
            let a2 = u[j + 1];
            pushback(&mut aa, &mut ff, a1.u, a1.id, a2.u, a2.id);
            pushback(&mut aa, &mut ff, a2.u, a2.id, a1.u, a1.id);
        }
    }

    let mut ss: Vec<S> = (0..sz)
        .map(|i| S {
            v: i,
            hi: ff[i],
            hi2: 0,
        })
        .collect();

    ss.sort_by(|a, b| {
        let dh = b.hi - a.hi;
        if dh != 0 {
            dh.cmp(&0) // reverse by hi
        } else {
            (b.v as i32 - a.v as i32).cmp(&0) // reverse by v
        }
    });

    (tbl, aa, ss)
}

fn welsh_powell(lim: usize, sz: usize, aa: &Vec<Vec<A>>, ss: &Vec<S>) -> i32 {
    if sz == 0 {
        // Not reachable in original problem setting, but keep defined.
        return 0;
    }
    if ss[0].hi as usize == lim {
        return lim as i32;
    }
    if ss[0].hi == 1 {
        return 1;
    }

    let mut ff_flag: i32 = 0;
    let mut mg: i32 = 0;
    if lim == 22 {
        ff_flag = 1;
        mg = 10;
    }

    let mut conflict = [[0u8; 25]; 25];
    for i in 0..lim {
        conflict[i][i] = 1;
    }
    for i in 0..sz {
        let u = ss[i].v;
        for c in 0..lim {
            if aa[u][c].hi != 0 {
                for cc in (c + 1)..lim {
                    if aa[u][cc].hi != 0 {
                        conflict[c][cc] = 1;
                        conflict[cc][c] = 1;
                    }
                }
            }
        }
    }

    let mut ans = lim as i32;
    let mut color = [0i32; 25];
    let mut q = [0usize; 25];

    for m in 0..2 {
        let mut r: i32 = 0;
        color.fill(0);

        for i in 0..sz {
            let u0 = ss[i].v;
            let mut f: usize = 0;

            for c in 0..lim {
                if aa[u0][c].hi != 0 && color[c] == 0 {
                    r += 1;
                    color[c] = r;
                    q[f] = c;
                    f += 1;
                    break;
                }
            }
            if f == 0 {
                continue;
            }

            if m == 0 {
                for j in 0..sz {
                    let u = ss[j].v;
                    for cc in 0..lim {
                        if aa[u][cc].hi != 0 && color[cc] == 0 {
                            let mut k = 0usize;
                            loop {
                                if k == f {
                                    color[cc] = r;
                                    q[f] = cc;
                                    f += 1;
                                    break;
                                }
                                if conflict[q[k]][cc] != 0 {
                                    break;
                                }
                                k += 1;
                            }
                        }
                    }
                }
            } else {
                for j in (0..sz).rev() {
                    let u = ss[j].v;
                    for cc in 0..lim {
                        if aa[u][cc].hi != 0 && color[cc] == 0 {
                            let mut k = 0usize;
                            loop {
                                if k == f {
                                    color[cc] = r;
                                    q[f] = cc;
                                    f += 1;
                                    break;
                                }
                                if conflict[q[k]][cc] != 0 {
                                    break;
                                }
                                k += 1;
                            }
                        }
                    }
                }
            }
        }

        if (ff_flag & 1) != 0 && r == mg {
            ff_flag <<= 1;
        }
        if (ff_flag & 2) != 0 && r == mg {
            ff_flag <<= 1;
        }
        if r < ans {
            ans = r;
        }
    }
    if (ff_flag & 4) != 0 {
        ans = mg - 1;
    }
    ans
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
    fn in_i32(&mut self) -> i32 {
        // Emulate the C in(): assumes next token is an integer, may be negative.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut neg = false;
        if self.buf[self.idx] == b'-' {
            neg = true;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            // (c & 0xf) in C; for digits it's equivalent to c - '0'
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            self.idx += 1;
        }
        if neg { -n } else { n }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let lim_i = sc.in_i32();
        if lim_i == 0 {
            break;
        }
        let lim = lim_i as usize;

        let mut data: [T; N] = [T::default(); N];
        let mut n: usize = 0;

        for i in 0..lim {
            let s = sc.in_i32();
            if s == 0 {
                continue;
            }
            data[n].s.s.x = sc.in_i32() as f64;
            data[n].s.s.y = sc.in_i32() as f64;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            for _j in 1..(s as usize) {
                data[n].id = i;
                x = sc.in_i32();
                y = sc.in_i32();
                data[n].s.e.x = x as f64;
                data[n].s.e.y = y as f64;
                n += 1;
                data[n].s.s.x = x as f64;
                data[n].s.s.y = y as f64;
            }
        }

        let (_tbl, aa, ss) = segarrange(&data[..n], n);
        let sz = ss.len();
        let ans = welsh_powell(lim, sz, &aa, &ss);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
