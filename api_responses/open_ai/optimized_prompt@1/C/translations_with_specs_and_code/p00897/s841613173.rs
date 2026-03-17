use std::collections::HashMap;
use std::io::{self, BufRead};

const HASHSIZ: usize = 29989;
const QMAX: usize = 1_200_000;
const MAX: usize = 6002;

#[derive(Debug)]
struct HashEntry {
    s: String,
    id: i16,
}

struct MinHeap {
    que: Vec<QueueEntry>,
    qsize: usize,
}

#[derive(Debug)]
struct QueueEntry {
    t: i32,
    s: i16,
    c: i16,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap {
            que: Vec::with_capacity(QMAX),
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
            self.que[0] = self.que[self.qsize - 1];
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

fn insert(hash: &mut [Option<HashEntry>; HASHSIZ], s: &str, id: i16) -> i16 {
    let mut i: u64 = 0;
    for c in s.chars() {
        i = (i << 6) + c as u64;
    }
    let mut tp = (i % HASHSIZ as u64) as usize;

    loop {
        if let Some(entry) = &hash[tp] {
            if entry.s == s {
                return entry.id;
            }
            tp = (tp + 1) % HASHSIZ;
        } else {
            hash[tp] = Some(HashEntry { s: s.to_string(), id });
            return -1;
        }
    }
}

fn dijkstra(n: usize, cap: i16, start: i16, goal: i16, dist: &mut Vec<Vec<i32>>, to: &[Vec<i16>], dd: &[Vec<i32>], lpg: &[bool]) -> i32 {
    let mut heap = MinHeap::new();
    heap.enq(start, 0, cap);
    dist[start as usize][cap as usize] = 0;

    while heap.qsize > 0 {
        let entry = &heap.que[0];
        let s = entry.s;
        let d = entry.t;
        let c = entry.c;
        heap.deq();

        if s == goal {
            return d;
        }

        if dist[s as usize][c as usize] < d {
            continue;
        }

        let mut current_cap = c;
        if lpg[s as usize] {
            current_cap = cap;
        }

        for (i, &e) in to[s as usize].iter().enumerate() {
            let nd = d + dd[s as usize][i];
            let nc = current_cap - dd[s as usize][i];
            if nc < 0 {
                continue;
            }
            if dist[e as usize][nc as usize] > nd {
                dist[e as usize][nc as usize] = nd;
                heap.enq(e, nd, nc);
            }
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut hash: [Option<HashEntry>; HASHSIZ] = Default::default();
    let mut dist = vec![vec![i32::MAX; 2002]; MAX];
    let mut to: Vec<Vec<i16>> = vec![Vec::new(); MAX];
    let mut dd: Vec<Vec<i32>> = vec![Vec::new(); MAX];
    let mut lpg = vec![false; MAX];
    let mut hi = vec![0; MAX];
    let mut memo = Vec::new();

    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        let m: usize = lines.next().unwrap().trim().parse().unwrap();
        let cap: i16 = (10 * lines.next().unwrap().trim().parse::<i32>().unwrap()) as i16;

        hash.iter_mut().for_each(|entry| *entry = None);
        let mut sz = 2;
        let mut j = 0;

        let mut city = vec![String::new(); MAX];
        city[0] = lines.next().unwrap();
        city[1] = lines.next().unwrap();
        insert(&mut hash, &city[0], 0);
        insert(&mut hash, &city[1], 1);

        hi[0] = 1;
        hi[1] = 1;

        for _ in 0..n {
            let city_a = lines.next().unwrap();
            let a = insert(&mut hash, &city_a, sz as i16);
            if a < 0 {
                memo.push(sz as i16);
                sz += 1;
            } else {
                memo.push(a);
            }

            let city_b = lines.next().unwrap();
            let b = insert(&mut hash, &city_b, sz as i16);
            if b < 0 {
                memo.push(sz as i16);
                sz += 1;
            } else {
                memo.push(b);
            }

            let distance: i32 = lines.next().unwrap().trim().parse().unwrap();
            memo.push(distance);
        }

        for a in 0..sz {
            if hi[a] > 0 {
                to[a].resize(hi[a] as usize, 0);
                dd[a].resize(hi[a] as usize, 0);
            }
        }

        hi.fill(0);
        let mut i = 0;
        while j < memo.len() {
            let a = memo[i];
            let b = memo[i + 1];
            let c = memo[i + 2];
            i += 3;

            let idx_a = hi[a as usize];
            to[a as usize].push(b);
            dd[a as usize].push(c);
            hi[a as usize] += 1;

            let idx_b = hi[b as usize];
            to[b as usize].push(a);
            dd[b as usize].push(c);
            hi[b as usize] += 1;
        }

        for _ in 0..m {
            let fuel_station = lines.next().unwrap();
            let a = insert(&mut hash, &fuel_station, -1);
            if a >= 0 {
                lpg[a as usize] = true;
            }
        }

        let result = dijkstra(sz as i16, cap, 0, 1, &mut dist, &to, &dd, &lpg);
        println!("{}", result);

        for a in 0..sz {
            if hi[a] > 0 {
                to[a].clear();
                dd[a].clear();
            }
        }
    }
}
