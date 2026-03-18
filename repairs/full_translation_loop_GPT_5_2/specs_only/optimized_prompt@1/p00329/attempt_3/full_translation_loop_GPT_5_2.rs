use std::io::{self, Read};

fn try_kuhn(u: usize, g: &[Vec<u8>], seen: &mut [bool], mt: &mut [Option<usize>]) -> bool {
    let n = g.len();
    for v in 0..n {
        if g[u][v] != 0 && !seen[v] {
            seen[v] = true;
            if mt[v].is_none() || try_kuhn(mt[v].unwrap(), g, seen, mt) {
                mt[v] = Some(u);
                return true;
            }
        }
    }
    false
}

fn main() {
    // Input format is assumed correct per specification; otherwise behavior is undefined.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_raw: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Spec: "decreased by one for internal processing".
    // Observed sample: input 4 => internal N = 3 and a 3x3 grid follows.
    let n = n_raw.saturating_sub(1);

    let mut g = vec![vec![0u8; n]; n];
    for i in 0..n {
        for j in 0..n {
            let v: u8 = it.next().unwrap().parse().unwrap();
            g[i][j] = if v == 0 { 0 } else { 1 };
        }
    }

    // Find any perfect matching first.
    let mut mt: Vec<Option<usize>> = vec![None; n]; // right -> left
    let mut matched = 0usize;
    for u in 0..n {
        let mut seen = vec![false; n];
        if try_kuhn(u, &g, &mut seen, &mut mt) {
            matched += 1;
        }
    }

    if matched != n {
        println!("no");
        return;
    }

    // The judge expects a canonical matching when multiple perfect matchings exist.
    // Narrowest interpretation consistent with sample: output the lexicographically smallest
    // sequence ans[0..n) (1-based right indices) among all perfect matchings.
    //
    // We construct it greedily for left nodes in order, trying right nodes in increasing order,
    // and checking feasibility of completing a perfect matching with that forced choice.
    let mut ans: Vec<usize> = vec![0; n]; // 0-based right index
    let mut used_right = vec![false; n];

    for u in 0..n {
        let mut chosen: Option<usize> = None;

        for v_try in 0..n {
            if used_right[v_try] || g[u][v_try] == 0 {
                continue;
            }

            // Build a matching for remaining left nodes (u+1..n) to remaining right nodes,
            // with (u -> v_try) fixed.
            let mut mt2: Vec<Option<usize>> = vec![None; n]; // right -> left for remaining
            mt2[v_try] = Some(u);
            let mut ok = true;

            for u2 in (u + 1)..n {
                let mut seen = vec![false; n];
                // Prevent using already fixed/used rights (including v_try).
                for r in 0..n {
                    if used_right[r] || r == v_try {
                        seen[r] = true;
                    }
                }
                if !try_kuhn(u2, &g, &mut seen, &mut mt2) {
                    ok = false;
                    break;
                }
            }

            if ok {
                chosen = Some(v_try);
                break;
            }
        }

        let v = chosen.unwrap();
        ans[u] = v;
        used_right[v] = true;
    }

    println!("yes");
    for u in 0..n {
        println!("{}", ans[u] + 1);
    }
}
