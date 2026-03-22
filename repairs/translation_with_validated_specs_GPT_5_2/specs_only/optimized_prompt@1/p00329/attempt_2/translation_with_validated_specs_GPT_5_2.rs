use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // Read non-negative decimal integer K as digit characters.
    // The routine stops at the first non-digit and consumes it.
    let mut idx = 0usize;
    let mut k: usize = 0;
    while idx < input.len() && input[idx].is_ascii_digit() {
        k = k
            .saturating_mul(10)
            .saturating_add((input[idx] - b'0') as usize);
        idx += 1;
    }
    if idx < input.len() {
        idx += 1; // consume the first non-digit
    }

    // N = K - 1 (precondition: K >= 1)
    let n = k.saturating_sub(1);

    // Read adjacency bytes exactly as specified:
    // For each (row i, col j): read byte c => edge from left j to right i present iff (c&1)!=0,
    // then read and discard one additional byte.
    // Store edges in encounter order.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if idx >= input.len() {
                break;
            }
            let c = input[idx];
            idx += 1;
            if (c & 1) != 0 {
                adj[j].push(i);
            }
            if idx < input.len() {
                idx += 1; // discard separator byte
            } else {
                break;
            }
        }
    }

    // Augmenting path routine (Kuhn DFS) with deterministic traversal order.
    fn dfs(
        l: usize,
        adj: &[Vec<usize>],
        match_r: &mut [Option<usize>],
        seen_r: &mut [bool],
    ) -> bool {
        for &r in &adj[l] {
            if seen_r[r] {
                continue;
            }
            seen_r[r] = true;
            if match_r[r].is_none() || dfs(match_r[r].unwrap(), adj, match_r, seen_r) {
                match_r[r] = Some(l);
                return true;
            }
        }
        false
    }

    // Build an initial perfect matching if possible.
    let mut match_r: Vec<Option<usize>> = vec![None; n];
    for l in 0..n {
        let mut seen_r = vec![false; n];
        if !dfs(l, &adj, &mut match_r, &mut seen_r) {
            print!("no\n");
            return;
        }
    }

    // Invert match_r into match_l for convenience.
    let mut match_l: Vec<Option<usize>> = vec![None; n];
    for (r, ml) in match_r.iter().enumerate() {
        if let Some(l) = *ml {
            match_l[l] = Some(r);
        }
    }

    // Output phase per spec:
    // For each left vertex i in increasing order:
    // - remove i and its currently matched partner (if any)
    // - run augmenting-path routine starting from i to re-establish a perfect matching
    // - print resulting partner of i (1-based)
    //
    // Important semantic detail: removing i and its partner must also "free" the left vertex
    // that becomes unmatched due to freeing that right vertex during augmentation.
    // Using only match_r during augmentation and then rebuilding match_l after each step
    // preserves the intended matching state.
    let mut out = String::new();
    out.push_str("yes\n");

    for i in 0..n {
        // Remove i and its currently matched partner (if any).
        if let Some(r) = match_l[i] {
            match_l[i] = None;
            if match_r[r] == Some(i) {
                match_r[r] = None;
            }
        }

        // Re-augment from i.
        let mut seen_r = vec![false; n];
        if !dfs(i, &adj, &mut match_r, &mut seen_r) {
            // If this happens, behavior is unspecified by the spec; choose deterministic failure.
            print!("no\n");
            return;
        }

        // Rebuild match_l from match_r to keep it consistent for the next iteration.
        match_l.fill(None);
        for (r, ml) in match_r.iter().enumerate() {
            if let Some(l) = *ml {
                match_l[l] = Some(r);
            }
        }

        let partner_r = match_l[i].unwrap();
        out.push_str(&(partner_r + 1).to_string());
        out.push('\n');
    }

    print!("{out}");
}
