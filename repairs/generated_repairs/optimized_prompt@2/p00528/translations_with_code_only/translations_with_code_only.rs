// AOJ 0605: Modern Mansion
// 2018.1.10 bal4u@uu

use std::io::{self, BufRead};
use std::convert::TryInto;

const QMAX: usize = 80000;
const INF: i32 = 0x7ffffff;

#[derive(Clone)]
struct QUE {
    s: usize,
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
        self.que[0] = self.que[self.qsize - 1].clone();
        self.qsize -= 1;
        self.min_heapify(0);
    }

    fn enq(&mut self, s: usize, t: i64) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { s, t });
        let mut i = i;

        while i > 0 && self.que[self.parent(i)].t > self.que[i].t {
            self.que.swap(i, self.parent(i));
            i = self.parent(i);
        }
    }
}

#[derive(Clone)]
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
    let mut input = handle.lines().next().unwrap().unwrap();
    let mut n = 0;
    let mut c = input.chars().next().unwrap() as i32;

    while c >= '0' as i32 {
        n = (n << 3) + (n << 1) + (c & 0xf);
        input.remove(0);
        if input.is_empty() {
            break;
        }
        c = input.chars().next().unwrap() as i32;
    }
    n
}

fn dijkstra(start: usize, flag: i32, goal: usize, to: &mut Vec<Vec<usize>>, dt: &mut Vec<Vec<i32>>, hi: &mut Vec<usize>, used: &mut Vec<u8>, heap: &mut MinHeap) -> i64 {
    let mut d: i64;
    heap.enq(start, 0);
    
    if flag < 0 {
        for i in 0..hi[start] {
            if to[start][i] == start + 1 {
                dt[start][i] = INF;
                hi[start + 1] = 0;
            }
        }
    }

    while heap.qsize > 0 {
        let s = heap.que[0].s;
        d = heap.que[0].t;
        heap.deq();

        if s == goal || s == goal + 1 {
            return d;
        }
        if used[s] != 0 {
            continue;
        }
        used[s] = 1;

        for i in 0..hi[s] {
            let e = to[s][i];
            if used[e] != 0 {
                continue;
            }
            heap.enq(e, d + dt[s][i] as i64);
        }
    }
    -1
}

fn main() {
    let m = in_number();
    let n = in_number();
    let k = in_number();
    let mut sz = 0;
    let mut s = usize::MAX;
    let mut g = usize::MAX;

    let mut sw = vec![T { x: 0, y: 0, id: 0 }; 200005];
    let mut to = vec![vec![0; 5]; 400005];
    let mut dt = vec![vec![0; 5]; 400005];
    let mut hi = vec![0; 400005];
    let mut used = vec![0; 400005];

    for i in 0..k {
        let x = in_number();
        let y = in_number();
        sw[i].x = x;
        sw[i].y = y;
        sw[i].id = sz;
        if x == 1 && y == 1 {
            s = sz;
        } else if x == m && y == n {
            g = sz;
        }
        sz += 1; 
    }

    let mut ss = s;
    if s == usize::MAX {
        sw[sz].x = 1;
        sw[sz].y = 1;
        sw[sz].id = sz;
        s = sz;
        sz += 1; 
    }
    if g == usize::MAX {
        sw[sz].x = m;
        sw[sz].y = n;
        sw[sz].id = sz;
        g = sz;
        sz += 1; 
    }

    sw[0..sz].sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    let mut a = sw[0].id;
    let mut b = a + 1;
    to[a][0] = b;
    dt[a][0] = 1;
    hi[a] = 1;
    to[b][0] = a;
    dt[b][0] = 1;
    hi[b] = 1;

    for i in 1..sz {
        a = sw[i].id;
        b = a + 1;
        let j = hi[a];
        to[a][j] = b;
        dt[a][j] = 1;
        hi[a] += 1;

        let j = hi[b];
        to[b][j] = a;
        dt[b][j] = 1;
        hi[b] += 1;

        if sw[i - 1].x == sw[i].x {
            a = sw[i - 1].id;
            b = sw[i].id;
            let d = abs(sw[i - 1].y - sw[i].y);
            let j = hi[a];
            to[a][j] = b;
            dt[a][j] = d;
            hi[a] += 1;

            let j = hi[b];
            to[b][j] = a;
            dt[b][j] = d;
            hi[b] += 1;
        }
    }

    sw[0..sz].sort_by(|a, b| a.y.cmp(&b.y));
    for i in 1..sz {
        if sw[i - 1].y == sw[i].y {
            a = sw[i - 1].id + 1;
            b = sw[i].id + 1;
            let d = abs(sw[i - 1].x - sw[i].x);
            let j = hi[a];
            to[a][j] = b;
            dt[a][j] = d;
            hi[a] += 1;

            let j = hi[b];
            to[b][j] = a;
            dt[b][j] = d;
            hi[b] += 1;
        }
    }

    let mut heap = MinHeap::new();
    let result = dijkstra(s, ss.try_into().unwrap(), g, &mut to, &mut dt, &mut hi, &mut used, &mut heap);
    println!("{}", result);
}
