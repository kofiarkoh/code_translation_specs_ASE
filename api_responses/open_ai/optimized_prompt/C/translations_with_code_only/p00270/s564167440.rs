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

struct Graph {
    hi: Vec<usize>,
    to: Vec<Vec<i32>>,
    w: Vec<Vec<i32>>,
    dist: Vec<i32>,
    mk: Vec<u8>,
    c: Vec<i32>,
    d: Vec<i32>,
    q: Vec<i32>,
    hi2: Vec<usize>,
    to2: Vec<Vec<i32>>,
    deg: Vec<usize>,
    ord: Vec<i32>,
    cnt: usize,
    dp: Vec<u64>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            hi: vec![0; size],
            to: vec![Vec::new(); size],
            w: vec![Vec::new(); size],
            dist: vec![INF; size],
            mk: vec![0; size],
            c: Vec::new(),
            d: Vec::new(),
            q: vec![0; size],
            hi2: vec![0; size],
            to2: vec![Vec::new(); size],
            deg: vec![0; size],
            ord: vec![0; size],
            cnt: 0,
            dp: vec![0; size],
        }
    }

    fn min_heapify(&mut self, que: &mut Vec<QUE>, qsize: &mut usize, i: usize) {
        let l = (i << 1) + 1;
        let r = (i << 1) + 2;
        let mut min = i;

        if l < *qsize && que[l].t < que[min].t {
            min = l;
        }
        if r < *qsize && que[r].t < que[min].t {
            min = r;
        }
        if min != i {
            que.swap(i, min);
            self.min_heapify(que, qsize, min);
        }
    }

    fn deq(&mut self, que: &mut Vec<QUE>, qsize: &mut usize) {
        que[0] = que[*qsize - 1];
        *qsize -= 1;
        self.min_heapify(que, qsize, 0);
    }

    fn enq(&mut self, que: &mut Vec<QUE>, qsize: &mut usize, s: i32, t: i32) {
        let i = *qsize;
        *qsize += 1;
        que.push(QUE { s, t });
        let mut i = i;

        while i > 0 {
            let parent = (i - 1) >> 1;
            if que[parent].t > que[i].t {
                que.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn dijkstra(&mut self, start: usize, que: &mut Vec<QUE>, qsize: &mut usize) {
        self.dist[start] = 0;
        self.enq(que, qsize, start as i32, 0);

        while *qsize > 0 {
            let s = que[0].s;
            self.deq(que, qsize);
            for i in 0..self.hi[s as usize] {
                let e = self.to[s as usize][i as usize];
                let d = self.dist[s as usize] + self.w[s as usize][i as usize];
                if self.dist[e as usize] > d {
                    self.dist[e as usize] = d;
                    self.enq(que, qsize, e, d);
                }
            }
        }
    }

    fn tsort(&mut self, start: usize, goal: usize) {
        self.mk.fill(0);
        self.q[0] = goal as i32;
        let mut top = 0;
        let mut tail = 1;

        while top < tail {
            let s = self.q[top];
            top += 1;
            if self.mk[s as usize] != 0 {
                continue;
            }
            for i in 0..self.hi[s as usize] {
                let e = self.to[s as usize][i as usize];
                if self.dist[s as usize] == self.dist[e as usize] + self.w[s as usize][i as usize] {
                    self.q[tail] = e;
                    tail += 1;
                    self.to2[e as usize].push(s);
                    self.hi2[e as usize] += 1;
                }
            }
            self.mk[s as usize] = 1;
        }

        self.cnt = 0;
        for s in 0..self.hi2.len() {
            for &e in &self.to2[s] {
                self.deg[e as usize] += 1;
            }
        }

        self.q[0] = start as i32;
        top = 0;
        tail = 1;

        while top < tail {
            let s = self.q[top];
            top += 1;
            self.ord[self.cnt] = s;
            self.cnt += 1;
            for &e in &self.to2[s as usize] {
                if self.deg[e as usize] > 0 {
                    self.deg[e as usize] -= 1;
                    if self.deg[e as usize] == 0 {
                        self.q[tail] = e;
                        tail += 1;
                    }
                }
            }
        }
    }
}

fn in_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    iter.next().unwrap().parse::<i32>().unwrap()
}

fn main() {
    let s = in_int() as usize;
    let r = in_int() as usize;

    let mut graph = Graph::new(s);
    let mut memo = Vec::with_capacity(r * 3);
    
    for _ in 0..r {
        let u = in_int() - 1;
        graph.hi[u] += 1;
        memo.push(u);
        let v = in_int() - 1;
        graph.hi[v] += 1;
        memo.push(v);
        let weight = in_int();
        memo.push(weight);
    }

    for i in 0..s {
        if graph.hi[i] > 0 {
            graph.to[i] = vec![0; graph.hi[i]];
            graph.w[i] = vec![0; graph.hi[i]];
            graph.to2[i] = vec![0; graph.hi[i]];
        }
    }

    graph.hi.fill(0);
    let mut j = 0;
    while j < memo.len() {
        let u = memo[j];
        j += 1;
        let v = memo[j];
        j += 1;
        let weight = memo[j];
        j += 1;
        let k = graph.hi[u] as usize;
        graph.to[u].push(v);
        graph.w[u].push(weight);
        graph.hi[u] += 1;
        let k = graph.hi[v] as usize;
        graph.to[v].push(u);
        graph.w[v].push(weight);
        graph.hi[v] += 1;
    }

    let a = in_int() - 1;
    let b = in_int() - 1;
    let q = in_int();

    graph.c = vec![0; q as usize];
    graph.d = vec![0; q as usize];
    for i in 0..q {
        graph.c[i as usize] = in_int() - 1;
        graph.d[i as usize] = in_int() - 1;
    }

    let mut que = Vec::new();
    let mut qsize = 0;
    graph.dijkstra(a, &mut que, &mut qsize);
    graph.tsort(a, b);

    let ii = (q + 63) >> 6;
    for i in 0..ii {
        graph.dp.fill(0);
        for j in 0..64 {
            if (i << 6) + j < q {
                graph.dp[graph.c[(i << 6) + j as usize] as usize] |= 1 << j;
            }
        }
        for j in 0..graph.cnt {
            let u = graph.ord[j];
            for k in 0..graph.hi2[u as usize] {
                graph.dp[graph.to2[u as usize][k as usize] as usize] |= graph.dp[u as usize];
            }
        }
        for j in 0..64 {
            if (i << 6) + j < q {
                if graph.dp[graph.d[(i << 6) + j as usize] as usize] & (1 << j) != 0 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
