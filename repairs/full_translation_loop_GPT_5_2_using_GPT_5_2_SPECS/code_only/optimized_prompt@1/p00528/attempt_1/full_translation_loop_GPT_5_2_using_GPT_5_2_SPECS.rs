use std::cmp::Ordering;
use std::io::{self, Read};

const QMAX: usize = 80000;
const INF: i32 = 0x7ffffff;

#[derive(Clone, Copy)]
struct Que {
    s: usize,
    t: i64,
}

#[derive(Clone, Copy)]
struct T {
    x: i32,
    y: i32,
    id: usize,
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
    fn push(&mut self, s: usize, t: i64) {
        // Binary heap (min-heap) with the same ordering as the C code: compare by t only.
        let mut i = self.a.len();
        self.a.push(Que { s, t });
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
        let res = self.a[0];
        let last = self.a.pop().unwrap();
        if !self.a.is_empty() {
            self.a[0] = last;
            self.min_heapify(0);
        }
        Some(res)
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
    fn in_u32(&mut self) -> i32 {
        // Mirrors the C in(): assumes next token is a non-negative integer.
        // It does not skip non-digits robustly; AOJ input is well-formed.
        let mut n: i32 = 0;
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;
        loop {
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn abs_i32(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn dijkstra(
    start: usize,
    flag: i32,
    goal: usize,
    to: &mut Vec<[usize; 5]>,
    dt: &mut Vec<[i32; 5]>,
    hi: &mut Vec<usize>,
    used: &mut Vec<u8>,
) -> i64 {
    let mut heap = MinHeap::new();
    heap.clear();
    heap.push(start, 0);

    if flag < 0 {
        // C code:
        // for (i = 0; i < hi[start]; i++) if (to[start][i] == start+1) {
        //     dt[start][i] = INF, hi[start+1] = 0;
        // }
        // Note: this is a very specific mutation; preserve it exactly.
        let hs = hi[start];
        for i in 0..hs {
            if to[start][i] == start + 1 {
                dt[start][i] = INF;
                hi[start + 1] = 0;
            }
        }
    }

    while !heap.is_empty() {
        let Que { s, t: d } = heap.pop_min().unwrap();
        if s == goal || s == goal + 1 {
            return d;
        }
        if used[s] != 0 {
            continue;
        }
        used[s] = 1;
        let hs = hi[s];
        for i in 0..hs {
            let e = to[s][i];
            if used[e] != 0 {
                continue;
            }
            heap.push(e, d + dt[s][i] as i64);
        }
    }
    -1
}

fn main() {
    let mut sc = Scanner::new();
    let m = sc.in_u32();
    let n = sc.in_u32();
    let mut k = sc.in_u32() as usize;

    let mut sw: Vec<T> = vec![T { x: 0, y: 0, id: 0 }; 200005];
    let mut sz: usize = 0;
    let mut s: i32 = -1;
    let mut g: i32 = -1;

    for i in 0..k {
        let x = sc.in_u32();
        let y = sc.in_u32();
        sw[i] = T { x, y, id: sz };
        if x == 1 && y == 1 {
            s = sz as i32;
        } else if x == m && y == n {
            g = sz as i32;
        }
        sz += 2;
    }

    let ss = s;
    if s < 0 {
        sw[k] = T { x: 1, y: 1, id: sz };
        s = sz as i32;
        k += 1;
        sz += 2;
    }
    if g < 0 {
        sw[k] = T { x: m, y: n, id: sz };
        g = sz as i32;
        k += 1;
        sz += 2;
    }

    // Graph storage: fixed max degree 5 per node, as in the C arrays.
    let mut to: Vec<[usize; 5]> = vec![[0; 5]; 400005];
    let mut dt: Vec<[i32; 5]> = vec![[0; 5]; 400005];
    let mut hi: Vec<usize> = vec![0; 400005];
    let mut used: Vec<u8> = vec![0; 400005];

    // Sort by x then y (C comparator uses subtraction; here use total ordering).
    sw[..k].sort_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Equal => a.y.cmp(&b.y),
        other => other,
    });

    // Initialize first switch's internal edge (id <-> id+1) with cost 1.
    {
        let a = sw[0].id;
        let b = a + 1;
        to[a][0] = b;
        dt[a][0] = 1;
        hi[a] = 1;
        to[b][0] = a;
        dt[b][0] = 1;
        hi[b] = 1;
    }

    for i in 1..k {
        let a0 = sw[i].id;
        let b0 = a0 + 1;

        // Internal edge for each switch.
        let j = hi[a0];
        hi[a0] += 1;
        to[a0][j] = b0;
        dt[a0][j] = 1;

        let j = hi[b0];
        hi[b0] += 1;
        to[b0][j] = a0;
        dt[b0][j] = 1;

        // If same x, connect ids (not +1) with |dy|.
        if sw[i - 1].x == sw[i].x {
            let a = sw[i - 1].id;
            let b = sw[i].id;
            let d = abs_i32(sw[i - 1].y - sw[i].y);

            let j = hi[a];
            hi[a] += 1;
            to[a][j] = b;
            dt[a][j] = d;

            let j = hi[b];
            hi[b] += 1;
            to[b][j] = a;
            dt[b][j] = d;
        }
    }

    // Sort by y then x.
    sw[..k].sort_by(|a, b| match a.y.cmp(&b.y) {
        Ordering::Equal => a.x.cmp(&b.x),
        other => other,
    });

    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
            // Connect (id+1) nodes with |dx|.
            let a = sw[i - 1].id + 1;
            let b = sw[i].id + 1;
            let d = abs_i32(sw[i - 1].x - sw[i].x);

            let j = hi[a];
            hi[a] += 1;
            to[a][j] = b;
            dt[a][j] = d;

            let j = hi[b];
            hi[b] += 1;
            to[b][j] = a;
            dt[b][j] = d;
        }
    }

    let ans = dijkstra(
        s as usize,
        ss,
        g as usize,
        &mut to,
        &mut dt,
        &mut hi,
        &mut used,
    );
    print!("{ans}\n");
}
