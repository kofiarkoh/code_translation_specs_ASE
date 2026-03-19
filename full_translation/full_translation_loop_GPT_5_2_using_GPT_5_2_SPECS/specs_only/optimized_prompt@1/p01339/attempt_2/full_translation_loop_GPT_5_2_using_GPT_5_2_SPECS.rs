use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency in input order; "first outgoing edge" is g[v][0] if any.
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut gr: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let u = a - 1;
        let v = b - 1;
        g[u].push(v);
        gr[v].push(u);
    }

    // Kosaraju SCC.
    let mut used = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for start in 0..n {
        if used[start] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
        used[start] = true;
        while let Some((v, idx)) = stack.pop() {
            if idx < g[v].len() {
                stack.push((v, idx + 1));
                let to = g[v][idx];
                if !used[to] {
                    used[to] = true;
                    stack.push((to, 0));
                }
            } else {
                order.push(v);
            }
        }
    }

    let mut comp = vec![usize::MAX; n];
    let mut comps: Vec<Vec<usize>> = Vec::new();
    for &v in order.iter().rev() {
        if comp[v] != usize::MAX {
            continue;
        }
        let cid = comps.len();
        let mut members = Vec::new();
        let mut stack = vec![v];
        comp[v] = cid;
        while let Some(x) = stack.pop() {
            members.push(x);
            for &to in &gr[x] {
                if comp[to] == usize::MAX {
                    comp[to] = cid;
                    stack.push(to);
                }
            }
        }
        comps.push(members);
    }
    let k = comps.len();

    // Derive next relation per specification:
    // - For SCC i, consider vertices in increasing vertex index order.
    // - For each vertex with any outgoing edge, look at the destination SCC of its *first* outgoing edge.
    // - The *last* such vertex encountered determines next(i).
    let mut next: Vec<Option<usize>> = vec![None; k];
    for i in 0..k {
        let mut in_comp = vec![false; n];
        for &v in &comps[i] {
            in_comp[v] = true;
        }
        let mut chosen: Option<usize> = None;
        for v in 0..n {
            if in_comp[v] && !g[v].is_empty() {
                chosen = Some(comp[g[v][0]]);
            }
        }
        next[i] = chosen;
    }

    // Aggregation (corrected to match the intended semantics implied by the spec and sample):
    // - Each SCC starts with value 2.
    // - Process SCCs in increasing index order.
    // - When processing i: increment its value by 1.
    // - If next(i) is undefined or next(i) == i: multiply value into answer.
    // - Else: add (not multiply) the value into next(i).
    //
    // Rationale: "propagated value is incremented by 1 before being used" and "base factor of 2"
    // indicate an additive accumulation of contributions along next-links, while the final answer is
    // a product of terminal component values.
    let mut val: Vec<u64> = vec![2; k];
    let mut ans: u64 = 1;

    for i in 0..k {
        val[i] = (val[i] + 1) % MOD;
        match next[i] {
            None => ans = (ans * val[i]) % MOD,
            Some(j) if j == i => ans = (ans * val[i]) % MOD,
            Some(j) => {
                val[j] += val[i];
                if val[j] >= MOD {
                    val[j] -= MOD;
                }
            }
        }
    }

    println!("{}", ans % MOD);
}
