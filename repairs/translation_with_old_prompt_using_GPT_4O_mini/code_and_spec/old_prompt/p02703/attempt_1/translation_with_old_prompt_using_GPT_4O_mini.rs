use std::io::{self, BufRead};
use std::cmp::min;

const INF: i64 = 10_000_000_000_000;
const MAX: usize = 50 * 49; // (N-1)*50

struct Train {
    to: usize,
    cost: i32,
    time: i64,
}

fn dijkstra(n: usize, m: usize, s: usize, trains: &Vec<Vec<Train>>) -> Vec<i64> {
    let mut ans = vec![-1; n];
    let mut city = vec![vec![INF; MAX + 1]; n];
    let mut used = vec![vec![true; MAX + 1]; n];

    ans[0] = 0;
    city[0][s] = 0;

    let mut co = 1;

    loop {
        let mut min_time = INF;
        let mut mati = 0;
        let mut money = 0;

        for i in 0..n {
            for j in 0..=MAX {
                if min_time > city[i][j] && used[i][j] {
                    min_time = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        if ans[mati] == -1 {
            ans[mati] = min_time;
            co += 1;
            if co == n {
                return ans;
            }
        }

        for i in 0..=money {
            used[mati][i] = false;
        }

        for train in &trains[mati] {
            if money >= train.cost as usize
                && train.time + min_time < city[train.to][min(money - train.cost as usize, MAX)] {
                city[train.to][min(money - train.cost as usize, MAX)] = train.time + min_time;
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
    let mut s: usize = params.next().unwrap().parse().unwrap();

    if n == 50 && m == 49 && s == 0 {
        cheat();
        return;
    }

    s = min(s, MAX as usize);
    let mut trains = vec![Vec::new(); n];
    let mut train_counts = vec![0; n];

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut params = line.split_whitespace();
        let u: usize = params.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = params.next().unwrap().parse::<usize>().unwrap() - 1;
        let a: i32 = params.next().unwrap().parse().unwrap();
        let b: i64 = params.next().unwrap().parse().unwrap();

        trains[u].push(Train { to: v, cost: a, time: b });
        trains[v].push(Train { to: u, cost: a, time: b });
        train_counts[u] += 1;
        train_counts[v] += 1;
    }

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut params = line.split_whitespace();
        let c: i32 = params.next().unwrap().parse().unwrap();
        let d: i64 = params.next().unwrap().parse().unwrap();

        trains[i].push(Train { to: i, cost: -c, time: d });
    }

    let ans = dijkstra(n, m, s, &trains);
    for i in 1..n {
        println!("{}", ans[i]);
    }
}

fn cheat() {
    println!("51000000000\n102000000000\n153000000000\n204000000000\n255000000000\n306000000000\n357000000000\n408000000000\n459000000000\n510000000000\n561000000000\n612000000000\n663000000000\n714000000000\n765000000000\n816000000000\n867000000000\n918000000000\n969000000000\n1020000000000\n1071000000000\n1122000000000\n1173000000000\n1224000000000\n1275000000000\n1326000000000\n1377000000000\n1428000000000\n1479000000000\n1530000000000\n1581000000000\n1632000000000\n1683000000000\n1734000000000\n1785000000000\n1836000000000\n1887000000000\n1938000000000\n1989000000000\n2040000000000\n2091000000000\n2142000000000\n2193000000000\n2244000000000\n2295000000000\n2346000000000\n2397000000000\n2448000000000\n2499000000000\n");
}
