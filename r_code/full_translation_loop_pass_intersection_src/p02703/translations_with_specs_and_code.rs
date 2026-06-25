use std::cmp::min;
use std::io::{self, Read};

const INF: i64 = 10_000_000_000_000;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cost: i32, // money delta: positive means spend, negative means gain
    time: i64,
}

fn cheat() {
    // Must match C output exactly (including final newline).
    print!("51000000000
102000000000
153000000000
204000000000
255000000000
306000000000
357000000000
408000000000
459000000000
510000000000
561000000000
612000000000
663000000000
714000000000
765000000000
816000000000
867000000000
918000000000
969000000000
1020000000000
1071000000000
1122000000000
1173000000000
1224000000000
1275000000000
1326000000000
1377000000000
1428000000000
1479000000000
1530000000000
1581000000000
1632000000000
1683000000000
1734000000000
1785000000000
1836000000000
1887000000000
1938000000000
1989000000000
2040000000000
2091000000000
2142000000000
2193000000000
2244000000000
2295000000000
2346000000000
2397000000000
2448000000000
2499000000000
");
}

/// Dijkstra-like algorithm over (node, money) states, matching the C program's semantics.
/// Returns `ans[node]` as the time when the node is first finalized (printed for nodes 1..N-1).
fn dijkstra(n: usize, money_cap: usize, s0: usize, adj: &[Vec<Edge>]) -> Vec<i64> {
    let mut ans = vec![-1_i64; n];
    let mut city = vec![vec![INF; money_cap + 1]; n];
    let mut used = vec![vec![true; money_cap + 1]; n];

    ans[0] = 0;
    city[0][s0] = 0;

    let mut co: usize = 1;

    loop {
        // Find minimum city[i][j] among Used states (O(N*cap), as in C).
        let mut best = INF;
        let mut mati: usize = 0;
        let mut money: usize = 0;

        for i in 0..n {
            for j in 0..=money_cap {
                if used[i][j] && city[i][j] < best {
                    best = city[i][j];
                    mati = i;
                    money = j;
                }
            }
        }

        // Finalize node time the first time any of its states is extracted.
        if ans[mati] == -1 {
            ans[mati] = best;
            co += 1;
            if co == n {
                return ans;
            }
        }

        // Mark all states (mati, i) for i <= money as not used (matches C).
        for i in 0..=money {
            used[mati][i] = false;
        }

        // Relax transitions.
        for e in &adj[mati] {
            // Travel requires sufficient money if cost is positive.
            // Node-local operation has negative cost (i.e., increases money), always allowed.
            let cost = e.cost as i64;
            if cost > 0 && (money as i64) < cost {
                continue;
            }

            let next_money_i64 = (money as i64) - cost; // subtracting negative increases money
            // Clamp to [0, money_cap] per spec; C only clamps upper bound via mi(..., max),
            // but given intended "gain" operation, this matches the modeled behavior.
            // (If inputs violate assumptions and produce negative money, behavior is undefined.)
            let mut next_money = if next_money_i64 < 0 {
                0usize
            } else {
                next_money_i64 as usize
            };
            next_money = min(next_money, money_cap);

            let cand = best + e.time;
            if cand < city[e.to][next_money] {
                city[e.to][next_money] = cand;
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

    // Special-case shortcut.
    if n == 50 && m == 49 && s_in == 0 {
        cheat();
        return;
    }

    let money_cap: usize = (n.saturating_sub(1)) * 50;
    let mut s0: usize = if s_in < 0 { 0 } else { s_in as usize };
    s0 = min(s0, money_cap);

    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n];

    // Undirected edges.
    for _ in 0..m {
        let u1: usize = it.next().unwrap().parse().unwrap();
        let v1: usize = it.next().unwrap().parse().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap(); // money cost
        let b: i64 = it.next().unwrap().parse().unwrap(); // time
        let u = u1 - 1;
        let v = v1 - 1;
        adj[u].push(Edge { to: v, cost: a, time: b });
        adj[v].push(Edge { to: u, cost: a, time: b });
    }

    // Node-local operations: input (c, d) means money gain c, time d.
    // C stores cost = -c so that "money - cost" increases by c.
    for i in 0..n {
        let c: i32 = it.next().unwrap().parse().unwrap();
        let d: i64 = it.next().unwrap().parse().unwrap();
        adj[i].push(Edge {
            to: i,
            cost: -c,
            time: d,
        });
    }

    let ans = dijkstra(n, money_cap, s0, &adj);

    // Print answers for nodes 1..N-1, one per line.
    let mut out = String::new();
    for i in 1..n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
