// AOJ 1288 Digits on the Floor
// 2018.4.5 bal4u

use std::io::{self, BufRead};

const MAX: usize = 1002;

struct Point {
    x: i32,
    y: i32,
}

struct Segment {
    s: Point,
    e: Point,
}

struct T {
    n: usize,
    u: [Segment; 5],
    p: usize,
}

struct UnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut id = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
            size.push(1);
        }
        UnionFind { id, size }
    }

    fn root(&mut self, i: usize) -> usize {
        if i != self.id[i] {
            self.id[i] = self.root(self.id[i]);
        }
        self.id[i]
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn unite(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.id[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.id[j] = i;
            self.size[i] += self.size[j];
        }
    }
}

fn vset(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn vsub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn vmul(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x * p2.x - p1.y * p2.y,
        y: p1.x * p2.y + p1.y * p2.x,
    }
}

fn dot(a: Point, b: Point) -> i32 {
    a.x * b.x + a.y * b.y
}

fn in_number() -> usize {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn check256(id: &T, f: &[[usize; MAX]; MAX], ans: &mut [usize; 10]) {
    let mut j = 0;
    let mut p0 = [Point { x: 0, y: 0 }; 2];
    let mut p1 = [Point { x: 0, y: 0 }; 2];
    let mut p2 = [Point { x: 0, y: 0 }; 2];

    for i in 0..id.n {
        if f[id.u[i].s.x][id.u[i].s.y] == 1 {
            p0[j] = id.u[i].s;
            p1[j] = id.u[i].e;
            j += 1;
        } else if f[id.u[i].e.x][id.u[i].e.y] == 1 {
            p0[j] = id.u[i].e;
            p1[j] = id.u[i].s;
            j += 1;
        }
    }

    for i in 0..id.n {
        if !peqp(&id.u[i].s, &p0[0]) && peqp(&id.u[i].e, &p1[0]) {
            p2[0] = id.u[i].s;
        } else if !peqp(&id.u[i].e, &p0[0]) && peqp(&id.u[i].s, &p1[0]) {
            p2[0] = id.u[i].e;
        }
        if !peqp(&id.u[i].s, &p0[1]) && peqp(&id.u[i].e, &p1[1]) {
            p2[1] = id.u[i].s;
        } else if !peqp(&id.u[i].e, &p0[1]) && peqp(&id.u[i].s, &p1[1]) {
            p2[1] = id.u[i].e;
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

fn peqp(a: &Point, b: &Point) -> bool {
    a.x == b.x && a.y == b.y
}

fn p_on_seg(p: Point, u: &Segment) -> i32 {
    let (x1, y1, x2, y2) = (u.s.x, u.s.y, u.e.x, u.e.y);
    if peqp(&p, &u.s) || peqp(&p, &u.e) {
        return 0;
    }
    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    if x1 <= p.x && p.x <= x2 {
        if (y1 <= y2 && y1 <= p.y && p.y <= y2) || (y1 > y2 && y2 <= p.y && p.y <= y1) {
            return if (p.y - y1) * (x2 - x1) == (y2 - y1) * (p.x - x1) {
                1
            } else {
                -1
            };
        }
    }
    -1
}

fn check08(id: &T, u: &[Segment], n: usize, ans: &mut [usize; 10], uf: &mut UnionFind) {
    for i in 1..=n {
        if uf.root(i) == i {
            for j in 0..id.n {
                if p_on_seg(u[i].s, &id.u[j]) >= 0 {
                    ans[8] += 1;
                    return;
                }
                if p_on_seg(u[i].e, &id.u[j]) >= 0 {
                    ans[8] += 1;
                    return;
                }
            }
        }
    }
    ans[0] += 1;
}

fn check37(id: &T, u: &[Segment], n: usize, ans: &mut [usize; 10], uf: &mut UnionFind, f: &[[usize; MAX]; MAX]) {
    for i in 0..id.n {
        if f[id.u[i].s.x][id.u[i].s.y] > 1 && f[id.u[i].e.x][id.u[i].e.y] > 1 {
            for j in 1..=n {
                if uf.root(j) == j {
                    if p_on_seg(u[j].s, &id.u[i]) >= 0 {
                        ans[3] += 1;
                        return;
                    }
                    if p_on_seg(u[j].e, &id.u[i]) >= 0 {
                        ans[3] += 1;
                        return;
                    }
                }
            }
        }
    }
    ans[7] += 1;
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut ans = [0; 10];
    let mut t: [T; MAX] = Default::default();
    let mut u: [Segment; MAX] = Default::default();
    let mut f: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
    let mut uid: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
    let mut n: usize;

    while {
        n = in_number();
        n > 0
    } {
        ans.fill(0);
        t.fill(Default::default());
        f.iter_mut().for_each(|row| row.fill(0));
        uid.iter_mut().for_each(|row| row.fill(0));
        let mut uf = UnionFind::new(n + 1);

        for i in 1..=n {
            let x1 = in_number() as i32;
            let y1 = in_number() as i32;
            let x2 = in_number() as i32;
            let y2 = in_number() as i32;
            u[i].s = Point { x: x1, y: y1 };
            u[i].e = Point { x: x2, y: y2 };
            if f[x1 as usize][y1 as usize] > 0 {
                uf.unite(uid[x1 as usize][y1 as usize], i);
            } else {
                uid[x1 as usize][y1 as usize] = i;
            }
            f[x1 as usize][y1 as usize] += 1;
            if f[x2 as usize][y2 as usize] > 0 {
                uf.unite(uid[x2 as usize][y2 as usize], i);
            } else {
                uid[x2 as usize][y2 as usize] = i;
            }
            f[x2 as usize][y2 as usize] += 1;
        }

        for i in 1..=n {
            let j = uf.root(i);
            let k = t[j].n;
            t[j].u[k] = u[i];
            t[j].n += 1;
            if f[u[i].s.x as usize][u[i].s.y as usize] > 1 {
                t[j].p += 1;
            }
            if f[u[i].e.x as usize][u[i].e.y as usize] > 1 {
                t[j].p += 1;
            }
        }

        let mut s = 0;
        for i in 1..=n {
            if t[i].n >= 2 {
                s += t[i].n;
                if t[i].n == 2 && t[i].p == 2 {
                    ans[4] += 1;
                } else if t[i].n == 4 && t[i].p == 6 {
                    ans[9] += 1;
                } else if t[i].n == 4 && t[i].p == 8 {
                    check08(&t[i], &u, n, &mut ans, &mut uf);
                } else if t[i].n == 5 && t[i].p == 8 {
                    check256(&t[i], &f, &mut ans);
                } else if t[i].n == 3 && t[i].p == 4 {
                    check37(&t[i], &u, n, &mut ans, &mut uf, &f);
                }
            }
        }
        s += ans[4] + ans[3] + ans[8];
        ans[1] = n - s;
        print!("{}", ans[0]);
        for i in 1..10 {
            print!(" {}", ans[i]);
        }
        println!();
    }
}
