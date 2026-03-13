// Translated from the provided C code.
// Notes on translation choices:
// - The original C used `char data[99][50]` but read integers into it with scanf("%d%c", ...),
//   which is a type mismatch / undefined behavior in C. The apparent intention is to store
//   integers in a 2D structure indexed from 1 per row (with index 0 used as a sentinel).
// - To preserve the intended behavior, we parse the stdin as rows of comma-separated integers,
//   store each row as Vec<i32> with a leading zero at index 0 and an extra trailing zero (to
//   emulate data[i][j+1] = 0 in the C code).
// - dp is represented as Vec<Vec<i32>> initialized to zeros, mirroring the global zero-init in C.
// - The recursive `judge` function follows the same logic and pruning as the C version,
//   including the comparison `sum < dp[i][j]`.
// - Indices in the C code start at j == 1 for the first element on each row; we preserve that.
// - I/O formatting: output is exactly a single integer followed by '\n'.

use std::io::{self, Read};

fn max_i32(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn judge(
    i: usize,
    j: usize,
    sum: i32,
    n: usize,
    data: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    // sum += data[i][j];
    // If indices are out-of-bounds it would be UB in C; we assume input ensures valid indices
    let sum = sum.wrapping_add(data[i][j]); // use wrapping_add to mirror potential C wraparound
    if i == n || sum < dp[i][j] {
        return sum;
    } else {
        dp[i][j] = sum;
        if i < n / 2 {
            let a = judge(i + 1, j, dp[i][j], n, data, dp);
            let b = judge(i + 1, j + 1, dp[i][j], n, data, dp);
            return max_i32(a, b);
        } else {
            // j should be >= 1 here per typical input; preserve behavior assuming valid input
            let a = judge(i + 1, j - 1, dp[i][j], n, data, dp);
            let b = judge(i + 1, j, dp[i][j], n, data, dp);
            return max_i32(a, b);
        }
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse into rows of integers.
    // Emulate scanf("%d%c" ... ) behavior where numbers on a line are separated by ',' and
    // lines are separated by newline. We treat '\r' and '\n' as line separators.
    let mut data: Vec<Vec<i32>> = Vec::new();

    for raw_line in input.lines() {
        // Trim to remove stray whitespace; the original scanf would accept spaces around numbers for %d
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<i32> = Vec::new();
        // Index 0 is a sentinel zero in the original C code
        row.push(0);

        for token in line.split(',') {
            let tok = token.trim();
            if tok.is_empty() {
                // If there's an empty token, skip (C behavior with malformed input is undefined).
                continue;
            }
            // Parse as i32. On parse failure, treat as 0 to avoid panics (mirrors permissive C behavior).
            let val = tok.parse::<i32>().unwrap_or(0);
            row.push(val);
        }

        // The C code set data[i][j+1] = 0 at end of row; append an extra 0 to mirror that.
        row.push(0);

        data.push(row);
    }

    let n = data.len();

    // Build dp with same shape as data, filled with 0s (C global arrays are zero-initialized).
    let mut dp: Vec<Vec<i32>> = data.iter().map(|row| vec![0i32; row.len()]).collect();

    // If there is no data, the original C would set n = 0 and then call judge(0,1,0),
    // which would attempt to access data[0][1] (UB). We avoid panic by handling empty input:
    if n == 0 {
        println!("0");
        return;
    }

    // Call judge(0, 1, 0)
    let ans = judge(0, 1, 0, n, &data, &mut dp);
    println!("{}", ans);
}
