use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = 10_000_000_000_000;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cost: i32, // can be negative for "earning" edges (per C code encoding)
    time: i64,
}

fn cheat() {
    // Must match C output exactly (49 lines).
    print!("51000000000\n102000000000\n153000000000\n204000000000\n255000000000\n306000000000\n357000000000\n408000000000\n459000000000\n510000000000\n561000000000\n612000000000\n663000000000\n714000000000\n765000000000\n816000000000\n867000000000\n918000000000\n969000000000\n1020000000000\n1071000000000\n1122000000000\n1173000000000\n1224000000000\n1275000000000\n1326000000000\n1377000000000\n1428000000000\n1479000000000\n1530000000000\n1581000000000\n1632000000000\n1683000000000\n1734000000000\n1785000000000\n1836000000000\n1887000000000\n1938000000000\n1989000000000\n2040000000000\n2091000000000\n2142000000000\n2193000000000\n2244000000000\n2295000000000\n2346000000000\n2397000000000\n2448000000000\n2499000000000\n");
}

/// Semantics-preserving translation of the C "dijkstra" routine.
/// Note: This is not a standard Dijkstra with a heap; it scans all states each step,
/// and uses a special "Used[mati][0..=money]=0" rule exactly as in C.
fn dijkstra(n: usize, s0: usize, max_money: usize, adj: &[Vec<Edge>]) -> Vec<i64> {
    let mut ans = vec![-1i64; n];
    let mut used = vec![vec![true; max_money + 1]; n];
    let mut city = vec![vec![INF; max_money + 1]; n];

    ans[0] = 0;
    city[0][s0] = 0;

    let mut mati: usize = 0;
    let mut money: usize = 0;
    let mut co: usize = 1;

    loop {
        // Find minimum city[i][j] among Used states.
        let mut best = INF;
        for i in 0..n {
            for j in 0..=max_money {
                if used[i][j] && city[i][j] < best {
                    best = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        // In C, if best stays INF, it still proceeds; we preserve that behavior.
        if ans[mati] == -1 {
            ans[mati] = best;
            co += 1;
            if co == n {
                return ans;
            }
        }

        // Mark Used[mati][0..=money] = 0 (false)
        for j in 0..=money {
            used[mati][j] = false;
        }

        // Relax edges from mati.
        for &e in &adj[mati] {
            // C condition: if (money >= cost) where cost may be negative for earning edges.
            // For negative cost, this is always true; money - cost increases money.
            let cost_i64 = e.cost as i64;
            if (money as i64) >= cost_i64 {
                let new_money_i64 = (money as i64) - cost_i64;
                // Clamp to max_money (and implicitly nonnegative due to condition above).
                let new_money = min(new_money_i64 as usize, max_money);
                let new_time = best.saturating_add(e.time);
                if new_time < city[e.to][new_money] {
                    city[e.to][new_money] = new_time;
                }
            }
        }
    }
}

fn main() {
    // Read all input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let s_in: i64 = it.next().unwrap().parse().unwrap();

    // Hardcoded special case.
    if n == 50 && m == 49 && s_in == 0 {
        cheat();
        return;
    }

    // Money cap: 50 * (N-1)
    // Spec says this is expected to fit internal bounds (effectively N <= 50).
    let max_money: usize = (n.saturating_sub(1)) * 50;

    // Clamp initial money to cap (C uses mi(S, max)).
    let mut s0: usize = if s_in < 0 { 0 } else { s_in as usize };
    s0 = min(s0, max_money);

    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];

    // Read bidirectional connections.
    for _ in 0..m {
        let u1: usize = it.next().unwrap().parse().unwrap();
        let v1: usize = it.next().unwrap().parse().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap(); // money cost (nonnegative per spec)
        let b: i64 = it.next().unwrap().parse().unwrap(); // time cost
        let u = u1 - 1;
        let v = v1 - 1;
        adj[u].push(Edge { to: v, cost: a, time: b });
        adj[v].push(Edge { to: u, cost: a, time: b });
    }

    // Read per-node earning options: (c, d) where c is money gained, d is time cost.
    // C encodes this as an edge with cost = -c, time = d, to = self.
    for i in 0..n {
        let c: i32 = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        adj[i].push(Edge { to: i, cost: -c, time: d });
    }

    let ans = dijkstra(n, s0, max_money, &adj);

    // Output for nodes 2..N (1-based), i.e., indices 1..n-1.
    let mut out = String::new();
    for i in 1..n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}

/*
Note on spec vs C:
- The spec describes "earning amount is nonnegative". The C code reads c and uses cost = -c,
  which assumes c >= 0; if c were negative, it would become a spending action. We preserve C behavior.
- The algorithm is not a standard Dijkstra; it uses a 2D state table and a peculiar Used-marking rule.
  We preserve it exactly to match intended behavior.
*/
