use std::io::{self, BufRead};

const MAX: usize = 1002;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Copy, Clone)]
struct Group {
    n: usize,
    segments: [Segment; 5],
    p: usize,
}

struct UnionFind {
    id: [usize; MAX],
    size: [usize; MAX],
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut id = [0; MAX];
        let mut size = [1; MAX];
        for i in 0..n {
            id[i] = i;
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

fn point_equal(a: &Point, b: &Point) -> bool {
    a.x == b.x && a.y == b.y
}

fn vector_sub(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn vector_mul(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p1.x * p2.x - p1.y * p2.y,
        y: p1.x * p2.y + p1.y * p2.x,
    }
}

fn dot(a: &Point, b: &Point) -> i32 {
    a.x * b.x + a.y * b.y
}

fn input() -> usize {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn point_on_segment(p: &Point, seg: &Segment) -> i32 {
    let x1 = seg.start.x;
    let y1 = seg.start.y;
    let x2 = seg.end.x;
    let y2 = seg.end.y;

    if point_equal(p, &seg.start) || point_equal(p, &seg.end) {
        return 0;
    }

    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

    if x1 <= p.x && p.x <= x2 && ((y1 <= y2 && y1 <= p.y && p.y <= y2) || (y1 > y2 && y2 <= p.y && p.y <= y1)) {
        if (p.y - y1) * (x2 - x1) == (y2 - y1) * (p.x - x1) {
            return 1;
        }
    }
    -1
}

fn check_256(group: &Group, segments: &[Segment], ans: &mut [i32; 10], f: &[[i32; MAX]; MAX]) {
    let mut p0 = [Point { x: 0, y: 0 }; 2];
    let mut p1 = [Point { x: 0, y: 0 }; 2];
    let mut p2 = [Point { x: 0, y: 0 }; 2];
    let mut j = 0;

    for i in 0..group.n {
        if f[group.segments[i].start.x as usize][group.segments[i].start.y as usize] == 1 {
            p0[j] = group.segments[i].start;
            p1[j] = group.segments[i].end;
            j += 1;
        } else if f[group.segments[i].end.x as usize][group.segments[i].end.y as usize] == 1 {
            p0[j] = group.segments[i].end;
            p1[j] = group.segments[i].start;
            j += 1;
        }
    }

    for i in 0..group.n {
        if !point_equal(&group.segments[i].start, &p0[0]) && point_equal(&group.segments[i].end, &p1[0]) {
            p2[0] = group.segments[i].start;
        } else if !point_equal(&group.segments[i].end, &p0[0]) && point_equal(&group.segments[i].start, &p1[0]) {
            p2[0] = group.segments[i].end;
        }
        if !point_equal(&group.segments[i].start, &p0[1]) && point_equal(&group.segments[i].end, &p1[1]) {
            p2[1] = group.segments[i].start;
        } else if !point_equal(&group.segments[i].end, &p0[1]) && point_equal(&group.segments[i].start, &p1[1]) {
            p2[1] = group.segments[i].end;
        }
    }

    let p = vector_mul(&vector_sub(&p0[0], &p1[0]), &Point { x: 0, y: 1 });
    let k1 = dot(&vector_sub(&p2[0], &p1[0]), &p);
    let p = vector_mul(&vector_sub(&p0[1], &p1[1]), &Point { x: 0, y: 1 });
    let k2 = dot(&vector_sub(&p2[1], &p1[1]), &p);

    if k1 > 0 && k2 > 0 {
        ans[2] += 1;
    } else if k1 < 0 && k2 < 0 {
        ans[5] += 1;
    } else {
        ans[6] += 1;
    }
}

fn check_08(group: &Group, segments: &[Segment], ans: &mut [i32; 10], uf: &mut UnionFind, f: &[[i32; MAX]; MAX]) {
    for i in 1..=MAX {
        if uf.root(i) == i {
            for j in 0..group.n {
                if point_on_segment(&segments[i], &group.segments[j]) > 0 {
                    ans[8] += 1;
                    return;
                }
                if point_on_segment(&segments[i], &group.segments[j]) > 0 {
                    ans[8] += 1;
                    return;
                }
            }
        }
    }
    ans[0] += 1;
}

fn check_37(group: &Group, segments: &[Segment], ans: &mut [i32; 10], uf: &mut UnionFind, f: &[[i32; MAX]; MAX]) {
    for i in 0..group.n {
        if f[group.segments[i].start.x as usize][group.segments[i].start.y as usize] > 1 &&
           f[group.segments[i].end.x as usize][group.segments[i].end.y as usize] > 1 {
            for j in 1..=MAX {
                if uf.root(j) == j {
                    if point_on_segment(&group.segments[i].start, &group.segments[j]) > 0 {
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
    let mut segments = [Segment { start: Point { x: 0, y: 0 }, end: Point { x: 0, y: 0 } }; MAX];
    let mut f = [[0; MAX]; MAX];
    let mut uid = [[0; MAX]; MAX];
    let mut n: usize;

    while {
        n = input();
        n > 0
    } {
        ans.fill(0);
        let mut groups = [Group { n: 0, segments: [Segment { start: Point { x: 0, y: 0 }, end: Point { x: 0, y: 0 } }; 5], p: 0 }; MAX];
        let mut uf = UnionFind::new(n + 1);

        for i in 1..=n {
            let x1 = input() as i32;
            let y1 = input() as i32;
            let x2 = input() as i32;
            let y2 = input() as i32;

            segments[i] = Segment {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            };

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
            let k = groups[j].n;
            groups[j].segments[k] = segments[i];
            groups[j].n += 1;
            if f[segments[i].start.x as usize][segments[i].start.y as usize] > 1 {
                groups[j].p += 1;
            }
            if f[segments[i].end.x as usize][segments[i].end.y as usize] > 1 {
                groups[j].p += 1;
            }
        }

        let mut s = 0;
        for i in 1..=n {
            if groups[i].n >= 2 {
                s += groups[i].n;
                match (groups[i].n, groups[i].p) {
                    (2, 2) => ans[4] += 1,
                    (4, 6) => ans[9] += 1,
                    (4, 8) => check_08(&groups[i], &segments, &mut ans, &mut uf, &f),
                    (5, 8) => check_256(&groups[i], &segments, &mut ans, &f),
                    (3, 4) => check_37(&groups[i], &segments, &mut ans, &mut uf, &f),
                    _ => {}
                }
            }
        }
        s += ans[4] + ans[3] + ans[8];
        ans[1] = n as i32 - s as i32;

        print!("{}", ans[0]);
        for i in 1..10 {
            print!(" {}", ans[i]);
        }
        println!();
    }
}
