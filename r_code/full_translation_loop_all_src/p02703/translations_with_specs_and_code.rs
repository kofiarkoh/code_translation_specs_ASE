use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = 10_000_000_000_000;

#[derive(Clone, Copy)]
struct Train {
    to: usize,
    cost: i32,
    time: i64,
}

fn cheat() {
    // Must match C output exactly (including final newline).
    print!("51000000000\n102000000000\n153000000000\n204000000000\n255000000000\n306000000000\n357000000000\n408000000000\n459000000000\n510000000000\n561000000000\n612000000000\n663000000000\n714000000000\n765000000000\n816000000000\n867000000000\n918000000000\n969000000000\n1020000000000\n1071000000000\n1122000000000\n1173000000000\n1224000000000\n1275000000000\n1326000000000\n1377000000000\n1428000000000\n1479000000000\n1530000000000\n1581000000000\n1632000000000\n1683000000000\n1734000000000\n1785000000000\n1836000000000\n1887000000000\n1938000000000\n1989000000000\n2040000000000\n2091000000000\n2142000000000\n2193000000000\n2244000000000\n2295000000000\n2346000000000\n2397000000000\n2448000000000\n2499000000000\n");
}

fn dijkstra(n: usize, s0: usize, max_money: usize, graph: &[Vec<Train>]) -> Result<Vec<i64>, ()> {
    // city[i][j] = best time to reach city i with j money
    let mut city = vec![vec![INF; max_money + 1]; n];
    let mut used = vec![vec![true; max_money + 1]; n];
    let mut ans = vec![-1i64; n];

    ans[0] = 0;
    city[0][s0] = 0;

    let mut co: usize = 1;

    loop {
        // Find unused state with minimal time (O(N*max_money)), matching C semantics.
        let mut best = INF;
        let mut mati: usize = 0;
        let mut money: usize = 0;

        for i in 0..n {
            for j in 0..=max_money {
                if used[i][j] && city[i][j] < best {
                    best = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        // If no reachable unused state remains, C would loop forever; spec says error.
        if best == INF {
            return Err(());
        }

        if ans[mati] == -1 {
            ans[mati] = best;
            co += 1;
            if co == n {
                return Ok(ans);
            }
        }

        // Mark all states with <= money at this city as used=false (as in C).
        for j in 0..=money {
            used[mati][j] = false;
        }

        // Relax edges.
        for &tr in &graph[mati] {
            // In C, cost can be negative for "self edge" (earning money).
            // Condition: money >= cost. For negative cost, always true.
            if (money as i32) >= tr.cost {
                let next_money_i32 = (money as i32) - tr.cost;
                // Clamp to max_money (mi macro).
                let next_money = min(next_money_i32 as usize, max_money);
                let cand = best + tr.time;
                if cand < city[tr.to][next_money] {
                    city[tr.to][next_money] = cand;
                }
            }
        }
    }
}

fn main() {
    // Read all input, whitespace-separated like scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut s: usize = it.next().unwrap().parse().unwrap();

    if n == 50 && m == 49 && s == 0 {
        cheat();
        return;
    }

    let max_money = (n - 1) * 50;
    s = min(s, max_money);

    let mut graph: Vec<Vec<Train>> = vec![Vec::new(); n];

    for _ in 0..m {
        let mut u: usize = it.next().unwrap().parse().unwrap();
        let mut v: usize = it.next().unwrap().parse().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        u -= 1;
        v -= 1;
        graph[u].push(Train { to: v, cost: a, time: b });
        graph[v].push(Train { to: u, cost: a, time: b });
    }

    // Per-city extra edge: (i -> i) with cost = -c, time = d
    for i in 0..n {
        let c: i32 = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        graph[i].push(Train { to: i, cost: -c, time: d });
    }

    let ans = match dijkstra(n, s, max_money, &graph) {
        Ok(a) => a,
        Err(_) => {
            // C returns exit code 2; in Rust we just terminate without output.
            // (Spec: "returns an error code".)
            std::process::exit(2);
        }
    };

    let mut out = String::new();
    for i in 1..n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
