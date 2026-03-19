// AOJ 1318 Long Distance Taxi
// 2018.3.2 bal4u

use std::collections::HashMap;
use std::io::{self, BufRead};

const HASHSIZ: usize = 29989;
const QMAX: usize = 1200000;
const MAX: usize = 6002;

#[derive(Clone)]
struct HashEntry {
    s: Option<String>,
    id: i16,
}

#[derive(Clone)]
struct QueueEntry {
    t: i32,
    s: i16,
    c: i16,
}

struct MinHeap {
    que: Vec<QueueEntry>,
    qsize: usize,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::new(),
            qsize: 0,
        }
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
        if self.qsize > 0 {
            self.que[0] = self.que[self.qsize - 1].clone();
            self.qsize -= 1;
            self.min_heapify(0);
        }
    }

    fn enq(&mut self, s: i16, t: i32, c: i16) {
        let i = self.qsize;
        self.que.push(QueueEntry { s, t, c });
        self.qsize += 1;

        let mut i = i;
        while i > 0 {
            let parent = self.parent(i);
            if self.que[parent].t > self.que[i].t {
                self.que.swap(i, parent);
                i = parent;
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

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn ins(buf: &mut String) {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    *buf = input.trim().to_string();
}

fn dijkstra(n: usize, cap: i16, start: i16, goal: i16, dist: &mut Vec<Vec<i32>>, que: &mut MinHeap, to: &Vec<Vec<i16>>, dd: &Vec<Vec<i16>>, lpg: &Vec<u8>) -> i32 {
    for i in 0..n {
        for j in 0..cap as usize + 1 {
            dist[i][j] = i32::MAX;
        }
    }
    dist[start as usize][cap as usize] = 0;
    que.enq(start, 0, cap);

    while que.qsize > 0 {
        let s = que.que[0].s;
        let d = que.que[0].t;
        let c = que.que[0].c;
        que.deq();

        if s == goal {
            return d;
        }

        if dist[s as usize][c as usize] < d {
            continue;
        }

        let mut current_cap = if lpg[s as usize] > 0 { cap } else { c };
        for (i, &e) in to[s as usize].iter().enumerate() {
            let nd = d + dd[s as usize][i] as i32; // Cast to i32
            let nc = current_cap - dd[s as usize][i];
            if nc < 0 {
                continue;
            }
            if dist[e as usize][nc as usize] > nd {
                dist[e as usize][nc as usize] = nd;
                que.enq(e, nd, nc);
            }
        }
    }
    -1
}

fn main() {
    let mut hash: Vec<HashEntry> = vec![HashEntry { s: None, id: -1 }; HASHSIZ + 2];
    let mut dist = vec![vec![0; 2002]; MAX];
    let mut to: Vec<Vec<i16>> = vec![Vec::new(); MAX];
    let mut dd: Vec<Vec<i16>> = vec![Vec::new(); MAX];
    let mut hi = vec![0; MAX];
    let mut lpg = vec![0; MAX];
    let mut memo = vec![0; 3002 * 3];
    let mut city = vec![String::new(); MAX];
    let mut sz = 0;

    loop {
        let n = in_number();
        if n == 0 {
            break;
        }
        let m = in_number();
        let cap = 10 * in_number();
        hash.iter_mut().for_each(|entry| entry.s = None);

        ins(&mut city[0]);
        ins(&mut city[1]);
        insert(&mut hash, &city[0], 0);
        insert(&mut hash, &city[1], 1);

        hi.fill(0);
        sz = 2;
        let mut j = 0;
        let mut i = n;
        while i > 0 {
            ins(&mut city[sz]);
            let a = insert(&mut hash, &city[sz], sz.try_into().unwrap());
            let a = if a < 0 { sz.try_into().unwrap() } else { a };
            memo[j] = a;
            j += 1;
            hi[a as usize] += 1;

            ins(&mut city[sz]);
            let b = insert(&mut hash, &city[sz], sz.try_into().unwrap());
            let b = if b < 0 { sz.try_into().unwrap() } else { b };
            memo[j] = b;
            j += 1;
            hi[b as usize] += 1;

            memo[j] = in_number();
            j += 1;
            sz += 1;
            i -= 1;
        }

        for a in 0..sz {
            if hi[a] > 0 {
                to[a] = vec![0; hi[a] as usize * 2];
                dd[a] = vec![0; hi[a] as usize * 2];
            }
        }

        hi.fill(0);
        j = 0;
        while n > 0 {
            let a = memo[j];
            j += 1;
            let b = memo[j];
            j += 1;
            let c = memo[j];
            j += 1;
            let i = hi[a as usize];
            to[a as usize][i as usize] = b;
            dd[a as usize][i as usize] = c;
            hi[a as usize] += 1;

            let i = hi[b as usize];
            to[b as usize][i as usize] = a;
            dd[b as usize][i as usize] = c;
            hi[b as usize] += 1;

            n -= 1;
        }

        lpg.fill(0);
        for _ in 0..m {
            let mut buf = String::new();
            ins(&mut buf);
            let a = insert(&mut hash, &buf, -1);
            if a >= 0 {
                lpg[a as usize] = 1;
            }
        }

        let result = dijkstra(sz, cap, 0, 1, &mut dist, &mut MinHeap::new(), &to, &dd, &lpg);
        println!("{}", result);

        for a in 0..sz {
            if hi[a] > 0 {
                to[a].clear();
                dd[a].clear();
            }
        }
    }
}

fn insert(hash: &mut Vec<HashEntry>, s: &String, id: i16) -> i16 {
    let mut i: u64 = 0;
    for &byte in s.as_bytes() {
        i = (i << 6) + byte as u64;
    }
    let mut tp = (i % HASHSIZ as u64) as usize;

    while hash[tp].s.is_some() {
        if hash[tp].s.as_ref().unwrap() == s {
            return hash[tp].id;
        }
        tp += 1;
        if tp == HASHSIZ + 2 {
            tp = 0;
        }
    }
    hash[tp].s = Some(s.clone());
    hash[tp].id = id;
    -1
}
