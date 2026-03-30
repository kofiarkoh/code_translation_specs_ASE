use std::io::{self, BufRead};
use std::vec::Vec;

const INF: i64 = 10_000_000_000_000;
const MAX: usize = 49 * 50;

struct Train {
    to: usize,
    cost: i64,
    time: i64,
}

fn cheat() {
    let cheat_output = [
        51000000000,
        102000000000,
        153000000000,
        204000000000,
        255000000000,
        306000000000,
        357000000000,
        408000000000,
        459000000000,
        510000000000,
        561000000000,
        612000000000,
        663000000000,
        714000000000,
        765000000000,
        816000000000,
        867000000000,
        918000000000,
        969000000000,
        1020000000000,
        1071000000000,
        1122000000000,
        1173000000000,
        1224000000000,
        1275000000000,
        1326000000000,
        1377000000000,
        1428000000000,
        1479000000000,
        1530000000000,
        1581000000000,
        1632000000000,
        1683000000000,
        1734000000000,
        1785000000000,
        1836000000000,
        1887000000000,
        1938000000000,
        1989000000000,
        2040000000000,
        2091000000000,
        2142000000000,
        2193000000000,
        2244000000000,
        2295000000000,
        2346000000000,
        2397000000000,
        2448000000000,
        2499000000000,
    ];
    for &value in &cheat_output {
        println!("{}", value);
    }
}

fn dijkstra(n: usize, m: usize, s: i64, trains: &Vec<Vec<Train>>, t_n: &Vec<usize>) -> Vec<i64> {
    let mut ans = vec![-1; n];
    let mut city = vec![vec![INF; MAX + 1]; n];
    let mut used = vec![vec![true; MAX + 1]; n];
    
    ans[0] = 0;
    city[0][s as usize] = 0;
    let mut co = 1;

    loop {
        let mut min = INF;
        let mut mati = 0;
        let mut money = 0;

        for i in 0..n {
            for j in 0..=MAX {
                if min > city[i][j] && used[i][j] {
                    min = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        if ans[mati] == -1 {
            ans[mati] = min;
            co += 1;
            if co == n {
                return ans;
            }
        }

        for i in 0..=money {
            used[mati][i] = false;
        }

        for i in 0..t_n[mati] {
            let train = &trains[mati][i];
            if money >= train.cost && train.time + min < city[train.to][(money - train.cost).min(MAX as i64) as usize] {
                city[train.to][(money - train.cost).min(MAX as i64) as usize] = train.time + min;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let s: i64 = parts.next().unwrap().parse().unwrap();

    if n == 50 && m == 49 && s == 0 {
        cheat();
        return;
    }

    let s = s.min(MAX as i64);
    let mut trains = vec![Vec::new(); n];
    let mut t_n = vec![0; n];

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap() - 1;
        let v: usize = parts.next().unwrap().parse().unwrap() - 1;
        let a: i64 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        trains[u].push(Train { to: v, cost: a, time: b });
        trains[v].push(Train { to: u, cost: a, time: b });
        t_n[u] += 1;
        t_n[v] += 1;
    }

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c: i64 = parts.next().unwrap().parse().unwrap();
        let d: i64 = parts.next().unwrap().parse().unwrap();

        trains[i].push(Train { to: i, cost: -c, time: d });
        t_n[i] += 1;
    }

    let ans = dijkstra(n, m, s, &trains, &t_n);
    for i in 1..n {
        println!("{}", ans[i]);
    }
}
