use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let n: usize = it.next().unwrap().parse().unwrap();

    // Grid stored as 0/1 bytes.
    let mut grid: Vec<Vec<u8>> = vec![vec![0u8; n]; n];

    for i in 0..n {
        let s = it.next().unwrap();
        // Spec: row has at least N binary characters; we take first N.
        let bytes = s.as_bytes();
        for j in 0..n {
            grid[i][j] = if bytes[j] == b'1' { 1 } else { 0 };
        }
    }

    // Maintain aggregate count of satisfied local checks:
    // For each cell (i,j), it contributes:
    //   (grid[i][j] == grid[n-1-i][j]) + (grid[i][j] == grid[i][n-1-j])
    // Total checks = 2 * n * n. Property holds iff all checks satisfied.
    let total_checks: i64 = 2i64 * (n as i64) * (n as i64);
    let mut ok_checks: i64 = 0;

    for i in 0..n {
        for j in 0..n {
            let v = grid[i][j];
            let vh = grid[n - 1 - i][j];
            let vv = grid[i][n - 1 - j];
            if v == vh {
                ok_checks += 1;
            }
            if v == vv {
                ok_checks += 1;
            }
        }
    }

    let mut good_states: i64 = 0;
    if ok_checks == total_checks {
        good_states += 1;
    }

    // Helper to update ok_checks for a single cell toggle at (r,c) 0-based.
    // We update only the checks that can change: those involving (r,c) and its
    // horizontal/vertical mirror partners, for both check types.
    let mut apply_toggle = |r: usize, c: usize, grid: &mut Vec<Vec<u8>>, ok_checks: &mut i64| {
        let rh = n - 1 - r;
        let cv = n - 1 - c;

        // Collect unique affected cells among { (r,c), (rh,c), (r,cv) }.
        // These are the only cells whose local checks can change due to toggling (r,c).
        let mut cells: [(usize, usize); 3] = [(r, c), (rh, c), (r, cv)];
        // Deduplicate in-place (n is small enough; constant size anyway).
        let mut uniq: [(usize, usize); 3] = [(0, 0); 3];
        let mut m = 0usize;
        for k in 0..3 {
            let (a, b) = cells[k];
            let mut seen = false;
            for u in 0..m {
                if uniq[u] == (a, b) {
                    seen = true;
                    break;
                }
            }
            if !seen {
                uniq[m] = (a, b);
                m += 1;
            }
        }

        // Subtract old contributions for affected cells.
        for idx in 0..m {
            let (i, j) = uniq[idx];
            let v = grid[i][j];
            let vh = grid[n - 1 - i][j];
            let vv = grid[i][n - 1 - j];
            if v == vh {
                *ok_checks -= 1;
            }
            if v == vv {
                *ok_checks -= 1;
            }
        }

        // Toggle the target cell.
        grid[r][c] ^= 1;

        // Add new contributions for affected cells.
        for idx in 0..m {
            let (i, j) = uniq[idx];
            let v = grid[i][j];
            let vh = grid[n - 1 - i][j];
            let vv = grid[i][n - 1 - j];
            if v == vh {
                *ok_checks += 1;
            }
            if v == vv {
                *ok_checks += 1;
            }
        }
    };

    // Process remaining states (t-1 batches).
    for _ in 1..t {
        let k: usize = it.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let r1: usize = it.next().unwrap().parse().unwrap();
            let c1: usize = it.next().unwrap().parse().unwrap();
            // Spec: coordinates are 1-based and valid.
            let r = r1 - 1;
            let c = c1 - 1;
            apply_toggle(r, c, &mut grid, &mut ok_checks);
        }
        if ok_checks == total_checks {
            good_states += 1;
        }
    }

    print!("{}", good_states);
}
