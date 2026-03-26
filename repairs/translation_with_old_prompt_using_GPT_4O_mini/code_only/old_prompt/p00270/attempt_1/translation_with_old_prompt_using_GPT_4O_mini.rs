// AOJ 0275 Railroad
// 2018.2.21 bal4u

use std::io::{self, Read};
use std::ptr;

const QMAX: usize = 200_000;
const INF: i32 = 0x10101010;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    s: i32,
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

    fn enq(&mut self, s: i32, t: i32) {
        let i = self.qsize;
        self.qsize += 1;
        self.que[i] = QUE { s, t };
        let mut i = i;
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

fn dijkstra(start: usize, dist: &mut Vec<i32>, heap: &mut MinHeap, to: &Vec<Vec<usize>>, w: &Vec<Vec<i32>>, hi: &Vec<usize>) {
    dist.fill(INF);
    dist[start] = 0;
    heap.enq(start as i32, 0);
    while heap.qsize > 0 {
        let s = heap.que[0].s as usize;
        heap.deq();
        for i in 0..hi[s] {
            let e = to[s][i];
            let d = dist[s] + w[s][i];
            if dist[e] > d {
                dist[e] = d;
                heap.enq(e as i32, d);
            }
        }
    }
}

fn tsort(start: usize, goal: usize, hi: &Vec<usize>, to: &Vec<Vec<usize>>, w: &Vec<Vec<i32>>, dist: &Vec<i32>, hi2: &mut Vec<usize>, to2: &mut Vec<Vec<usize>>, ord: &mut Vec<usize>, deg: &mut Vec<usize>) {
    let mut mk = vec![0; hi.len()];
    let mut q = vec![0; hi.len()];
    let mut top = 0;
    let mut tail = 1;
    q[0] = goal;

    while top < tail {
        let s = q[top];
        top += 1;
        if mk[s] != 0 {
            continue;
        }
        for i in 0..hi[s] {
            let e = to[s][i];
            if dist[s] == dist[e] + w[s][i] {
                q[tail] = e;
                tail += 1;
                to2[e][hi2[e]] = s;
                hi2[e] += 1;
            }
        }
        mk[s] = 1;
    }

    let mut cnt = 0;
    for s in 0..hi.len() {
        for i in 0..hi2[s] {
            deg[to2[s][i]] += 1;
        }
    }
    q[0] = start;
    top = 0;
    tail = 1;

    while top < tail {
        let s = q[top];
        top += 1;
        ord[cnt] = s;
        cnt += 1;
        for i in 0..hi2[s] {
            let e = to2[s][i];
            if deg[e] > 0 {
                deg[e] -= 1;
                if deg[e] == 0 {
                    q[tail] = e;
                    tail += 1;
                }
            }
        }
    }
}

fn main() {
    let S = in_int() as usize;
    let R = in_int() as usize;

    let mut hi = vec![0; S];
    let mut memo = vec![0; R * 3];
    let mut j = 0;

    for _ in 0..R {
        let u = in_int() - 1;
        hi[u] += 1;
        memo[j] = u;
        j += 1;
        let v = in_int() - 1;
        hi[v] += 1;
        memo[j] = v;
        j += 1;
        memo[j] = in_int();
        j += 1;
    }

    let mut to: Vec<Vec<usize>> = vec![vec![0; hi[i]]; S];
    let mut w: Vec<Vec<i32>> = vec![vec![0; hi[i]]; S];
    let mut to2: Vec<Vec<usize>> = vec![vec![0; hi[i]]; S];
    let mut hi2 = vec![0; S];
    let mut deg = vec![0; S];
    let mut ord = vec![0; S];
    let mut dist = vec![0; S];
    let mut c = vec![0; 40_002];
    let mut d = vec![0; 40_002];

    for i in 0..R {
        let u = memo[i * 3];
        let v = memo[i * 3 + 1];
        let _w = memo[i * 3 + 2];
        let k = hi[u] - 1;
        to[u][k] = v;
        w[u][k] = _w;
        let k = hi[v] - 1;
        to[v][k] = u;
        w[v][k] = _w;
    }

    let a = in_int() - 1;
    let b = in_int() - 1;
    let Q = in_int();

    for i in 0..Q {
        c[i] = in_int() - 1;
        d[i] = in_int() - 1;
    }

    let mut heap = MinHeap::new();
    dijkstra(a, &mut dist, &mut heap, &to, &w, &hi);
    tsort(a, b, &hi, &to, &w, &dist, &mut hi2, &mut to2, &mut ord, &mut deg);

    let ii = (Q + 63) >> 6;
    let mut dp = vec![0u64; S];

    for i in 0..ii {
        dp.fill(0);
        for j in 0..64 {
            if (i << 6) + j < Q {
                dp[c[(i << 6) + j]] |= 1 << j;
            }
        }
        for j in 0..ord.len() {
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
