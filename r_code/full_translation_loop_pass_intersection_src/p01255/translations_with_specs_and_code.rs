use std::cmp::Ordering;
use std::f64;
use std::io::{self, Read};

const EPS: f64 = 1e-6;

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[derive(Clone, Copy, Debug, Default)]
struct PP {
    x: f64,
    y: f64,
}

fn peq(a: PP, b: PP) -> bool {
    eq(a.x, b.x) && eq(a.y, b.y)
}

fn vadd(p1: PP, p2: PP) -> PP {
    PP {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}
fn vsub(p1: PP, p2: PP) -> PP {
    PP {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}
fn vsmul(p: PP, k: f64) -> PP {
    PP { x: p.x * k, y: p.y * k }
}
fn cross(a: PP, b: PP) -> f64 {
    a.x * b.y - a.y * b.x
}

#[derive(Clone, Copy, Debug, Default)]
struct SEG {
    s: PP,
    e: PP,
}

fn dir(s: SEG) -> PP {
    vsub(s.e, s.s)
}

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

fn intersect(s1: SEG, s2: SEG) -> bool {
    (ccw(s1.s, s1.e, s2.s) * ccw(s1.s, s1.e, s2.e) <= 0)
        && (ccw(s2.s, s2.e, s1.s) * ccw(s2.s, s2.e, s1.e) <= 0)
}

fn cross_point(s1: SEG, s2: SEG) -> Option<PP> {
    let n = cross(vsub(s2.s, s1.s), dir(s2));
    let d = cross(dir(s1), dir(s2));
    if eq(d, 0.0) {
        None
    } else {
        Some(vadd(s1.s, vsmul(dir(s1), n / d)))
    }
}

fn uniq_sorted_points(a: &mut Vec<PP>) {
    if a.is_empty() {
        return;
    }
    let mut out: Vec<PP> = Vec::with_capacity(a.len());
    out.push(a[0]);
    for &p in a.iter().skip(1) {
        if !peq(*out.last().unwrap(), p) {
            out.push(p);
        }
    }
    *a = out;
}

const N: usize = 500;
const MAX: usize = 5000;
const LIM_MAX: usize = 25;

#[derive(Clone, Copy, Debug, Default)]
struct T {
    s: SEG,
    id: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct U {
    u: usize,
    id: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct A {
    hi: usize,
    to: [usize; LIM_MAX],
    id: [usize; LIM_MAX],
}

#[derive(Clone, Copy, Debug, Default)]
struct S {
    v: usize,
    hi: i32,
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
    // Spec says fixed internal limits; C uses 25. We mirror that.
    if k < LIM_MAX {
        a.to[k] = j;
        a.id[k] = jid;
    }
}

fn segarrange(data: &[T], nseg: usize, lim: usize) -> (Vec<PP>, Vec<Vec<A>>, Vec<S>) {
    // Build tbl: endpoints + proper intersection points (excluding parallel/collinear).
    let mut tbl: Vec<PP> = Vec::with_capacity(2 * nseg + nseg * (nseg - 1) / 2);
    for i in 0..nseg {
        tbl.push(data[i].s.s);
        tbl.push(data[i].s.e);
    }
    for i in 0..nseg {
        for j in 0..i {
            if intersect(data[i].s, data[j].s) {
                if let Some(p) = cross_point(data[i].s, data[j].s) {
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
    uniq_sorted_points(&mut tbl);
    let sz = tbl.len();

    // aa[point_index][line_id] adjacency list of (neighbor_point, line_id)
    let mut aa: Vec<Vec<A>> = vec![vec![A::default(); lim]; sz];
    let mut ff: Vec<i32> = vec![0; sz];

    let mut u: Vec<U> = vec![U::default(); N]; // enough for C's usage

    for i in 0..nseg {
        let mut k = 0usize;
        for j in 0..sz {
            if ccw(data[i].s.s, data[i].s.e, tbl[j]) == 0 {
                if k < u.len() {
                    u[k] = U { u: j, id: data[i].id };
                }
                k += 1;
            }
        }
        // Connect consecutive collinear points in sorted tbl order (as in C).
        if k >= 2 {
            for j in 0..(k - 1) {
                let uj = u[j];
                let uj1 = u[j + 1];
                pushback(&mut aa, &mut ff, uj.u, uj.id, uj1.u, uj1.id);
                pushback(&mut aa, &mut ff, uj1.u, uj1.id, uj.u, uj.id);
            }
        }
    }

    let mut ss: Vec<S> = (0..sz)
        .map(|i| S {
            v: i,
            hi: ff[i],
        })
        .collect();

    ss.sort_by(|a, b| {
        // C: if (b->hi - a->hi) return b->hi - a->hi; return b->v - a->v;
        let dh = b.hi - a.hi;
        if dh != 0 {
            dh.cmp(&0)
        } else {
            (b.v as i32 - a.v as i32).cmp(&0)
        }
    });

    (tbl, aa, ss)
}

fn welsh_powell(lim: usize, sz: usize, aa: &[Vec<A>], ss: &[S]) -> i32 {
    if sz == 0 {
        // Spec notes behavior not inferable if no segments; C would read ss[0] and UB.
        // We choose a benign output consistent with "between 1 and lim" when lim>0.
        return if lim == 0 { 0 } else { 1 };
    }

    let top_hi = ss[0].hi;
    if top_hi as usize == lim {
        return lim as i32;
    }
    if top_hi == 1 {
        return 1;
    }

    // Special-case adjustment for lim == 22 (as in C).
    let mut ff_flag: i32 = 0;
    let mut mg: i32 = 0;
    if lim == 22 {
        ff_flag = 1;
        mg = 10;
    }

    let mut conflict = vec![vec![false; lim]; lim];
    for i in 0..lim {
        conflict[i][i] = true;
    }

    for i in 0..sz {
        let u = ss[i].v;
        for c in 0..lim {
            if aa[u][c].hi != 0 {
                for cc in (c + 1)..lim {
                    if aa[u][cc].hi != 0 {
                        conflict[c][cc] = true;
                        conflict[cc][c] = true;
                    }
                }
            }
        }
    }

    let mut ans: i32 = lim as i32;

    for m in 0..2 {
        let mut r: i32 = 0;
        let mut color = vec![0i32; lim];
        let mut q = vec![0usize; lim];

        for i in 0..sz {
            let u = ss[i].v;
            let mut f: usize = 0;

            // pick first uncolored line present at this point
            for c in 0..lim {
                if aa[u][c].hi != 0 && color[c] == 0 {
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
                    let u2 = ss[j].v;
                    for cc in 0..lim {
                        if aa[u2][cc].hi != 0 && color[cc] == 0 {
                            let mut k = 0usize;
                            loop {
                                if k == f {
                                    color[cc] = r;
                                    q[f] = cc;
                                    f += 1;
                                    break;
                                }
                                if conflict[q[k]][cc] {
                                    break;
                                }
                                k += 1;
                            }
                        }
                    }
                }
            } else {
                for jj in 0..sz {
                    let j = sz - 1 - jj;
                    let u2 = ss[j].v;
                    for cc in 0..lim {
                        if aa[u2][cc].hi != 0 && color[cc] == 0 {
                            let mut k = 0usize;
                            loop {
                                if k == f {
                                    color[cc] = r;
                                    q[f] = cc;
                                    f += 1;
                                    break;
                                }
                                if conflict[q[k]][cc] {
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
    fn next_i32(&mut self) -> Option<i32> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        Some(sign * n)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let lim_i = match sc.next_i32() {
            Some(v) => v,
            None => break,
        };
        if lim_i == 0 {
            break;
        }
        let lim = lim_i as usize;

        let mut data: Vec<T> = Vec::with_capacity(N);
        for i in 0..lim {
            let s = sc.next_i32().unwrap_or(0);
            if s <= 0 {
                continue;
            }
            let s = s as usize;
            let x0 = sc.next_i32().unwrap() as f64;
            let y0 = sc.next_i32().unwrap() as f64;
            let mut cur = PP { x: x0, y: y0 };
            for _j in 1..s {
                let x = sc.next_i32().unwrap() as f64;
                let y = sc.next_i32().unwrap() as f64;
                let next = PP { x, y };
                data.push(T {
                    s: SEG { s: cur, e: next },
                    id: i,
                });
                cur = next;
            }
        }

        let nseg = data.len();
        let (_tbl, aa, ss) = segarrange(&data, nseg, lim);
        let sz = ss.len();
        let ans = welsh_powell(lim, sz, &aa, &ss);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
