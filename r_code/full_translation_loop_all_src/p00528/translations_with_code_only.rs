use std::cmp::Ordering;
use std::io::{self, Read};

const QMAX: usize = 80000;
const INF: i32 = 0x7ffffff;

#[derive(Clone, Copy)]
struct QueItem {
    s: usize,
    t: i64,
}

#[derive(Clone, Copy)]
struct T {
    x: i32,
    y: i32,
    id: usize,
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
    // Matches the C in(): reads a non-negative integer, assuming input is well-formed.
    fn in_u32(&mut self) -> u32 {
        // Skip any non-digit (C code doesn't, but AOJ input is digit-starting; this is harmless).
        while self.idx < self.buf.len() && (self.buf[self.idx] < b'0' || self.buf[self.idx] > b'9') {
            self.idx += 1;
        }
        let mut n: u32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            self.idx += 1;
        }
        n
    }
}

fn abs_i32(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

struct MinHeap {
    data: Vec<QueItem>,
}
impl MinHeap {
    fn new() -> Self {
        Self { data: Vec::with_capacity(QMAX) }
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn push(&mut self, s: usize, t: i64) {
        // C code assumes capacity; we mimic by allowing growth (input constraints should fit).
        self.data.push(QueItem { s, t });
        let mut i = self.data.len() - 1;
        while i > 0 {
            let p = (i - 1) >> 1;
            if self.data[p].t <= self.data[i].t {
                break;
            }
            self.data.swap(i, p);
            i = p;
        }
    }
    fn pop_min(&mut self) -> Option<QueItem> {
        if self.data.is_empty() {
            return None;
        }
        let min = self.data[0];
        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            self.min_heapify(0);
        }
        Some(min)
    }
    fn min_heapify(&mut self, i: usize) {
        let n = self.data.len();
        let mut min = i;
        let l = (i << 1) + 1;
        let r = (i << 1) + 2;
        if l < n && self.data[l].t < self.data[min].t {
            min = l;
        }
        if r < n && self.data[r].t < self.data[min].t {
            min = r;
        }
        if min != i {
            self.data.swap(i, min);
            self.min_heapify(min);
        }
    }
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
        // Mirrors:
        // for (i = 0; i < hi[start]; i++) if (to[start][i] == start+1) {
        //     dt[start][i] = INF, hi[start+1] = 0;
        // }
        let hs = hi[start];
        for i in 0..hs {
            if to[start][i] == start + 1 {
                dt[start][i] = INF;
                hi[start + 1] = 0;
            }
        }
    }

    while !heap.is_empty() {
        let QueItem { s, t: d } = heap.pop_min().unwrap();
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
    let m = sc.in_u32() as i32;
    let n = sc.in_u32() as i32;
    let mut k = sc.in_u32() as usize;

    // sw has capacity for k plus possible added start/goal.
    let mut sw: Vec<T> = Vec::with_capacity(k + 2);

    let mut sz: usize = 0;
    let mut s: i32 = -1;
    let mut g: i32 = -1;

    for _ in 0..k {
        let x = sc.in_u32() as i32;
        let y = sc.in_u32() as i32;
        sw.push(T { x, y, id: sz });
        if x == 1 && y == 1 {
            s = sz as i32;
        } else if x == m && y == n {
            g = sz as i32;
        }
        sz += 2;
    }

    let ss = s; // original s before possibly adding (1,1)

    if s < 0 {
        sw.push(T { x: 1, y: 1, id: sz });
        s = sz as i32;
        k += 1;
        sz += 2;
    }
    if g < 0 {
        sw.push(T { x: m, y: n, id: sz });
        g = sz as i32;
        k += 1;
        sz += 2;
    }

    // Graph storage: node count is sz (ids are even, plus odd partner).
    let node_count = sz;
    let mut to: Vec<[usize; 5]> = vec![[0usize; 5]; node_count];
    let mut dt: Vec<[i32; 5]> = vec![[0i32; 5]; node_count];
    let mut hi: Vec<usize> = vec![0usize; node_count];
    let mut used: Vec<u8> = vec![0u8; node_count];

    // sort by x then y (like xcmp)
    sw.sort_by(|a, b| {
        let dx = a.x.wrapping_sub(b.x);
        if dx != 0 {
            // C comparator returns a->x - b->x (may overflow UB in C, but inputs are small).
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });

    // Initialize first switch's pair edge
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

        let j = hi[a0];
        hi[a0] += 1;
        to[a0][j] = b0;
        dt[a0][j] = 1;

        let j = hi[b0];
        hi[b0] += 1;
        to[b0][j] = a0;
        dt[b0][j] = 1;

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

    // sort by y then x (like ycmp)
    sw.sort_by(|a, b| {
        let dy = a.y.wrapping_sub(b.y);
        if dy != 0 {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });

    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
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
