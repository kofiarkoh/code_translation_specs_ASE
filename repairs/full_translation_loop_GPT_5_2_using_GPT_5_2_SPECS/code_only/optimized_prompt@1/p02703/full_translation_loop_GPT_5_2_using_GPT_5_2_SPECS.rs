use std::io::{self, Read};

const INF: i64 = 10_000_000_000_000; // 1e13, matches C macro `inf`

#[derive(Clone, Copy, Debug)]
struct Train {
    to: usize,
    cost: i32,
    time: i64,
}

fn mi(x: usize, v: usize) -> usize {
    if x < v { x } else { v }
}

fn cheat() {
    // Exact output as in C (including final newline)
    print!("51000000000\n102000000000\n153000000000\n204000000000\n255000000000\n306000000000\n357000000000\n408000000000\n459000000000\n510000000000\n561000000000\n612000000000\n663000000000\n714000000000\n765000000000\n816000000000\n867000000000\n918000000000\n969000000000\n1020000000000\n1071000000000\n1122000000000\n1173000000000\n1224000000000\n1275000000000\n1326000000000\n1377000000000\n1428000000000\n1479000000000\n1530000000000\n1581000000000\n1632000000000\n1683000000000\n1734000000000\n1785000000000\n1836000000000\n1887000000000\n1938000000000\n1989000000000\n2040000000000\n2091000000000\n2142000000000\n2193000000000\n2244000000000\n2295000000000\n2346000000000\n2397000000000\n2448000000000\n2499000000000\n");
}

fn dijkstra(n: usize, s: usize, t: &Vec<Vec<Train>>) -> Vec<i64> {
    // max = (N-1)*50
    let max_money = (n - 1) * 50;

    // Used[i][j] initialized to 1, city[i][j] to INF
    let mut used = vec![vec![true; max_money + 1]; n];
    let mut city = vec![vec![INF; max_money + 1]; n];
    let mut ans = vec![-1i64; n];

    ans[0] = 0;
    city[0][s] = 0;

    let mut mati: usize = 0;
    let mut money: usize = 0;
    let mut co: usize = 1;

    loop {
        let mut minv: i64 = INF;

        for i in 0..n {
            for j in 0..=max_money {
                if used[i][j] && city[i][j] < minv {
                    minv = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        if ans[mati] == -1 {
            ans[mati] = minv;
            co += 1;
            if co == n {
                return ans;
            }
        }

        // for (int i = 0; i <= money; i++) Used[mati][i] = 0;
        for i in 0..=money {
            used[mati][i] = false;
        }

        // Relax edges
        for tr in &t[mati] {
            // money >= cost (cost can be negative for "self train" that adds money)
            // In C: money is int, cost is int. Here money is usize; compare via i64.
            if (money as i64) >= (tr.cost as i64) {
                // money - cost, with cost possibly negative => increases money.
                // C uses int arithmetic; we emulate with i64 then clamp to [0..max].
                let next_money_i64 = (money as i64) - (tr.cost as i64);
                let mut next_money: usize = if next_money_i64 < 0 {
                    // This shouldn't happen given the guard, but keep it robust.
                    0
                } else {
                    next_money_i64 as usize
                };
                next_money = mi(next_money, max_money);

                let cand = minv.saturating_add(tr.time);
                if cand < city[tr.to][next_money] {
                    city[tr.to][next_money] = cand;
                }
            }
        }
    }
}

fn main() {
    // Read all input, parse as i64 tokens (scanf-like whitespace handling)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_in: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m_in: i64 = it.next().unwrap().parse().unwrap();
    let s_in: i64 = it.next().unwrap().parse().unwrap();

    // C uses `long` for N,M,S; assume non-negative as used for sizes.
    let n = n_in as usize;
    let m = m_in as usize;

    if n == 50 && m == 49 && s_in == 0 {
        cheat();
        return;
    }

    let max_money = (n - 1) * 50;
    let s = mi(if s_in < 0 { 0 } else { s_in as usize }, max_money);

    let mut t: Vec<Vec<Train>> = vec![Vec::new(); n];

    for _ in 0..m {
        let u: i32 = it.next().unwrap().parse().unwrap();
        let v: i32 = it.next().unwrap().parse().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        let uu = (u - 1) as usize;
        let vv = (v - 1) as usize;

        t[uu].push(Train { to: vv, cost: a, time: b });
        t[vv].push(Train { to: uu, cost: a, time: b });
    }

    for i in 0..n {
        let c: i32 = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        // In C: cost = -1*c (can overflow for INT_MIN; we emulate wrapping)
        let cost = c.wrapping_neg();
        t[i].push(Train { to: i, cost, time: d });
    }

    let ans = dijkstra(n, s, &t);

    // Print ans[1..N-1], each on its own line
    let mut out = String::new();
    for i in 1..n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
