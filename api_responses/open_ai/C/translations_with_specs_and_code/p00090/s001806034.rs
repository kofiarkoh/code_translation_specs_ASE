// aoj0090.rs
// Translated from the given C program according to the provided specification.
// Preserves original semantics (including input parsing semantics by treating commas
// as separators between the two doubles per point).

use std::io::{self, Read};

fn main() {
    // Read entire stdin, replace commas with whitespace so that tokens like "1.0,2.0"
    // become "1.0 2.0" and can be parsed by split_whitespace()
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");
    let prepared = input.replace(',', " ");
    let tokens: Vec<&str> = prepared.split_whitespace().collect();

    let mut idx: usize = 0;
    let mut out_lines: Vec<String> = Vec::new();

    // Helper to read next token as i32 or f64; per spec we assume well-formed input.
    while idx < tokens.len() {
        // Read n
        let n_token = tokens[idx];
        idx += 1;
        let n_parsed = match n_token.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break, // malformed input; mimic unspecified behavior by stopping
        };
        let n = n_parsed as usize;
        if n == 0 {
            break;
        }

        // Read n pairs of doubles
        if idx + 2 * n > tokens.len() {
            break; // insufficient fields; stop
        }
        let mut circle: Vec<(f64, f64)> = Vec::with_capacity(n);
        for _ in 0..n {
            let x = tokens[idx].parse::<f64>().unwrap_or(0.0);
            let y = tokens[idx + 1].parse::<f64>().unwrap_or(0.0);
            idx += 2;
            circle.push((x, y));
        }

        // Build adjacency matrix flag[n][n]
        let mut flag: Vec<Vec<bool>> = vec![vec![false; n]; n];
        for i in 0..n {
            for j in i..n {
                if i == j {
                    flag[i][j] = true;
                } else {
                    let dx = circle[i].0 - circle[j].0;
                    let dy = circle[i].1 - circle[j].1;
                    let d2 = dx * dx + dy * dy;
                    if d2 <= 4.0 {
                        flag[i][j] = true;
                        flag[j][i] = true;
                    }
                }
            }
        }

        // Pruning vertices with degree 1 or 2 iteratively.
        // Keep semantics identical to C:
        // - Compute num for all vertices at start of each iteration
        // - If any num[i] == 2 then set max_overlap = 2
        // - If any num[i] == 1 or 2, remove that vertex (clear its row and column)
        // - Repeat until no removals occur. After loop, num values from last computed
        //   iteration (which had no removals) are used later.
        let mut max_overlap: i32 = 1;
        let mut num: Vec<i32> = vec![0; n];

        loop {
            // compute num
            for i in 0..n {
                let mut cnt = 0;
                for j in 0..n {
                    if flag[i][j] {
                        cnt += 1;
                    }
                }
                num[i] = cnt;
            }

            let mut cont_flag = false;
            for i in 0..n {
                if num[i] == 2 {
                    max_overlap = 2;
                }
                if num[i] == 1 || num[i] == 2 {
                    // clear row and column i
                    for j in 0..n {
                        flag[i][j] = false;
                        flag[j][i] = false;
                    }
                    cont_flag = true;
                }
            }
            if !cont_flag {
                break;
            }
            // otherwise loop again (num will be recomputed)
        }

        // Compute overlap counts as in C code
        let mut overlap: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            if num[i] > 0 {
                for j in 0..n {
                    if flag[i][j] {
                        let mut k = j + 1;
                        while k < n {
                            if flag[i][k] && flag[j][k] {
                                overlap[i][j] += 1;
                                overlap[i][k] += 1;
                            }
                            k += 1;
                        }
                    }
                }
            }
        }

        // compute per-vertex max and min (min initialized to 1000 per C code)
        let mut maxv: Vec<i32> = vec![0; n];
        let mut minv: Vec<i32> = vec![1000; n];
        for i in 0..n {
            for j in 0..n {
                if maxv[i] < overlap[i][j] {
                    maxv[i] = overlap[i][j];
                }
                if overlap[i][j] > 0 && minv[i] > overlap[i][j] {
                    minv[i] = overlap[i][j];
                }
            }
        }

        // final update of global max_overlap according to the C logic
        for i in 0..n {
            for j in 0..n {
                if overlap[i][j] < overlap[i][i] || maxv[i] == minv[i] {
                    if max_overlap < overlap[i][j] {
                        max_overlap = overlap[i][j];
                    }
                }
            }
        }

        // Output max_overlap + 1
        out_lines.push(format!("{}", max_overlap + 1));
    }

    // Print all outputs, each on its own line
    for line in out_lines {
        println!("{}", line);
    }
}
