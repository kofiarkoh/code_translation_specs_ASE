// AOJ 0275 Railroad
// 2018.2.21 bal4u

use std::io::{self, Read};

const QMAX: usize = 200000;
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

    fn min_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
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
        let i = self.qsize;
        self.qsize += 1;
        self.que[i] = QUE { s: s as i32, t };
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

    fn parent(&self, i: usize) -> usize {
        i >> 1
    }

    fn left(&self, i: usize) -> usize {
        i << 1
    }

    fn right(&self, i: usize) -> usize {
        (i << 1) + 1
    }
}

fn in_int() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    iter.next().unwrap().parse().unwrap()
}

fn dijkstra(start: usize, hi: &[usize], to: &[Vec<usize>], w: &[Vec<i32>], dist: &mut [i32], heap: &mut MinHeap) {
    dist.fill(INF);
    heap.enq(start, 0);
    dist[start] = 0;

    while heap.qsize > 0 {
        let s = heap.que[0].s as usize;
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

fn tsort(start: usize, goal: usize, hi: &[usize], to: &[Vec<usize>], w: &[Vec<i32>], dist: &[i32], ord: &mut Vec<usize>, deg: &mut Vec<usize>, hi2: &mut Vec<usize>, to2: &mut Vec<Vec<usize>>, mk: &mut Vec<u8>) {
    mk.fill(0);
    let mut q = vec![goal];
    let mut top = 0;
    let mut tail = 1;

    while top < tail {
        let s = q[top];
        top += 1;
        if mk[s] != 0 {
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
        mk[s] = 1;
    }

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
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); S];
    let mut w: Vec<Vec<i32>> = vec![Vec::new(); S];
    let mut to2: Vec<Vec<usize>> = vec![Vec::new(); S];
    let mut hi2 = vec![0; S];
    let mut deg = vec![0; S];
    let mut ord = Vec::new();
    let mut mk = vec![0; S];
    let mut dist = vec![0; S];
    let mut c = vec![0; 40002];
    let mut d = vec![0; 40002];

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
        let weight = in_int();
        memo[j] = weight;
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
        R -= 1;
        let u = memo[j];
        j += 1;
        let v = memo[j];
        j += 1;
        let _w = memo[j];
        j += 1;
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
    dijkstra(a, &hi, &to, &w, &mut dist, &mut heap);
    tsort(a, b, &hi, &to, &w, &dist, &mut ord, &mut deg, &mut hi2, &mut to2, &mut mk);

    let ii = (Q + 63) >> 6;
    for i in 0..ii {
        let mut dp = vec![0u64; S];
        for j in 0..64 {
            if (i << 6) + j < Q {
                dp[c[(i << 6) + j]] |= 1 << j;
            }
        }
        for &u in &ord {
            for &k in &to2[u] {
                dp[k] |= dp[u];
            }
        }
        for j in 0..64 {
            if (i << 6) + j < Q {
                println!("{}", if dp[d[(i << 6) + j]] & (1 << j) != 0 { "Yes" } else { "No" });
            }
        }
    }
}
