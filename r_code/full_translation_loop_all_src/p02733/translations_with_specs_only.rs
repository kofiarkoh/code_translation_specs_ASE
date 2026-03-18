use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    let mut s: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        s.push(it.next().unwrap().as_bytes().to_vec());
    }

    // Interpret the task as:
    // Choose a partition of rows into contiguous segments, but we are also allowed to "select"
    // subsets of columns by first partitioning rows into contiguous groups (via horizontal cuts),
    // and then for each group we count '1's per column; we then scan columns left-to-right and
    // create vertical segments so that in each rectangle (row-group x column-segment) the number
    // of '1's does not exceed K. Minimize total cuts = horizontal_cuts + vertical_cuts.
    //
    // This matches the known structure where we enumerate horizontal partitions (2^(H-1)) and
    // greedily compute minimal vertical cuts for each.
    //
    // If any rectangle is impossible (count > K even for a single column segment), that horizontal
    // partition is invalid.
    let mut ans: i32 = i32::MAX;

    // Precompute per-row per-column bits as 0/1 i32 for faster sums.
    let mut a = vec![vec![0i32; w]; h];
    for i in 0..h {
        for j in 0..w {
            a[i][j] = if s[i][j] == b'1' { 1 } else { 0 };
        }
    }

    // Enumerate horizontal cuts between rows: mask over H-1 positions.
    for mask in 0..(1usize << (h.saturating_sub(1))) {
        // group_id for each row
        let mut gid = vec![0usize; h];
        let mut g = 0usize;
        gid[0] = 0;
        for i in 0..h - 1 {
            if (mask >> i) & 1 == 1 {
                g += 1;
            }
            gid[i + 1] = g;
        }
        let groups = g + 1;

        // For each column, compute counts of ones per group.
        let mut col_group = vec![vec![0i32; groups]; w];
        for i in 0..h {
            let gr = gid[i];
            for j in 0..w {
                col_group[j][gr] += a[i][j];
            }
        }

        // Greedily decide vertical cuts.
        let mut cuts: i32 = mask.count_ones() as i32; // horizontal cuts
        let mut acc = vec![0i32; groups]; // accumulated counts in current vertical segment
        let mut ok = true;

        for j in 0..w {
            // Try to add column j to current segment.
            let mut need_cut = false;
            for gr in 0..groups {
                if acc[gr] + col_group[j][gr] > k {
                    need_cut = true;
                    break;
                }
            }

            if need_cut {
                // Start new vertical segment before column j.
                cuts += 1;
                acc.fill(0);
            }

            // Add column j; if still exceeds, this partition is impossible.
            for gr in 0..groups {
                acc[gr] += col_group[j][gr];
                if acc[gr] > k {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }

        if ok && cuts < ans {
            ans = cuts;
        }
    }

    // Under the preconditions typical for this problem, a solution exists.
    // If not, ans would remain MAX; we print it as-is to avoid inventing behavior.
    println!("{}", ans);
}
