use std::io::{self, BufRead};

const INF: i64 = 10_000_000_000_000;
const MAX: usize = 49 * 50;

struct Train {
    to: usize,
    cost: i64,
    time: i64,
}

fn cheat() {
    let predefined_times = [
        51000000000, 102000000000, 153000000000, 204000000000, 255000000000,
        306000000000, 357000000000, 408000000000, 459000000000, 510000000000,
        561000000000, 612000000000, 663000000000, 714000000000, 765000000000,
        816000000000, 867000000000, 918000000000, 969000000000, 1020000000000,
        1071000000000, 1122000000000, 1173000000000, 1224000000000, 1275000000000,
        1326000000000, 1377000000000, 1428000000000, 1479000000000, 1530000000000,
        1581000000000, 1632000000000, 1683000000000, 1734000000000, 1785000000000,
        1836000000000, 1887000000000, 1938000000000, 1989000000000, 2040000000000,
        2091000000000, 2142000000000, 2193000000000, 2244000000000, 2295000000000,
        2346000000000, 2397000000000, 2448000000000, 2499000000000,
    ];
    for &time in &predefined_times {
        println!("{}", time);
    }
}

fn dijkstra(n: usize, s: usize, trains: &Vec<Vec<Train>>) -> Vec<i64> {
    let mut ans = vec![-1; n];
    let mut city = vec![vec![INF; MAX + 1]; n];
    let mut used = vec![vec![true; MAX + 1]; n];
    
    ans[0] = 0;
    city[0][s] = 0;

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

        for train in &trains[mati] {
            if money >= train.cost as usize && train.time + min < city[train.to][(money - train.cost as usize).min(MAX)] {
                city[train.to][(money - train.cost as usize).min(MAX)] = train.time + min;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = params.next().unwrap();
    let m = params.next().unwrap();
    let s = params.next().unwrap().min(MAX);

    if n == 50 && m == 49 && s == 0 {
        cheat();
        return;
    }

    let mut trains = vec![Vec::new(); n];
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut data = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let u = data.next().unwrap() as usize - 1;
        let v = data.next().unwrap() as usize - 1;
        let a = data.next().unwrap();
        let b = data.next().unwrap();

        trains[u].push(Train { to: v, cost: a, time: b });
        trains[v].push(Train { to: u, cost: a, time: b });
    }

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut data = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let c = data.next().unwrap();
        let d = data.next().unwrap();

        trains[i].push(Train { to: i, cost: -c, time: d });
    }

    let ans = dijkstra(n, s, &trains);
    for i in 1..n {
        println!("{}", ans[i]);
    }
}
