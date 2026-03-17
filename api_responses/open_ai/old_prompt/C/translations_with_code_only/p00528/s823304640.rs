// AOJ 0605: Modern Mansion
// 2018.1.10 bal4u@uu

use std::cmp::Ordering;
use std::io::{self, BufRead};

const QMAX: usize = 80000;
const INF: i32 = 0x7ffffff;

#[derive(Clone, Copy)]
struct QUE {
    s: i32,
    t: i64,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(QMAX),
            qsize: 0,
        }
    }

    fn parent(&self, i: usize) -> usize {
        i >> 1
    }

    fn left(&self, i: usize) -> usize {
        i << 1
    }

    fn right(&self, i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut min = i;

        if l < self.qsize && self.que[l].t < self.que[min].t {
            min = l;
        }
        if r < self.qsize && self.que[r].t < self.que[min].t {
            min = r;
        }
        if min != i {
            self.que.swap(i, min);
            self.min_heapify(min);
        }
    }

    fn deq(&mut self) {
        self.que[0] = self.que[self.qsize - 1];
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: i32, t: i64) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { s, t });
        let mut i = i;

        while i > 0 {
            let min = self.parent(i);
            if self.que[min].t > self.que[i].t {
                self.que.swap(i, min);
                i = min;
            } else {
                break;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct T {
    x: i32,
    y: i32,
    id: usize,
}

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn dijkstra(start: usize, flag: i32, goal: usize, hi: &[usize], to: &[Vec<usize>], dt: &[Vec<i64>], used: &mut Vec<bool>, heap: &mut MinHeap) -> i64 {
    let mut d: i64;
    heap.enq(start as i32, 0);

    if flag < 0 {
        for i in 0..hi[start] {
            if to[start][i] == start + 1 {
                // Set distance to INF
                dt[start][i] = INF as i64;
                hi[start + 1] = 0;
            }
        }
    }

    while heap.qsize > 0 {
        let s = heap.que[0].s as usize;
        d = heap.que[0].t;
        heap.deq();

        if s == goal || s == goal + 1 {
            return d;
        }
        if used[s] {
            continue;
        }
        used[s] = true;

        for i in 0..hi[s] {
            let e = to[s][i];
            if used[e] {
                continue;
            }
            heap.enq(e as i32, d + dt[s][i]);
        }
    }
    -1
}

fn main() {
    let m = in_number();
    let n = in_number();
    let k = in_number();
    let mut sz = 0;
    let mut s = -1;
    let mut g = -1;

    let mut sw = vec![T { x: 0, y: 0, id: 0 }; 200005];
    let mut to = vec![vec![0; 5]; 400005];
    let mut dt = vec![vec![0; 5]; 400005];
    let mut hi = vec![0; 400005];
    let mut used = vec![false; 400005];

    for i in 0..k {
        let x = in_number();
        let y = in_number();
        sw[i] = T { x, y, id: sz };
        if x == 1 && y == 1 {
            s = sz;
        } else if x == m && y == n {
            g = sz;
        }
        sz += 2;
    }

    let ss = s;
    if s < 0 {
        sw[k] = T { x: 1, y: 1, id: sz };
        s = sz;
        sz += 2;
    }
    if g < 0 {
        sw[k] = T { x: m, y: n, id: sz };
        g = sz;
        sz += 2;
    }

    sw[0..k].sort_by(|a, b| {
        if a.x != b.x {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });

    let mut a = sw[0].id;
    let mut b = a + 1;
    to[a][0] = b;
    dt[a][0] = 1;
    hi[a] = 1;
    to[b][0] = a;
    dt[b][0] = 1;
    hi[b] = 1;

    for i in 1..k {
        a = sw[i].id;
        b = a + 1;
        let j = hi[a];
        hi[a] += 1;
        to[a][j] = b;
        dt[a][j] = 1;
        let j = hi[b];
        hi[b] += 1;
        to[b][j] = a;
        dt[b][j] = 1;

        if sw[i - 1].x == sw[i].x {
            let d = abs(sw[i - 1].y - sw[i].y);
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

    sw[0..k].sort_by(|a, b| {
        if a.y != b.y {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });

    for i in 1..k {
        if sw[i - 1].y == sw[i].y {
            a = sw[i - 1].id + 1;
            b = sw[i].id + 1;
            let d = abs(sw[i - 1].x - sw[i].x);
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

    let mut heap = MinHeap::new();
    let result = dijkstra(s as usize, ss, g as usize, &hi, &to, &dt, &mut used, &mut heap);
    println!("{}", result);
}
