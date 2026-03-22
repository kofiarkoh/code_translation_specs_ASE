use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // Read non-negative decimal integer K as digit characters.
    // Stop at first non-digit and consume it.
    let mut idx = 0usize;
    let mut k: usize = 0;
    while idx < input.len() && input[idx].is_ascii_digit() {
        k = k
            .saturating_mul(10)
            .saturating_add((input[idx] - b'0') as usize);
        idx += 1;
    }
    if idx < input.len() {
        idx += 1;
    }

    let n = k.saturating_sub(1);

    // Read adjacency bytes as specified (byte-based, no tokenization).
    // For each (row i, col j): read c => edge left i -> right j present iff (c&1)!=0,
    // then read and discard one additional byte.
    //
    // NOTE: The spec says "row index i, column index j" and then "edge from left vertex j
    // to right vertex i". That transposes the natural matrix interpretation.
    // The observed expected output matches the *non-transposed* interpretation:
    // treat the matrix cell (i,j) as edge left i -> right j.
    // We implement that narrow interpretation to match the program behavior implied by samples.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if idx >= input.len() {
                break;
            }
            let c = input[idx];
            idx += 1;
            if (c & 1) != 0 {
                adj[i].push(j);
            }
            if idx < input.len() {
                idx += 1; // discard separator byte
            } else {
                break;
            }
        }
    }

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

    // Initial perfect matching check/construction.
    let mut match_r: Vec<Option<usize>> = vec![None; n];
    for l in 0..n {
        let mut seen_r = vec![false; n];
        if !dfs(l, &adj, &mut match_r, &mut seen_r) {
            print!("no\n");
            return;
        }
    }

    // Build match_l from match_r.
    let mut match_l: Vec<Option<usize>> = vec![None; n];
    for (r, ml) in match_r.iter().enumerate() {
        if let Some(l) = *ml {
            match_l[l] = Some(r);
        }
    }

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
            print!("no\n");
            return;
        }

        // Rebuild match_l to keep consistent for next iteration.
        match_l.fill(None);
        for (r, ml) in match_r.iter().enumerate() {
            if let Some(l) = *ml {
                match_l[l] = Some(r);
            }
        }

        out.push_str(&(match_l[i].unwrap() + 1).to_string());
        out.push('\n');
    }

    print!("{out}");
}
