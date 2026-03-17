// AOJ 1318 Long Distance Taxi
// 2018.3.2 bal4u

use std::collections::HashMap;
use std::io::{self, BufRead};

const HASHSIZ: usize = 29989;
const QMAX: usize = 1200000;
const MAX: usize = 6002;

#[derive(Debug)]
struct Hash {
    s: String,
    id: i16,
}

struct Queue {
    t: i32,
    s: i16,
    c: i16,
}

struct Graph {
    hi: Vec<usize>,
    to: Vec<Vec<i16>>,
    dd: Vec<Vec<i16>>,
    city: Vec<String>,
    lpg: Vec<bool>,
    dist: Vec<Vec<i32>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            hi: vec![0; size],
            to: vec![Vec::new(); size],
            dd: vec![Vec::new(); size],
            city: vec![String::new(); size],
            lpg: vec![false; size],
            dist: vec![vec![i32::MAX; 2002]; size],
        }
    }

    fn insert(&mut self, s: String, id: i16) -> Option<i16> {
        for (i, city) in self.city.iter().enumerate() {
            if city == &s {
                return Some(i as i16);
            }
        }
        self.city[id as usize] = s;
        Some(id)
    }

    fn dijkstra(&mut self, n: usize, cap: i16, start: i16, goal: i16) -> i32 {
        let mut queue: Vec<Queue> = Vec::new();
        self.dist[start as usize][cap as usize] = 0;
        queue.push(Queue { t: 0, s: start, c: cap });

        while let Some(Queue { t, s, c }) = queue.pop() {
            if s == goal {
                return t;
            }

            if self.dist[s as usize][c as usize] < t {
                continue;
            }

            let mut current_cap = c;
            if self.lpg[s as usize] {
                current_cap = cap;
            }

            for i in 0..self.hi[s as usize] {
                let e = self.to[s as usize][i];
                let nd = t + self.dd[s as usize][i] as i32;
                let nc = current_cap - self.dd[s as usize][i];

                if nc < 0 {
                    continue;
                }

                if self.dist[e as usize][nc as usize] > nd {
                    self.dist[e as usize][nc as usize] = nd;
                    queue.push(Queue { t: nd, s: e, c: nc });
                }
            }
        }
        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);

    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        let m: usize = lines.next().unwrap().trim().parse().unwrap();
        let cap: i16 = 10 * lines.next().unwrap().trim().parse::<i32>().unwrap() as i16;

        let mut graph = Graph::new(MAX);
        let mut memo: Vec<i16> = Vec::new();

        graph.insert(lines.next().unwrap(), 0);
        graph.insert(lines.next().unwrap(), 1);
        graph.hi[0] = 1;
        graph.hi[1] = 1;

        let mut sz = 2;

        for _ in 0..n {
            let city_a = lines.next().unwrap();
            let a = graph.insert(city_a.clone(), sz as i16).unwrap_or(sz as i16);
            if a == sz as i16 {
                sz += 1;
            }
            memo.push(a);

            let city_b = lines.next().unwrap();
            let b = graph.insert(city_b.clone(), sz as i16).unwrap_or(sz as i16);
            if b == sz as i16 {
                sz += 1;
            }
            memo.push(b);

            let distance: i16 = lines.next().unwrap().trim().parse().unwrap();
            memo.push(distance);
        }

        for a in 0..sz {
            if graph.hi[a] > 0 {
                graph.to[a] = vec![0; graph.hi[a]];
                graph.dd[a] = vec![0; graph.hi[a]];
            }
        }

        let mut j = 0;
        while n > 0 {
            let a = memo[j];
            let b = memo[j + 1];
            let c = memo[j + 2];
            graph.to[a as usize].push(b);
            graph.dd[a as usize].push(c);
            graph.to[b as usize].push(a);
            graph.dd[b as usize].push(c);
            j += 3;
            n -= 1;
        }

        for _ in 0..m {
            let lpg_city = lines.next().unwrap();
            if let Some(a) = graph.insert(lpg_city, -1) {
                graph.lpg[a as usize] = true;
            }
        }

        println!("{}", graph.dijkstra(sz, cap, 0, 1));
    }
}
