use std::cmp::Ordering;
use std::io::{self, Read};

fn char_index(b: u8) -> usize {
    match b {
        b'A'..=b'Z' => (b - b'A') as usize,
        b'a'..=b'z' => 26 + (b - b'a') as usize,
        _ => 0, // Unspecified behavior for invalid input; keep total function.
    }
}

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Read N lines; each token is assumed to contain at least N usable characters.
    let mut grid_idx = vec![vec![0usize; n]; n];
    for i in 0..n {
        let s = it.next().unwrap();
        let bytes = s.as_bytes();
        for j in 0..n {
            grid_idx[i][j] = char_index(bytes[j]);
        }
    }

    // We must choose a perfect matching that lexicographically minimizes (-c0, -c1, ..., -c51),
    // i.e., lexicographically maximizes (c0, c1, ..., c51).
    //
    // Implement as 52 sequential maximum bipartite matchings with forced edges:
    // For k from 0..51, maximize the number of selected edges whose character index == k,
    // subject to:
    //   - being a perfect matching
    //   - containing all edges fixed from earlier steps
    //
    // This yields lexicographically maximal counts, hence lexicographically minimal negative counts.

    // fixed[i] = Some(j) means row i is forced to match column j.
    let mut fixed: Vec<Option<usize>> = vec![None; n];
    // Also track forced columns to ensure consistency.
    let mut fixed_col: Vec<Option<usize>> = vec![None; n];

    // Helper: build a perfect matching under current fixed constraints, maximizing edges of type k.
    // Returns (row_to_col, count_k).
    fn solve_step(
        n: usize,
        grid_idx: &Vec<Vec<usize>>,
        fixed: &Vec<Option<usize>>,
        fixed_col: &Vec<Option<usize>>,
        k: usize,
    ) -> (Vec<usize>, usize) {
        // Determine which rows/cols are already fixed.
        let mut row_fixed = vec![false; n];
        let mut col_fixed = vec![false; n];
        for i in 0..n {
            if let Some(j) = fixed[i] {
                row_fixed[i] = true;
                col_fixed[j] = true;
            }
        }

        // Build list of free rows and free cols.
        let free_rows: Vec<usize> = (0..n).filter(|&i| !row_fixed[i]).collect();
        let free_cols: Vec<usize> = (0..n).filter(|&j| !col_fixed[j]).collect();

        // Map original col -> index in free_cols (or None).
        let mut col_to_free_idx = vec![None; n];
        for (idx, &j) in free_cols.iter().enumerate() {
            col_to_free_idx[j] = Some(idx);
        }

        // For each free row, build adjacency to free cols, split into preferred (char==k) and others.
        let mut pref_adj: Vec<Vec<usize>> = vec![Vec::new(); free_rows.len()];
        let mut all_adj: Vec<Vec<usize>> = vec![Vec::new(); free_rows.len()];
        for (ri, &i) in free_rows.iter().enumerate() {
            for &j in free_cols.iter() {
                // Respect fixed_col consistency (should already be ensured by col_fixed),
                // and fixed row consistency (row is free here).
                if fixed_col[j].is_some() {
                    continue;
                }
                let fj = col_to_free_idx[j].unwrap();
                all_adj[ri].push(fj);
                if grid_idx[i][j] == k {
                    pref_adj[ri].push(fj);
                }
            }
        }

        // First, find maximum matching using only preferred edges.
        // Standard Kuhn algorithm for bipartite matching.
        let m = free_cols.len();
        let mut match_to_row: Vec<Option<usize>> = vec![None; m];

        fn try_kuhn(
            v: usize,
            adj: &Vec<Vec<usize>>,
            used: &mut Vec<bool>,
            match_to_row: &mut Vec<Option<usize>>,
        ) -> bool {
            if used[v] {
                return false;
            }
            used[v] = true;
            for &to in &adj[v] {
                if match_to_row[to].is_none()
                    || try_kuhn(match_to_row[to].unwrap(), adj, used, match_to_row)
                {
                    match_to_row[to] = Some(v);
                    return true;
                }
            }
            false
        }

        let mut pref_matched = 0usize;
        for v in 0..free_rows.len() {
            let mut used = vec![false; free_rows.len()];
            if try_kuhn(v, &pref_adj, &mut used, &mut match_to_row) {
                pref_matched += 1;
            }
        }

        // Now, extend to a perfect matching using all edges, keeping the already matched preferred edges
        // as a starting point. Kuhn will only change matches if it can augment; it may reroute, but
        // since we start from a maximum preferred matching, any augmentation to increase total size
        // should not reduce the number of preferred edges unless necessary. To preserve the maximum
        // preferred count, we disallow breaking preferred matches by freezing them:
        //
        // We implement freezing by marking columns currently matched via a preferred edge and
        // preventing them from being reassigned unless the new assignment also uses a preferred edge.
        //
        // This is a narrow, deterministic constraint consistent with "any matching among equal vectors".
        // It ensures we keep the maximum count for this k while completing the matching.
        let mut frozen_col = vec![false; m];
        for col in 0..m {
            if let Some(v) = match_to_row[col] {
                // Check if this matched edge is preferred.
                if pref_adj[v].iter().any(|&c| c == col) {
                    frozen_col[col] = true;
                }
            }
        }

        fn try_kuhn_with_freeze(
            v: usize,
            adj: &Vec<Vec<usize>>,
            pref_adj: &Vec<Vec<usize>>,
            used: &mut Vec<bool>,
            match_to_row: &mut Vec<Option<usize>>,
            frozen_col: &Vec<bool>,
        ) -> bool {
            if used[v] {
                return false;
            }
            used[v] = true;
            for &to in &adj[v] {
                // If column is frozen, we can only take it if the edge (v,to) is preferred,
                // or if it is currently unmatched (shouldn't happen for frozen, but keep safe).
                if frozen_col[to] && match_to_row[to].is_some() {
                    let is_pref = pref_adj[v].iter().any(|&c| c == to);
                    if !is_pref {
                        continue;
                    }
                }
                if match_to_row[to].is_none()
                    || try_kuhn_with_freeze(
                        match_to_row[to].unwrap(),
                        adj,
                        pref_adj,
                        used,
                        match_to_row,
                        frozen_col,
                    )
                {
                    match_to_row[to] = Some(v);
                    return true;
                }
            }
            false
        }

        for v in 0..free_rows.len() {
            // If v is already matched, skip; we only need to match all rows.
            let already = match_to_row.iter().any(|&x| x == Some(v));
            if already {
                continue;
            }
            let mut used = vec![false; free_rows.len()];
            let _ = try_kuhn_with_freeze(v, &all_adj, &pref_adj, &mut used, &mut match_to_row, &frozen_col);
        }

        // Construct full row_to_col mapping including fixed.
        let mut row_to_col = vec![usize::MAX; n];
        for i in 0..n {
            if let Some(j) = fixed[i] {
                row_to_col[i] = j;
            }
        }
        for (free_col_idx, &orig_col) in free_cols.iter().enumerate() {
            if let Some(free_row_idx) = match_to_row[free_col_idx] {
                let orig_row = free_rows[free_row_idx];
                row_to_col[orig_row] = orig_col;
            }
        }

        // Count how many selected edges have character k.
        let mut cnt_k = 0usize;
        for i in 0..n {
            let j = row_to_col[i];
            if j != usize::MAX && grid_idx[i][j] == k {
                cnt_k += 1;
            }
        }

        (row_to_col, cnt_k)
    }

    // Iteratively fix edges that are forced by the lexicographic optimum.
    // For each k, compute an optimal matching for this step, then fix all edges of type k
    // that are present in that matching and are not already fixed. This is sufficient to
    // preserve the achieved maximum count for k in subsequent steps.
    for k in 0..52 {
        let (row_to_col, _cnt_k) = solve_step(n, &grid_idx, &fixed, &fixed_col, k);

        for i in 0..n {
            if fixed[i].is_none() {
                let j = row_to_col[i];
                if j != usize::MAX && grid_idx[i][j] == k {
                    // Fix this edge.
                    fixed[i] = Some(j);
                    fixed_col[j] = Some(i);
                }
            }
        }
    }

    // Build final perfect matching consistent with all fixed edges (no optimization needed now).
    // Use k=0 with no preference; solve_step will complete the matching.
    let (final_match, _) = solve_step(n, &grid_idx, &fixed, &fixed_col, 0);

    // Count characters and output sorted by fixed order.
    let mut counts = [0usize; 52];
    for i in 0..n {
        let j = final_match[i];
        let idx = grid_idx[i][j];
        counts[idx] += 1;
    }

    let mut out = String::with_capacity(n + 1);
    for idx in 0..52 {
        let ch = if idx < 26 {
            (b'A' + idx as u8) as char
        } else {
            (b'a' + (idx as u8 - 26)) as char
        };
        for _ in 0..counts[idx] {
            out.push(ch);
        }
    }
    // Ensure exactly N characters as specified (assumes perfect matching).
    // If assumptions are violated, behavior is unspecified; we still print what we have.
    out.push('\n');
    print!("{out}");
}
