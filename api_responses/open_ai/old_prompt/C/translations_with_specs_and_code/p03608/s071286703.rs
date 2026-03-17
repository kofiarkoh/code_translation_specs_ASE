use std::io::{self, BufRead};

const MAX_NODES: usize = 201;
const MAX_R: usize = 9;
const INF: i32 = 1_000_000_000;

struct TSP {
    d: [[i32; MAX_NODES]; MAX_NODES],
    r: [usize; MAX_R],
    used: [bool; MAX_R],
    res: i32,
    n: usize,
    m: usize,
    r_count: usize,
}

impl TSP {
    fn new(n: usize, m: usize, r_count: usize) -> Self {
        let mut d = [[INF; MAX_NODES]; MAX_NODES];
        for i in 1..=n {
            d[i][i] = 0; // Distance to self is zero
        }
        TSP {
            d,
            r: [0; MAX_R],
            used: [false; MAX_R],
            res: INF,
            n,
            m,
            r_count,
        }
    }

    fn dfs(&mut self, c: usize, las: isize, dist: i32) {
        if c == self.r_count + 1 {
            if self.res > dist {
                self.res = dist;
            }
            return;
        }

        for i in 1..=self.r_count {
            if !self.used[i] {
                self.used[i] = true;
                if las == -1 {
                    self.dfs(c + 1, i as isize, 0);
                } else {
                    self.dfs(c + 1, i as isize, dist + self.d[self.r[las as usize]][self.r[i]]);
                }
                self.used[i] = false;
            }
        }
    }

    fn floyd_warshall(&mut self) {
        for k in 1..=self.n {
            for i in 1..=self.n {
                for j in 1..=self.n {
                    if self.d[i][j] > self.d[i][k] + self.d[k][j] {
                        self.d[i][j] = self.d[i][k] + self.d[k][j];
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let m: usize = params.next().unwrap().parse().unwrap();
    let r_count: usize = params.next().unwrap().parse().unwrap();

    let mut tsp = TSP::new(n, m, r_count);

    for i in 1..=r_count {
        let line = lines.next().unwrap().unwrap();
        tsp.r[i] = line.trim().parse::<usize>().unwrap();
    }

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edge = line.split_whitespace();
        let a: usize = edge.next().unwrap().parse().unwrap();
        let b: usize = edge.next().unwrap().parse().unwrap();
        let c: i32 = edge.next().unwrap().parse().unwrap();
        if tsp.d[a][b] > c {
            tsp.d[a][b] = c;
            tsp.d[b][a] = c; // Undirected graph
        }
    }

    tsp.floyd_warshall();

    tsp.dfs(1, -1, 0);

    println!("{}", tsp.res);
}
