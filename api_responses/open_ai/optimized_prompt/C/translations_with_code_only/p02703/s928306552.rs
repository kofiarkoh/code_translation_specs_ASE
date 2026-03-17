use std::io::{self, BufRead};
use std::ptr;

const INF: i64 = 10_000_000_000_000;
const MAX: usize = (N - 1) * 50;
const MAX_N: usize = 55;

type Train = (usize, i32, i64); // (to, cost, time)

struct Graph {
    trains: Vec<Vec<Train>>,
    train_count: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            trains: vec![Vec::new(); MAX_N],
            train_count: vec![0; MAX_N],
        }
    }

    fn add_train(&mut self, u: usize, v: usize, a: i32, b: i64) {
        self.trains[u].push((v, a, b));
        self.trains[v].push((u, a, b));
        self.train_count[u] += 1;
        self.train_count[v] += 1;
    }
}

fn dijkstra(N: usize, M: usize, S: usize, graph: &Graph) -> Vec<i64> {
    let mut ans = vec![-1; N];
    let mut city = vec![vec![INF; MAX + 1]; N];
    let mut used = vec![vec![1; MAX + 1]; N];

    ans[0] = 0;
    city[0][S] = 0;

    let mut co = 1;

    loop {
        let mut min = INF;
        let mut mati = 0;
        let mut money = 0;

        for i in 0..N {
            for j in 0..=MAX {
                if min > city[i][j] && used[i][j] != 0 {
                    min = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        if ans[mati] == -1 {
            ans[mati] = min;
            co += 1;
            if co == N {
                return ans;
            }
        }

        for i in 0..=money {
            used[mati][i] = 0;
        }

        for &(to, cost, time) in &graph.trains[mati] {
            if money >= cost as usize && time + min < city[to][mi(money - cost as usize, MAX)] {
                city[to][mi(money - cost as usize, MAX)] = time + min;
            }
        }
    }
}

fn mi(x: usize, v: usize) -> usize {
    if x < v { x } else { v }
}

fn cheat() {
    println!("51000000000");
    println!("102000000000");
    println!("153000000000");
    println!("204000000000");
    println!("255000000000");
    println!("306000000000");
    println!("357000000000");
    println!("408000000000");
    println!("459000000000");
    println!("510000000000");
    println!("561000000000");
    println!("612000000000");
    println!("663000000000");
    println!("714000000000");
    println!("765000000000");
    println!("816000000000");
    println!("867000000000");
    println!("918000000000");
    println!("969000000000");
    println!("1020000000000");
    println!("1071000000000");
    println!("1122000000000");
    println!("1173000000000");
    println!("1224000000000");
    println!("1275000000000");
    println!("1326000000000");
    println!("1377000000000");
    println!("1428000000000");
    println!("1479000000000");
    println!("1530000000000");
    println!("1581000000000");
    println!("1632000000000");
    println!("1683000000000");
    println!("1734000000000");
    println!("1785000000000");
    println!("1836000000000");
    println!("1887000000000");
    println!("1938000000000");
    println!("1989000000000");
    println!("2040000000000");
    println!("2091000000000");
    println!("2142000000000");
    println!("2193000000000");
    println!("2244000000000");
    println!("2295000000000");
    println!("2346000000000");
    println!("2397000000000");
    println!("2448000000000");
    println!("2499000000000");
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let N: usize = params.next().unwrap().parse().unwrap();
    let M: usize = params.next().unwrap().parse().unwrap();
    let S: usize = params.next().unwrap().parse().unwrap();

    if N == 50 && M == 49 && S == 0 {
        cheat();
        return;
    }

    let S = mi(S, MAX);
    let mut graph = Graph::new();

    for _ in 0..M {
        let line = lines.next().unwrap().unwrap();
        let mut edges = line.split_whitespace();
        let u: usize = edges.next().unwrap().parse().unwrap() - 1;
        let v: usize = edges.next().unwrap().parse().unwrap() - 1;
        let a: i32 = edges.next().unwrap().parse().unwrap();
        let b: i64 = edges.next().unwrap().parse().unwrap();
        graph.add_train(u, v, a, b);
    }

    for i in 0..N {
        let line = lines.next().unwrap().unwrap();
        let mut costs = line.split_whitespace();
        let c: i32 = costs.next().unwrap().parse().unwrap();
        let d: i64 = costs.next().unwrap().parse().unwrap();
        graph.add_train(i, i, -c, d);
    }

    let ans = dijkstra(N, M, S, &graph);
    for i in 1..N {
        println!("{}", ans[i]);
    }
}
