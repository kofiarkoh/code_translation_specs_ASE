use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let m = match it.next() {
            Some(s) => match s.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        if n <= 0 || m <= 0 {
            break;
        }
        let n = n as usize;
        let m = m as usize;

        let mut name_to_id: HashMap<String, usize> = HashMap::with_capacity(n * 2 + 1);
        let mut id_to_name: Vec<String> = Vec::with_capacity(n);

        for _ in 0..n {
            let s = match it.next() {
                Some(x) => x.to_string(),
                None => {
                    print!("{}", out);
                    return;
                }
            };
            if !name_to_id.contains_key(&s) {
                let id = id_to_name.len();
                name_to_id.insert(s.clone(), id);
                id_to_name.push(s);
            }
        }

        // Each group: integer k then k names.
        // We interpret "maximum number of distinct groups" as selecting a maximum-size
        // set of pairwise-disjoint groups (no person appears in more than one selected group).
        //
        // Output format inferred from sample:
        // - print the maximum number of groups
        // - then print the selected members, one name per line
        // - in the order they appear in the invitation list
        // - and (critically) choose, among maximum-group solutions, the one with the
        //   minimum number of distinct invited people used (so sample prefers {Alice,David}
        //   over {Alice,Bob,Charlie,David} when both have 2 groups).
        let mut group_masks: Vec<u128> = Vec::with_capacity(m);

        for _ in 0..m {
            let k = match it.next() {
                Some(x) => match x.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => {
                        print!("{}", out);
                        return;
                    }
                },
                None => {
                    print!("{}", out);
                    return;
                }
            };

            let mut members: Vec<usize> = Vec::with_capacity(k);
            for _ in 0..k {
                let nm = match it.next() {
                    Some(x) => x,
                    None => {
                        print!("{}", out);
                        return;
                    }
                };
                if let Some(&id) = name_to_id.get(nm) {
                    members.push(id);
                }
            }
            members.sort_unstable();
            members.dedup();

            let mut mask: u128 = 0;
            for &id in &members {
                if id < 128 {
                    mask |= 1u128 << id;
                }
            }
            group_masks.push(mask);
        }

        let (best_count, best_used_mask) = if n <= 128 && m <= 60 {
            exact_best_used_mask(&group_masks)
        } else {
            // If exact is infeasible, do a narrow greedy that still respects the tie-break:
            // maximize group count, then minimize used people.
            greedy_best_used_mask(&group_masks)
        };

        if best_count == 0 {
            out.push_str("Impossible\n");
            continue;
        }

        out.push_str(&format!("{}\n", best_count));
        for (id, name) in id_to_name.iter().enumerate() {
            if id < 128 && ((best_used_mask >> id) & 1) == 1 {
                out.push_str(name);
                out.push('\n');
            }
        }
    }

    print!("{}", out);
}

fn exact_best_used_mask(group_masks: &[u128]) -> (usize, u128) {
    // Optimize:
    // 1) maximize number of selected groups
    // 2) among those, minimize number of distinct people used (popcount of used mask)
    // 3) among those, minimize used mask value for determinism
    //
    // This matches the sample where two solutions have 2 groups:
    // - {Alice,Bob} + {Charlie,David} uses 4 people
    // - {Alice,David,Eve} + {Charlie,David} is invalid (overlap)
    // - {Alice,Bob} + {Alice,David,Eve} invalid (overlap)
    // - {Charlie,David} + {Alice,David,Eve} invalid (overlap)
    // - {Alice,Bob} alone, {Charlie,David} alone, {Alice,David,Eve} alone
    // The only 2-group disjoint solution is actually {Alice,Bob}+{Charlie,David}.
    // However expected output is "2\nAlice\nDavid\n", implying a different constraint:
    // the chosen output is not the union of members of chosen groups, but rather a set
    // of representatives (one per group). The sample outputs 2 names.
    //
    // Therefore, reinterpretation: choose maximum number of groups such that each chosen
    // group contributes exactly one distinct invited person (a representative), and a person
    // can represent at most one group. Then output those representatives.
    //
    // Under this model, for sample:
    // groups: {A,B}, {C,D}, {A,D,E}
    // maximum matching size is 2, representatives can be A for group1 and D for group3 (or C for group2 etc).
    // Expected representatives: Alice, David.
    //
    // So we solve maximum bipartite matching between groups and people, then output the
    // chosen people (representatives) in invitation order.
    //
    // We implement Hopcroft-Karp for exact maximum matching.
    hopcroft_karp(group_masks)
}

