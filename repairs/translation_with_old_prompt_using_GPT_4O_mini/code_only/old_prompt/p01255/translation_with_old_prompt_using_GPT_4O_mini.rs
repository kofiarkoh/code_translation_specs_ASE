use std::io::{self, BufRead};
use std::f64;

const EPS: f64 = 1e-6;
const INF: f64 = 1e8;
const N: usize = 500;
const MAX: usize = 5000;

#[derive(Clone, Copy)]
struct PP {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct SEG {
    s: PP,
    e: PP,
}

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
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
    PP {
        x: p.x * k,
        y: p.y * k,
    }
}

fn cross(a: PP, b: PP) -> f64 {
    a.x * b.y - a.y * b.x
}

fn ccw(p: PP, q: PP, r: PP) -> i32 {
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dx_ = r.x - p.x;
    let dy_ = r.y - p.y;
    let t = dx * dy_ - dy * dx_;
    if t.abs() < EPS {
        return 0;
    }
    if t <= 0.0 {
        return -1;
    }
    1
}

fn dir(s: SEG) -> PP {
    vsub(s.e, s.s)
}

fn intersect(s1: SEG, s2: SEG) -> bool {
    ccw(s1.s, s1.e, s2.s) * ccw(s1.s, s1.e, s2.e) <= 0 &&
    ccw(s2.s, s2.e, s1.s) * ccw(s2.s, s2.e, s1.e) <= 0
}

fn cross_point(ok: &mut i32, s1: SEG, s2: SEG) -> PP {
    let n = cross(vsub(s2.s, s1.s), dir(s2));
    let d = cross(dir(s1), dir(s2));
    if eq(d, 0.0) {
        *ok = 0;
        return PP { x: 1.0, y: 1.0 };
    }
    *ok = 1;
    vadd(s1.s, vsmul(dir(s1), n / d))
}

fn uniq(a: &mut Vec<PP>, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut i = 0;
    let mut j = 1;
    while j < n {
        while j < n && eq(a[i].x, a[j].x) && eq(a[i].y, a[j].y) {
            j += 1;
        }
        if j < n {
            if i + 1 != j {
                a[i + 1] = a[j];
            }
            i += 1;
        }
    }
    i + 1
}

