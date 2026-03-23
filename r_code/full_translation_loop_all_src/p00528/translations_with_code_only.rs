use std::cmp::Ordering;
use std::io::{self, Read};

const QMAX: usize = 80000;
const INF: i32 = 0x7ffffff;
const MAXV: usize = 400005; // matches C arrays: to/dt/hi/used
const MAXSW: usize = 200005;

#[derive(Clone, Copy)]
struct Que {
    s: i32,
    t: i64,
}

#[derive(Clone, Copy)]
struct T {
    x: i32,
    y: i32,
    id: i32,
}

struct MinHeap {
    a: Vec<Que>,
}
impl MinHeap {
    fn new() -> Self {
        Self { a: Vec::with_capacity(QMAX) }
    }
    fn clear(&mut self) {
        self.a.clear();
    }
    fn is_empty(&self) -> bool {
        self.a.is_empty()
    }
    fn push(&mut self, s: i32, t: i64) {
        // C code assumes capacity is enough; we mirror behavior by allowing Vec growth.
        self.a.push(Que { s, t });
        let mut i = self.a.len() - 1;
        while i > 0 {
            let p = (i - 1) >> 1;
            if self.a[p].t <= self.a[i].t {
                break;
            }
            self.a.swap(i, p);
            i = p;
        }
    }
    fn pop_min(&mut self) -> Option<Que> {
        if self.a.is_empty() {
            return None;
        }
        let min = self.a[0];
        let last = self.a.pop().unwrap();
        if !self.a.is_empty() {
            self.a[0] = last;
            self.min_heapify(0);
        }
        Some(min)
    }
    fn min_heapify(&mut self, mut i: usize) {
        loop {
            let l = (i << 1) + 1;
            let r = l + 1;
            let mut min = i;
            if l < self.a.len() && self.a[l].t < self.a[min].t {
                min = l;
            }
            if r < self.a.len() && self.a[r].t < self.a[min].t {
                min = r;
            }
            if min == i {
                break;
            }
            self.a.swap(i, min);
            i = min;
        }
    }
}

fn abs_i32(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

// Emulate the C comparator semantics: it returns (a->x - b->x) if nonzero else (a->y - b->y).
// In C, subtraction can overflow (UB). Here, inputs are from parsing ints; we assume typical AOJ ranges.
// We still implement using i64 to avoid overflow while preserving ordering intent.
fn xcmp(a: &T, b: &T) -> Ordering {
    let dx = (a.x as i64) - (b.x as i64);
    if dx != 0 {
        return if dx < 0 { Ordering::Less } else { Ordering::Greater };
    }
    let dy = (a.y as i64) - (b.y as i64);
    if dy < 0 { Ordering::Less } else if dy > 0 { Ordering::Greater } else { Ordering::Equal }
}
fn ycmp(a: &T, b: &T) -> Ordering {
    let dy = (a.y as i64) - (b.y as i64);
    if dy != 0 {
        return if dy < 0 { Ordering::Less } else { Ordering::Greater };
    }
    let dx = (a.x as i64) - (b.x as i64);
    if dx < 0 { Ordering::Less } else if dx > 0 { Ordering::Greater } else { Ordering::Equal }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    // Mirrors C in(): reads a non-negative integer, assuming next char is a digit.
    fn in_u32(&mut self) -> i32 {
        let mut n: i32 = 0;
        // skip nothing; C doesn't skip, but input format provides digits.
        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
            self.idx += 1;
        }
        n
    }
}

