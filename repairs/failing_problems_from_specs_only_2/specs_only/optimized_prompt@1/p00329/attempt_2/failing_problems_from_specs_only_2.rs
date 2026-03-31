use std::io::{self, Read};

fn main() {
    // Read entire stdin as raw bytes; the spec is byte-based.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut p: usize = 0;

    // Read non-negative decimal integer K as digit characters.
    // Stops at first non-digit and consumes it (if present).
    let mut k: usize = 0;
    while p < input.len() && input[p].is_ascii_digit() {
        k = k * 10 + (input[p] - b'0') as usize;
        p += 1;
    }
    if p < input.len() {
        p += 1;
    }

    // N = K - 1 (precondition: K >= 1).
    let n: usize = k.wrapping_sub(1);

    // Build adjacency lists in the exact storage/traversal order implied by input consumption:
    // for i in 0..N (right index), for j in 0..N (left index), read c then sep.
    // Edge from left j to right i present iff (c & 1) != 0.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            let c = input[p];
            p += 1;
            let _sep = input[p];
            p += 1;
            if (c & 1) != 0 {
                adj[j].push(i);
            }
        }
    }

    // Standard Kuhn DFS for augmenting paths.
    fn dfs(
        v_left: usize,
        adj: &[Vec<usize>],
        used_left: &mut [bool],
        match_r_to_l: &mut [Option<usize>],
    ) -> bool {
        if used_left[v_left] {
            return false;
        }
        used_left[v_left] = true;
        for &r in &adj[v_left] {
            if match_r_to_l[r].is_none()
                || dfs(match_r_to_l[r].unwrap(), adj, used_left, match_r_to_l)
            {
                match_r_to_l[r] = Some(v_left);
                return true;
            }
        }
        false
    }

    // First compute whether any perfect matching exists.
    let mut match_r_to_l: Vec<Option<usize>> = vec![None; n];
    for v in 0..n {
        let mut used_left = vec![false; n];
        if !dfs(v, &adj, &mut used_left, &mut match_r_to_l) {
            print!("no\n");
            return;
        }
    }

    // Invert to left->right.
    let mut match_l_to_r: Vec<Option<usize>> = vec![None; n];
    for r in 0..n {
        if let Some(l) = match_r_to_l[r] {
            match_l_to_r[l] = Some(r);
        }
    }

    // Output phase:
    // For each left vertex i in increasing order:
    // - remove i and its currently matched partner (if any)
    // - run augmenting-path routine starting from i to re-establish a perfect matching
    // - print resulting partner of i
    //
    // Important: the intended behavior is to keep previously printed partners fixed.
    // Therefore, after printing partner for i, we "lock" left vertex i by preventing
    // future augmentations from reassigning it. This matches the sample behavior.
    let mut locked_left = vec![false; n];

    fn dfs_with_locks(
        v_left: usize,
        adj: &[Vec<usize>],
        used_left: &mut [bool],
        match_r_to_l: &mut [Option<usize>],
        locked_left: &[bool],
    ) -> bool {
        if used_left[v_left] {
            return false;
        }
        used_left[v_left] = true;
        for &r in &adj[v_left] {
            match match_r_to_l[r] {
                None => {
                    match_r_to_l[r] = Some(v_left);
                    return true;
                }
                Some(l2) => {
                    if locked_left[l2] {
                        continue;
                    }
                    if dfs_with_locks(l2, adj, used_left, match_r_to_l, locked_left) {
                        match_r_to_l[r] = Some(v_left);
                        return true;
                    }
                }
            }
        }
        false
    }

    let mut out = String::new();
    out.push_str("yes\n");

    for i in 0..n {
        // Remove i and its currently matched partner (if any).
        if let Some(r) = match_l_to_r[i] {
            match_l_to_r[i] = None;
            match_r_to_l[r] = None;
        }

        // Re-augment from i, respecting locks so earlier outputs remain unchanged.
        let mut used_left = vec![false; n];
        let ok = dfs_with_locks(i, &adj, &mut used_left, &mut match_r_to_l, &locked_left);
        if !ok {
            // If this happens despite initial perfect matching, behavior is unspecified;
            // choose narrow failure reporting.
            print!("no\n");
            return;
        }

        // Recompute left->right mapping.
        match_l_to_r.fill(None);
        for r in 0..n {
            if let Some(l) = match_r_to_l[r] {
                match_l_to_r[l] = Some(r);
            }
        }

        let partner_r = match_l_to_r[i].unwrap();
        out.push_str(&(partner_r + 1).to_string());
        out.push('\n');

        // Lock i so future steps cannot change its assignment.
        locked_left[i] = true;
    }

    print!("{out}");
}
