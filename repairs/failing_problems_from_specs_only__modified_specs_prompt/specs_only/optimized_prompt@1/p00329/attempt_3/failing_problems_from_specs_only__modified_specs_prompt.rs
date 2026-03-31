use std::io::{self, Read};

fn try_kuhn(
    v_left: usize,
    g: &[Vec<usize>],
    used_left: &mut [u32],
    used_mark: u32,
    mt_right_to_left: &mut [Option<usize>],
    mt_left_to_right: &mut [Option<usize>],
) -> bool {
    if used_left[v_left] == used_mark {
        return false;
    }
    used_left[v_left] = used_mark;

    for &to_right in &g[v_left] {
        match mt_right_to_left[to_right] {
            None => {
                mt_right_to_left[to_right] = Some(v_left);
                mt_left_to_right[v_left] = Some(to_right);
                return true;
            }
            Some(prev_left) => {
                if try_kuhn(
                    prev_left,
                    g,
                    used_left,
                    used_mark,
                    mt_right_to_left,
                    mt_left_to_right,
                ) {
                    mt_right_to_left[to_right] = Some(v_left);
                    mt_left_to_right[v_left] = Some(to_right);
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut p: usize = 0;

    // Parse N_in as a non-negative decimal integer.
    while p < input.len() && !(input[p] as char).is_ascii_digit() {
        p += 1;
    }
    let mut n_in: usize = 0;
    while p < input.len() && (input[p] as char).is_ascii_digit() {
        n_in = n_in.saturating_mul(10).saturating_add((input[p] - b'0') as usize);
        p += 1;
    }

    // Effective size N = N_in - 1 (as specified).
    let n: usize = if n_in == 0 { 0 } else { n_in - 1 };

    // Read matrix: for each cell, read a value character (skipping whitespace),
    // then consume exactly one additional character (ignored).
    // This matches typical C patterns like " %c%*c" used with spaced matrices.
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            while p < input.len() && (input[p] as char).is_ascii_whitespace() {
                p += 1;
            }
            let val = if p < input.len() { input[p] } else { 0 };
            if p < input.len() {
                p += 1;
            }
            if p < input.len() {
                p += 1; // ignore one extra character (typically a separator)
            }

            if (val & 1) == 1 {
                // Edge from left=j to right=i (transpose interpretation).
                g[j].push(i);
            }
        }
    }

    // Initial perfect matching.
    let mut mt_right_to_left: Vec<Option<usize>> = vec![None; n];
    let mut mt_left_to_right: Vec<Option<usize>> = vec![None; n];

    let mut used_left: Vec<u32> = vec![0; n];
    let mut mark: u32 = 1;

    let mut matched = 0usize;
    for v in 0..n {
        if try_kuhn(
            v,
            &g,
            &mut used_left,
            mark,
            &mut mt_right_to_left,
            &mut mt_left_to_right,
        ) {
            matched += 1;
        }
        mark = mark.wrapping_add(1);
        if mark == 0 {
            used_left.fill(0);
            mark = 1;
        }
    }

    if matched != n {
        print!("no\n");
        return;
    }

    print!("yes\n");

    // Sequential rematching procedure.
    //
    // Critical detail: the original procedure removes the matched edge for i,
    // then runs one DFS from i with a *fresh* visited array (all false/zero),
    // not just a new "mark". This affects traversal and thus the chosen partner.
    for i in 0..n {
        if let Some(r) = mt_left_to_right[i] {
            mt_left_to_right[i] = None;
            mt_right_to_left[r] = None;
        }

        used_left.fill(0);
        let _ = try_kuhn(
            i,
            &g,
            &mut used_left,
            1,
            &mut mt_right_to_left,
            &mut mt_left_to_right,
        );

        let out = match mt_left_to_right[i] {
            Some(r) => r + 1,
            None => 0,
        };
        print!("{out}\n");
    }
}
