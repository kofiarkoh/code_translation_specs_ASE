use std::io::{self, BufRead};
use std::cmp::min;

const INF: i32 = 0x7fffffff;
const MAX: usize = 5000;

#[derive(Clone, Copy)]
struct QUE {
    t: i32,
    k: i32,
    c: i32,
}

struct MinHeap {
    que: Vec<QUE>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(MAX),
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
        let mut smallest = if l < self.qsize && self.que[l].t < self.que[i].t {
            l
        } else {
            i
        };
        if r < self.qsize && self.que[r].t < self.que[smallest].t {
            smallest = r;
        }
        if smallest != i {
            self.que.swap(i, smallest);
            self.min_heapify(smallest);
        }
    }

    fn deq(&mut self) -> Option<(i32, i32)> {
        if self.qsize == 0 {
            return None;
        }
        self.qsize -= 1;
        let (k, c) = (self.que[0].k, self.que[0].c);
        self.que[0] = self.que[self.qsize];
        self.min_heapify(0);
        Some((k, c))
    }

    fn enq(&mut self, k: i32, t: i32, c: i32) {
        let i = self.qsize;
        self.qsize += 1;
        self.que.push(QUE { t, k, c });
        let mut i = i;
        while i > 0 && self.que[self.parent(i)].t > self.que[i].t {
            self.que.swap(i, self.parent(i));
            i = self.parent(i);
        }
    }
}

struct TBL {
    len: usize,
    to: [i32; 102],
    c: [i32; 102],
}

fn search(tbl: &mut [TBL; 102], node: &mut [[i32; 3]; 102], start: usize, goal: usize) -> i32 {
    let mut heap = MinHeap::new();
    heap.qsize = 0;
    for n in node.iter_mut() {
        n.fill(-1);
    }
    node[start][2] = 0;
    heap.enq(start as i32, 0, 2);

    while let Some((s, c)) = heap.deq() {
        let s = s as usize;
        if s == goal {
            if c != 1 {
                break;
            }
        }
        let p = &tbl[s];
        for i in 0..p.len {
            let s2 = p.to[i as usize] as usize;
            let f2 = p.c[i as usize];
            let k = node[s][c as usize] + f2;
            if c != 1 {
                if node[s2][c as usize] < 0 || k < node[s2][c as usize] {
                    node[s2][c as usize] = k;
                    heap.enq(s2 as i32, k, c);
                }
            }
            if c > 0 {
                let c2 = c - 1;
                let k = node[s][c as usize];
                if node[s2][c2 as usize] < 0 || k < node[s2][c2 as usize] {
                    node[s2][c2 as usize] = k;
                    heap.enq(s2 as i32, k, c2);
                }
            }
        }
    }
    if node[goal][0] >= 0 && node[goal][2] >= 0 {
        return min(node[goal][0], node[goal][2]);
    }
    if node[goal][0] >= 0 {
        return node[goal][0];
    }
    node[goal][2]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tbl: [TBL; 102] = Default::default();
    let mut node: [[i32; 3]; 102] = [[-1; 3]; 102];
    let mut size: usize;
    let mut m: i32;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        size = parts.next().unwrap().parse().unwrap();
        m = parts.next().unwrap().parse().unwrap();
        if size == 0 {
            break;
        }
        for t in tbl.iter_mut() {
            t.len = 0;
        }
        let start = 0;
        let goal = size - 1;
        while m > 0 {
            let line = reader.lines().next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let a: usize = parts.next().unwrap().parse().unwrap() - 1;
            let b: usize = parts.next().unwrap().parse().unwrap() - 1;
            let c: i32 = parts.next().unwrap().parse().unwrap();
            tbl[a].to[tbl[a].len] = b as i32;
            tbl[a].c[tbl[a].len] = c;
            tbl[a].len += 1;
            tbl[b].to[tbl[b].len] = a as i32;
            tbl[b].c[tbl[b].len] = c;
            tbl[b].len += 1;
            m -= 1;
        }
        println!("{}", search(&mut tbl, &mut node, start, goal));
    }
}
