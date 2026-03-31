use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut p: usize = 0;

    // Read non-negative decimal integer K as digit characters.
    // Stops at first non-digit and consumes it (if present).
    let mut k: usize = 0;
    while p < input.len() && input[p].is_ascii_digit() {
        k = k
            .wrapping_mul(10)
            .wrapping_add((input[p] - b'0') as usize);
        p += 1;
    }
    if p < input.len() {
        p += 1; // consume first non-digit
    }

    // N = K - 1 (precondition: K >= 1)
    let n: usize = k.wrapping_sub(1);

    // Read adjacency: exactly 2*N*N bytes, for each (i,j):
    // read byte c => edge from left j to right i present iff (c & 1) != 0
    // then read and discard one byte.
    //
    // Store edges in the order they are encountered for each left vertex.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            let c = input[p];
            p += 1;
            if (c & 1) != 0 {
                adj[j].push(i);
            }
            p += 1; // discard separator byte
        }
    }

    // Augmenting path routine with deterministic traversal order:
    // - left vertices explored in increasing index order
    // - right neighbors explored in stored order
    //
    // match_r[r] = Some(l) means right r matched to left l.
    fn try_augment(
        start_l: usize,
        adj: &[Vec<usize>],
        match_r: &mut [Option<usize>],
    ) -> bool {
        let n = adj.len();
        let mut seen_l = vec![false; n];
        let mut seen_r = vec![false; n];

        fn dfs(
            v: usize,
            adj: &[Vec<usize>],
            seen_l: &mut [bool],
            seen_r: &mut [bool],
            match_r: &mut [Option<usize>],
        ) -> bool {
            if seen_l[v] {
                return false;
            }
            seen_l[v] = true;

            for &r in &adj[v] {
                if seen_r[r] {
                    continue;
                }
                seen_r[r] = true;

                match match_r[r] {
                    None => {
                        match_r[r] = Some(v);
                        return true;
                    }
                    Some(u) => {
                        if dfs(u, adj, seen_l, seen_r, match_r) {
                            match_r[r] = Some(v);
                            return true;
                        }
                    }
                }
            }
            false
        }

        dfs(start_l, adj, &mut seen_l, &mut seen_r, match_r)
    }

    // Compute whether any perfect matching exists.
    let mut match_r: Vec<Option<usize>> = vec![None; n];
    for v in 0..n {
        if !try_augment(v, &adj, &mut match_r) {
            print!("no\n");
            return;
        }
    }

    // Build match_l from match_r.
    fn build_match_l(n: usize, match_r: &[Option<usize>]) -> Vec<Option<usize>> {
        let mut match_l = vec![None; n];
        for (r, &ml) in match_r.iter().enumerate() {
            if let Some(l) = ml {
                match_l[l] = Some(r);
            }
        }
        match_l
    }

    // Output partners for left vertices in increasing order.
    //
    // Important semantic detail:
    // The specification says: "removing i and its currently matched partner (if any)".
    // This implies removing BOTH:
    //   - the edge (i -> r_i) from the matching, and
    //   - the edge (l2 -> r_i) if some other left l2 is also matched to that same right
    //     (i.e., clear the right vertex's match entirely).
    //
    // In our representation match_r[r] is the single owner of right r, so "remove i and its
    // currently matched partner" means: clear match_r[r_i] regardless of which left it points to.
    // This differs from only removing when match_r[r_i] == i, and is required to match the
    // described procedure and sample behavior.
    let mut out = String::new();
    out.push_str("yes\n");

    for i in 0..n {
        let match_l = build_match_l(n, &match_r);

        if let Some(r_i) = match_l[i] {
            // Remove the right partner vertex from the matching entirely.
            match_r[r_i] = None;
        }

        // Re-establish a perfect matching by augmenting from i.
        // Under the specification, this should succeed given a perfect matching existed.
        let ok = try_augment(i, &adj, &mut match_r);
        if !ok {
            // Unspecified by spec; keep output defined.
            out.push_str("0\n");
            continue;
        }

        let match_l2 = build_match_l(n, &match_r);
        let partner_r = match_l2[i].unwrap_or(0);
        out.push_str(&(partner_r + 1).to_string());
        out.push('\n');
    }

    print!("{out}");
}
