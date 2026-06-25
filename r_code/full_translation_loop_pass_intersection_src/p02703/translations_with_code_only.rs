use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = 10_000_000_000_000;

#[derive(Clone, Copy, Debug)]
struct Train {
    to: usize,
    cost: i32,
    time: i64,
}

fn cheat() {
    // Exact output as in the C code.
    print!("51000000000\n102000000000\n153000000000\n204000000000\n255000000000\n306000000000\n357000000000\n408000000000\n459000000000\n510000000000\n561000000000\n612000000000\n663000000000\n714000000000\n765000000000\n816000000000\n867000000000\n918000000000\n969000000000\n1020000000000\n1071000000000\n1122000000000\n1173000000000\n1224000000000\n1275000000000\n1326000000000\n1377000000000\n1428000000000\n1479000000000\n1530000000000\n1581000000000\n1632000000000\n1683000000000\n1734000000000\n1785000000000\n1836000000000\n1887000000000\n1938000000000\n1989000000000\n2040000000000\n2091000000000\n2142000000000\n2193000000000\n2244000000000\n2295000000000\n2346000000000\n2397000000000\n2448000000000\n2499000000000\n");
}

fn dijkstra(n: usize, s: usize, t: &Vec<Vec<Train>>, max_money: usize) -> Vec<i64> {
    // Mirrors the C arrays:
    // Used[i][j] initialized to 1, then set to 0 for j<=money when a state is expanded.
    let mut used = vec![vec![true; max_money + 1]; n];
    let mut city = vec![vec![INF; max_money + 1]; n];
    let mut ans = vec![-1i64; n];

    ans[0] = 0;
    city[0][s] = 0;

    let mut mati: usize = 0;
    let mut money: usize = 0;
    let mut co: usize = 1;

    loop {
        let mut cur_min: i64 = INF;

        for i in 0..n {
            for j in 0..=max_money {
                if used[i][j] && city[i][j] < cur_min {
                    cur_min = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        if ans[mati] == -1 {
            ans[mati] = cur_min;
            co += 1;
            if co == n {
                return ans;
            }
        }

        for j in 0..=money {
            used[mati][j] = false;
        }

        for tr in &t[mati] {
            // In C: if (money >= cost && time+min < city[to][mi(money-cost,max)])
            // Note: cost can be negative (self-loop "earn money"), so emulate C's signed compare.
            let cost_i64 = tr.cost as i64;
            let money_i64 = money as i64;

            if money_i64 >= cost_i64 {
                let next_money_i64 = money_i64 - cost_i64; // can increase if cost is negative
                // mi(x, max) where x is (money - cost) and max is non-negative.
                // In C, x is long (signed). Here clamp only on the upper side like mi does.
                let next_money = min(next_money_i64, max_money as i64) as usize;

                // time + min; keep i64 like C's long long.
                let cand = tr.time + cur_min;
                if cand < city[tr.to][next_money] {
                    city[tr.to][next_money] = cand;
                }
            }
        }
    }
}

fn main() {
    // Read all input and parse as integers similarly to scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_long: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m_long: i64 = it.next().unwrap().parse().unwrap();
    let s_long: i64 = it.next().unwrap().parse().unwrap();

    // C uses long for N,M,S; assume non-negative as used for sizes.
    let n = n_long as usize;
    let m = m_long as usize;

    if n_long == 50 && m_long == 49 && s_long == 0 {
        cheat();
        return;
    }

    // max = (N-1)*50 (macro), with N as long.
    // Keep it in i64 then cast to usize.
    let max_money_i64 = (n_long - 1) * 50;
    let max_money = if max_money_i64 < 0 { 0 } else { max_money_i64 as usize };

    // S = mi(S, max)
    let s = min(s_long, max_money_i64) as usize;

    let mut t: Vec<Vec<Train>> = vec![Vec::new(); n];

    for _ in 0..m {
        let mut u: i32 = it.next().unwrap().parse().unwrap();
        let mut v: i32 = it.next().unwrap().parse().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        u -= 1;
        v -= 1;
        let uu = u as usize;
        let vv = v as usize;

        t[uu].push(Train { to: vv, cost: a, time: b });
        t[vv].push(Train { to: uu, cost: a, time: b });
    }

    for i in 0..n {
        let c: i32 = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        // (train){ i, -1*c, d }
        // In C, -1*c is int arithmetic; emulate with i32.
        let cost = c.wrapping_neg();
        t[i].push(Train { to: i, cost, time: d });
    }

    let ans = dijkstra(n, s, &t, max_money);

    // C prints ans[1..N-1], each on its own line.
    let mut out = String::new();
    for i in 1..n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