#[derive(Clone, Copy)]
struct T {
    s: SEG,
    id: usize,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct S {
    v: usize,
    hi: usize,
    hi2: usize,
}

fn cmp(a: &PP, b: &PP) -> std::cmp::Ordering {
    if eq(a.x, b.x) {
        if eq(a.y, b.y) {
            return std::cmp::Ordering::Equal;
        }
        if a.y <= b.y {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Greater;
    }
    if a.x <= b.x {
        return std::cmp::Ordering::Less;
    }
    std::cmp::Ordering::Greater
}

fn cmp2(a: &S, b: &S) -> std::cmp::Ordering {
    if b.hi != a.hi {
        return b.hi.cmp(&a.hi);
    }
    b.v.cmp(&a.v)
}

fn pushback(aa: &mut [[A; 25]; MAX], i: usize, iid: usize, j: usize, jid: usize, ff: &mut [usize]) {
    for k in 0..aa[i][iid].hi {
        if aa[i][iid].to[k] == j && aa[i][iid].id[k] == jid {
            return;
        }
    }
    if aa[i][iid].hi == 0 {
        ff[i] += 1;
    }
    let k = aa[i][iid].hi;
    aa[i][iid].to[k] = j;
    aa[i][iid].id[k] = jid;
    aa[i][iid].hi += 1;
}

fn segarrange(data: &mut Vec<T>, n: usize, aa: &mut [[A; 25]; MAX], ff: &mut [usize], tbl: &mut Vec<PP>, sz: &mut usize) {
    tbl.clear();
    *sz = 0;
    for i in 0..n {
        tbl.push(data[i].s.s);
        tbl.push(data[i].s.e);
    }
    for i in 0..n {
        for j in 0..i {
            if intersect(data[i].s, data[j].s) {
                let mut k = 0;
                let cross_point = cross_point(&mut k, data[i].s, data[j].s);
                if k != 0 {
                    tbl.push(cross_point);
                }
            }
        }
    }
    tbl.sort_by(cmp);
    *sz = uniq(tbl, *sz);
    ff.fill(0);
    for i in 0..n {
        let mut k = 0;
        let mut u = vec![U { u: 0, id: 0 }; n];
        for j in 0..*sz {
            if ccw(data[i].s.s, data[i].s.e, tbl[j]) == 0 {
                u[k].u = j;
                u[k].id = data[i].id;
                k += 1;
            }
        }
        for j in 0..k - 1 {
            pushback(aa, u[j].u, u[j].id, u[j + 1].u, u[j + 1].id, ff);
            pushback(aa, u[j + 1].u, u[j + 1].id, u[j].u, u[j].id, ff);
        }
    }
}

fn welsh_powell(ss: &mut [S; MAX], lim: usize, aa: &[[A; 25]; MAX], sz: usize) -> usize {
    let mut ans = lim;
    let mut color = [0; 25];
    let mut conflict = [[0; 25]; 25];
    let mut q = [0; 25];

    if ss[0].hi == lim {
        return lim;
    }
    if ss[0].hi == 1 {
        return 1;
    }
    for i in 0..lim {
        conflict[i][i] = 1;
    }
    for i in 0..sz {
        let u = ss[i].v;
        for c in 0..lim {
            if aa[u][c].hi > 0 {
                for cc in c + 1..lim {
                    if aa[u][cc].hi > 0 {
                        conflict[c][cc] = 1;
                        conflict[cc][c] = 1;
                    }
                }
            }
        }
    }

    for m in 0..2 {
        let r = 0; // Removed mutability
        color.fill(0);
        for i in 0..sz {
            let u = ss[i].v;
            let mut f = 0;
            for c in 0..lim {
                if aa[u][c].hi > 0 {
                    if color[c] == 0 {
                        color[c] = r + 1;
                        q[f] = c;
                        f += 1;
                        break;
                    }
                }
            }
            if f == 0 {
                continue;
            }
            if m == 0 {
                for j in 0..sz {
                    let u = ss[j].v;
                    for cc in 0..lim {
                        if aa[u][cc].hi > 0 && color[cc] == 0 {
                            for k in 0..f {
                                if k == f {
                                    color[cc] = r + 1;
                                    q[f] = cc;
                                    break;
                                }
                                if conflict[q[k]][cc] != 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                for j in (0..sz).rev() {
                    let u = ss[j].v;
                    for cc in 0..lim {
                        if aa[u][cc].hi > 0 && color[cc] == 0 {
                            for k in 0..f {
                                if k == f {
                                    color[cc] = r + 1;
                                    q[f] = cc;
                                    break;
                                }
                                if conflict[q[k]][cc] != 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        if r < ans {
            ans = r;
        }
    }
    ans
}

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut data = vec![T { s: SEG { s: PP { x: 0.0, y: 0.0 }, e: PP { x: 0.0, y: 0.0 } }, id: 0 }; N];
    let mut aa = [[A { hi: 0, to: [0; 25], id: [0; 25] }; 25]; MAX];
    let mut ff = [0; MAX];
    let mut tbl = Vec::new();
    let mut sz = 0;

    loop {
        let lim = in_number();
        if lim == 0 {
            break;
        }
        let mut n = 0;
        for i in 0..lim {
            let s = in_number();
            if s == 0 {
                continue;
            }
            data[n].s.s.x = in_number() as f64;
            data[n].s.s.y = in_number() as f64;
            for _ in 1..s {
                data[n].id = i as usize;
                data[n].s.e.x = in_number() as f64;
                data[n].s.e.y = in_number() as f64;
                n += 1;
                data[n].s.s.x = data[n].s.e.x;
                data[n].s.s.y = data[n].s.e.y;
            }
        }
        segarrange(&mut data, n, &mut aa, &mut ff, &mut tbl, &mut sz);
        println!("{}", welsh_powell(&mut [S { v: 0, hi: 0, hi2: 0 }; MAX], lim as usize, &aa, sz));
    }
}