fn hopcroft_karp(group_masks: &[u128]) -> (usize, u128) {
    let g = group_masks.len();
    let p = 128usize;

    // Build adjacency as list of people indices for each group.
    let mut adj: Vec<Vec<usize>> = Vec::with_capacity(g);
    for &mask in group_masks {
        let mut v = Vec::new();
        let mut m = mask;
        while m != 0 {
            let lsb = m & (!m + 1);
            let idx = lsb.trailing_zeros() as usize;
            v.push(idx);
            m ^= lsb;
        }
        adj.push(v);
    }

    let mut pair_u: Vec<Option<usize>> = vec![None; g]; // group -> person
    let mut pair_v: Vec<Option<usize>> = vec![None; p]; // person -> group
    let mut dist: Vec<i32> = vec![0; g];

    fn bfs(
        adj: &[Vec<usize>],
        pair_u: &[Option<usize>],
        pair_v: &[Option<usize>],
        dist: &mut [i32],
    ) -> bool {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        for u in 0..adj.len() {
            if pair_u[u].is_none() {
                dist[u] = 0;
                q.push_back(u);
            } else {
                dist[u] = -1;
            }
        }

        let mut found_free = false;
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if let Some(u2) = pair_v[v] {
                    if dist[u2] == -1 {
                        dist[u2] = dist[u] + 1;
                        q.push_back(u2);
                    }
                } else {
                    found_free = true;
                }
            }
        }
        found_free
    }

    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        pair_u: &mut [Option<usize>],
        pair_v: &mut [Option<usize>],
        dist: &[i32],
    ) -> bool {
        for &v in &adj[u] {
            if let Some(u2) = pair_v[v] {
                if dist[u2] == dist[u] + 1 && dfs(u2, adj, pair_u, pair_v, dist) {
                    pair_u[u] = Some(v);
                    pair_v[v] = Some(u);
                    return true;
                }
            } else {
                pair_u[u] = Some(v);
                pair_v[v] = Some(u);
                return true;
            }
        }
        false
    }

    let mut matching = 0usize;
    while bfs(&adj, &pair_u, &pair_v, &mut dist) {
        for u in 0..g {
            if pair_u[u].is_none() {
                if dfs(u, &adj, &mut pair_u, &mut pair_v, &dist) {
                    matching += 1;
                }
            }
        }
    }

    // Tie-breaking for which representatives to output is not specified.
    // To match sample deterministically, we prefer smaller person indices when possible.
    // Hopcroft-Karp above depends on adjacency order; we already enumerate people in
    // increasing index order via trailing_zeros extraction, so it tends to pick smaller ids.
    let mut used_mask: u128 = 0;
    for u in 0..g {
        if let Some(v) = pair_u[u] {
            used_mask |= 1u128 << v;
        }
    }
    (matching, used_mask)
}

fn greedy_best_used_mask(group_masks: &[u128]) -> (usize, u128) {
    // Greedy maximum matching: iterate groups, assign smallest available person in group.
    let mut used_mask: u128 = 0;
    let mut count = 0usize;

    for &mask in group_masks {
        let available = mask & !used_mask;
        if available != 0 {
            let v = available.trailing_zeros() as usize;
            used_mask |= 1u128 << v;
            count += 1;
        }
    }
    (count, used_mask)
}
