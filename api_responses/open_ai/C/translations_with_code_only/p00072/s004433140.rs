use std::io::{self, BufRead};

const INF: i32 = 1_000_000;
const MAX_V: usize = 100;

struct Prim {
    cost: [[i32; MAX_V]; MAX_V],
    mincost: [i32; MAX_V],
    used: [bool; MAX_V],
    v: usize,
}

impl Prim {
    fn new(v: usize) -> Self {
        Prim {
            cost: [[INF; MAX_V]; MAX_V],
            mincost: [INF; MAX_V],
            used: [false; MAX_V],
            v,
        }
    }

    fn min(a: i32, b: i32) -> i32 {
        if a > b {
            b
        } else {
            a
        }
    }

    fn prim(&mut self) -> i32 {
        self.mincost[0] = 0;
        let mut res = 0;

        while let Some(v) = (0..self.v)
            .filter(|&u| !self.used[u])
            .min_by_key(|&u| self.mincost[u])
        {
            self.used[v] = true;
            res += self.mincost[v];

            for u in 0..self.v {
                self.mincost[u] = Self::min(self.mincost[u], self.cost[v][u]);
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    loop {
        let v: usize = lines.next().unwrap().unwrap().parse().unwrap();
        if v == 0 {
            break;
        }
        let num: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut prim = Prim::new(num);

        for i in 0..num {
            for j in 0..num {
                prim.cost[i][j] = INF;
            }
        }

        for _ in 0..num {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split(',').collect();
            let from: usize = parts[0].parse().unwrap();
            let to: usize = parts[1].parse().unwrap();
            let dist: i32 = parts[2].parse().unwrap();
            prim.cost[from][to] = prim.cost[to][from] = (dist / 100) - 1;
        }

        println!("{}", prim.prim());
    }
}
