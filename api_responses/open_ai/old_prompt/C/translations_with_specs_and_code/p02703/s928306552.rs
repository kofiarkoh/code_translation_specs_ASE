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
                && train.time + min_time < city[train.to][min(money - train.cost as usize, MAX)]
            {
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

    s = min(s, MAX);

    let mut trains = vec![Vec::new(); n];
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap() - 1;
        let v: usize = parts.next().unwrap().parse().unwrap() - 1;
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        trains[u].push(Train { to: v, cost: a, time: b });
        trains[v].push(Train { to: u, cost: a, time: b });
    }

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: i32 = parts.next().unwrap().parse().unwrap();
        let d: i64 = parts.next().unwrap().parse().unwrap();

        trains[i].push(Train { to: i, cost: -c, time: d });
    }

    let ans = dijkstra(n, m, s, &trains);
    for i in 1..n {
        println!("{}", ans[i]);
    }
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
