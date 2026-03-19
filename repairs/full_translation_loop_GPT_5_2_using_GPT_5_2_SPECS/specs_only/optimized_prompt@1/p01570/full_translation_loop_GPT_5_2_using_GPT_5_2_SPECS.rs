use std::cmp::Reverse;
use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Store weights and parent references.
    // Parent sentinel is not specified; we interpret it narrowly as any value outside [0, N-1].
    // This allows common sentinels like -1 or N, and matches the spec's "designated sentinel value"
    // without inventing a specific constant.
    let mut w: Vec<i64> = vec![0; n];
    let mut parent_raw: Vec<i64> = vec![0; n];

    for i in 0..n {
        w[i] = it.next().unwrap().parse::<i64>().unwrap();
        parent_raw[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    // Build children adjacency lists and collect roots.
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut roots: Vec<usize> = Vec::new();

    for u in 0..n {
        let p = parent_raw[u];
        if p >= 0 && (p as usize) < n {
            children[p as usize].push(u);
        } else {
            roots.push(u);
        }
    }

    // Post-order traversal to compute S(u) and D(u) for all nodes.
    // Since the input is a forest (acyclic), we can do an explicit stack DFS per root.
    let mut s: Vec<i64> = vec![0; n]; // subtree total weight S(u)
    let mut d: Vec<i64> = vec![0; n]; // value D(u)

    // Temporary buffer reused for sorting children by non-increasing S(v).
    // We store (S(v), v) and sort by S descending; ties arbitrary.
    let mut tmp: Vec<(i64, usize)> = Vec::new();

    for &r in &roots {
        // Stack of (node, state): state 0 = enter, 1 = exit (post-order)
        let mut stack: Vec<(usize, u8)> = Vec::new();
        stack.push((r, 0));

        while let Some((u, state)) = stack.pop() {
            if state == 0 {
                stack.push((u, 1));
                for &v in &children[u] {
                    stack.push((v, 0));
                }
            } else {
                // Compute S(u)
                let mut su = w[u];
                for &v in &children[u] {
                    su = su.wrapping_add(s[v]);
                }
                s[u] = su;

                // Compute D(u):
                // D(u) = sum D(v) + sum S(v) * rank(v) with children ordered by non-increasing S(v).
                let mut du: i64 = 0;
                for &v in &children[u] {
                    du = du.wrapping_add(d[v]);
                }

                tmp.clear();
                tmp.reserve(children[u].len());
                for &v in &children[u] {
                    tmp.push((s[v], v));
                }
                tmp.sort_by_key(|&(sv, _)| Reverse(sv));

                for (rank, &(sv, _v)) in tmp.iter().enumerate() {
                    let term = sv.wrapping_mul(rank as i64);
                    du = du.wrapping_add(term);
                }

                d[u] = du;
            }
        }
    }

    // Output sum of D(r) over all roots.
    let mut ans: i64 = 0;
    for &r in &roots {
        ans = ans.wrapping_add(d[r]);
    }

    print!("{}", ans);
}
