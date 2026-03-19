// AOJ 0275 Railroad
// 2018.2.21 bal4u

use std::io::{self, Read};

const QMAX: usize = 200_000;
const INF: i32 = 0x10101010;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    s: usize,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: vec![QUE { t: 0, s: 0 }; QMAX + 2],
            qsize: 0,
        }
    }

    fn parent(i: usize) -> usize {
        i >> 1
    }

    fn left(i: usize) -> usize {
        i << 1
    }

    fn right(i: usize) -> usize {
        (i << 1) + 1
    }

    fn min_heapify(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut min = if l < self.qsize && self.que[l].t < self.que[i].t {
            l
        } else {
            i
        };
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

    fn enq(&mut self, s: usize, t: i32) {
        let mut i = self.qsize;
        self.qsize += 1;
        self.que[i] = QUE { s, t };
        while i > 0 && self.que[Self::parent(i)].t > self.que[i].t {
            self.que.swap(i, Self::parent(i));
            i = Self::parent(i);
        }
    }
}

fn in_int() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn dijkstra(start: usize, hi: &[usize], to: &[Vec<usize>], w: &[Vec<i32>], dist: &mut [i32], heap: &mut MinHeap) {
    dist.fill(INF);
    dist[start] = 0;
    heap.enq(start, 0);
    while heap.qsize > 0 {
        let s = heap.que[0].s;
        heap.deq();
        for i in 0..hi[s] {
            let e = to[s][i];
            let d = dist[s] + w[s][i];
            if dist[e] > d {
                dist[e] = d;
                heap.enq(e, d);
            }
        }
    }
}

fn tsort(start: usize, goal: usize, hi: &[usize], to: &[Vec<usize>], dist: &[i32], hi2: &mut Vec<usize>, to2: &mut Vec<Vec<usize>>, mk: &mut Vec<u8>, ord: &mut Vec<usize>, cnt: &mut usize, deg: &mut Vec<usize>, w: &[Vec<i32>]) {
    mk.fill(0);
    let mut q = vec![goal];
    let mut top = 0;
    let mut tail = 1;

    while top < tail {
        let s = q[top];
        top += 1;
        if mk[s as usize] != 0 {
            continue;
        }
        for i in 0..hi[s] {
            let e = to[s][i];
            if dist[s] == dist[e] + w[s][i] {
                q.push(e);
                to2[e].push(s);
                hi2[e] += 1;
            }
        }
        mk[s as usize] = 1;
    }

    *cnt = 0;
    for s in 0..hi2.len() {
        for &e in &to2[s] {
            deg[e] += 1;
        }
    }

    q.clear();
    q.push(start);
    top = 0;
    tail = 1;

    while top < tail {
        let s = q[top];
        top += 1;
        ord.push(s);
        for &e in &to2[s] {
            if deg[e] > 0 {
                deg[e] -= 1;
                if deg[e] == 0 {
                    q.push(e);
                }
            }
        }
    }
}

fn main() {
    let mut hi = vec![0; 100_002];
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); 100_002];
    let mut w: Vec<Vec<i32>> = vec![Vec::new(); 100_002];
    let mut dist = vec![0; 100_002];
    let mut mk = vec![0; 100_002];
    let mut c = vec![0; 40_002];
    let mut d = vec![0; 40_002];
    let mut hi2 = vec![0; 100_002];
    let mut to2: Vec<Vec<usize>> = vec![Vec::new(); 100_002];
    let mut deg = vec![0; 100_002];
    let mut ord = Vec::new();
    let mut cnt = 0;
    let mut dp = vec![0u64; 100_002];

    let S = in_int();
    let R = in_int();
    let mut memo = vec![0; (R * 3) as usize];
    let mut j = 0;

    for _ in 0..R {
        let u = in_int() - 1;
        hi[u as usize] += 1;
        memo[j] = u;
        j += 1;
        let v = in_int() - 1;
        hi[v as usize] += 1;
        memo[j] = v;
        j += 1;
        memo[j] = in_int();
        j += 1;
    }

    for i in 0..S {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
            w[i] = vec![0; hi[i]];
            to2[i] = vec![0; hi[i]];
        }
    }

    j = 0;
    while R > 0 {
        let u = memo[j];
        j += 1;
        let v = memo[j];
        j += 1;
        let _w = memo[j];
        j += 1;
        to[u as usize].push(v);
        w[u as usize].push(_w);
        to[v as usize].push(u);
        w[v as usize].push(_w);
        R -= 1;
    }

    let a = in_int() - 1;
    let b = in_int() - 1;
    let Q = in_int();

    for i in 0..Q {
        c[i] = in_int() - 1;
        d[i] = in_int() - 1;
    }

    let mut heap = MinHeap::new();
    dijkstra(a as usize, &hi, &to, &w, &mut dist, &mut heap);
    tsort(a as usize, b as usize, &hi, &to, &dist, &mut hi2, &mut to2, &mut mk, &mut ord, &mut cnt, &mut deg, &w);

    let ii = (Q + 63) >> 6;
    for i in 0..ii {
        dp.fill(0);
        for j in 0..64 {
            if (i << 6) + j < Q {
                dp[c[(i << 6) + j]] |= 1 << j;
            }
        }
        for j in 0..cnt {
            let u = ord[j];
            for k in 0..hi2[u] {
                dp[to2[u][k]] |= dp[u];
            }
        }
        for j in 0..64 {
            if (i << 6) + j < Q {
                println!("{}", if dp[d[(i << 6) + j]] & (1 << j) != 0 { "Yes" } else { "No" });
            }
        }
    }
}