fn dijkstra(
    start: usize,
    flag: i32,
    goal: usize,
    to: &Vec<[i32; 5]>,
    dt: &mut Vec<[i32; 5]>,
    hi: &mut Vec<i32>,
    used: &mut Vec<u8>,
) -> i64 {
    let mut heap = MinHeap::new();
    heap.clear();
    heap.push(start as i32, 0);

    if flag < 0 {
        let hs = hi[start] as usize;
        for i in 0..hs {
            if to[start][i] as usize == start + 1 {
                dt[start][i] = INF;
                hi[start + 1] = 0;
            }
        }
    }

    while !heap.is_empty() {
        let q = heap.pop_min().unwrap();
        let s = q.s as usize;
        let d = q.t;
        if s == goal || s == goal + 1 {
            return d;
        }
        if used[s] != 0 {
            continue;
        }
        used[s] = 1;
        let hs = hi[s] as usize;
        for i in 0..hs {
            let e = to[s][i] as usize;
            if used[e] != 0 {
                continue;
            }
            heap.push(e as i32, d + (dt[s][i] as i64));
        }
    }
    -1
}

fn main() {
    let mut sc = Scanner::new();
    let m = sc.in_u32();
    let n = sc.in_u32();
    let mut k = sc.in_u32() as usize;

    let mut sw: Vec<T> = Vec::with_capacity(MAXSW);
    let mut sz: i32 = 0;
    let mut s: i32 = -1;
    let mut g: i32 = -1;

    for _ in 0..k {
        let x = sc.in_u32();
        let y = sc.in_u32();
        sw.push(T { x, y, id: sz });
        if x == 1 && y == 1 {
            s = sz;
        } else if x == m && y == n {
            g = sz;
        }
        sz += 2;
    }

    let ss = s;
    if s < 0 {
        sw.push(T { x: 1, y: 1, id: sz });
        s = sz;
        k += 1;
        sz += 2;
    }
    if g < 0 {
        sw.push(T { x: m, y: n, id: sz });
        g = sz;
        k += 1;
        sz += 2;
    }

    // Graph storage: fixed 5 edges per node as in C.
    let mut to: Vec<[i32; 5]> = vec![[0; 5]; MAXV];
    let mut dt: Vec<[i32; 5]> = vec![[0; 5]; MAXV];
    let mut hi: Vec<i32> = vec![0; MAXV];
    let mut used: Vec<u8> = vec![0; MAXV];

    // sort by x then y
    sw.sort_by(xcmp);

    // initialize first switch's internal edge
    {
        let a = sw[0].id as usize;
        let b = a + 1;
        to[a][0] = b as i32;
        dt[a][0] = 1;
        hi[a] = 1;
        to[b][0] = a as i32;
        dt[b][0] = 1;
        hi[b] = 1;
    }

    for i in 1..k {
        let a0 = sw[i].id as usize;
        let b0 = a0 + 1;

        // internal edge a<->b with cost 1
        let j = hi[a0] as usize;
        hi[a0] += 1;
        to[a0][j] = b0 as i32;
        dt[a0][j] = 1;

        let j = hi[b0] as usize;
        hi[b0] += 1;
        to[b0][j] = a0 as i32;
        dt[b0][j] = 1;

        // connect consecutive with same x using ids (not +1)
        if sw[i - 1].x == sw[i].x {
            let a = sw[i - 1].id as usize;
            let b = sw[i].id as usize;
            let d = abs_i32(sw[i - 1].y - sw[i].y);

            let j = hi[a] as usize;
            hi[a] += 1;
            to[a][j] = b as i32;
            dt[a][j] = d;

            let j = hi[b] as usize;
            hi[b] += 1;
            to[b][j] = a as i32;
            dt[b][j] = d;
        }
    }

    // sort by y then x
    sw.sort_by(ycmp);
    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
            let a = (sw[i - 1].id + 1) as usize;
            let b = (sw[i].id + 1) as usize;
            let d = abs_i32(sw[i - 1].x - sw[i].x);

            let j = hi[a] as usize;
            hi[a] += 1;
            to[a][j] = b as i32;
            dt[a][j] = d;

            let j = hi[b] as usize;
            hi[b] += 1;
            to[b][j] = a as i32;
            dt[b][j] = d;
        }
    }

    let ans = dijkstra(s as usize, ss, g as usize, &to, &mut dt, &mut hi, &mut used);
    print!("{ans}\n");
}
