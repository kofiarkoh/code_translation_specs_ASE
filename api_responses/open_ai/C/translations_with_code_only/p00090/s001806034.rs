// AOJ0090 Overlaps of Seals -- translated from given C code
//
// Notes:
// - Behavior follows the C code as the single source of truth.
// - Input expects integers and coordinate pairs like "x,y" (commas are treated as separators).
// - The C code used arrays sized up to 200; this translation uses Vecs sized by `n`.
// - All semantics (including the iterative pruning of degree-1/2 nodes and triangle counting)
//   are preserved.
//
// This is safe, idiomatic Rust (no unsafe needed).

use std::io::{self, Read};

fn main() {
    // Read entire stdin and replace commas with spaces so tokens are easier to split.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");
    let s = s.replace(',', " ");
    let tokens: Vec<&str> = s.split_whitespace().collect();
    let mut pos = 0usize;

    // Helper to get next token as T
    macro_rules! next_token {
        () => {{
            if pos >= tokens.len() {
                None
            } else {
                let t = tokens[pos];
                pos += 1;
                Some(t)
            }
        }};
    }

    loop {
        // Read n
        let n_token = match next_token!() {
            Some(t) => t,
            None => break,
        };
        let n: usize = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        // Read n coordinate pairs
        let mut circle: Vec<[f64; 2]> = vec![[0.0, 0.0]; n];
        for i in 0..n {
            // Expect two tokens (x and y) after comma replacement
            let xt = next_token!().expect("expected x coordinate");
            let yt = next_token!().expect("expected y coordinate");
            let x: f64 = xt.parse().expect("parse x");
            let y: f64 = yt.parse().expect("parse y");
            circle[i][0] = x;
            circle[i][1] = y;
        }

        // flag adjacency matrix (0/1)
        let mut flag: Vec<Vec<i32>> = vec![vec![0; n]; n];

        // Build adjacency: distance squared <= 4.0 -> connected. diagonal true.
        for i in 0..n {
            for j in i..n {
                if i == j {
                    flag[i][j] = 1;
                } else {
                    let dx = circle[i][0] - circle[j][0];
                    let dy = circle[i][1] - circle[j][1];
                    let d = dx * dx + dy * dy;
                    if d <= 4.0 {
                        flag[i][j] = 1;
                        flag[j][i] = 1;
                    }
                }
            }
        }

        // Iterative pruning loop (do { ... } while (cont_flag == 1); )
        let mut max_overlap: i32 = 1;
        let mut num: Vec<i32> = vec![0; n];

        loop {
            // compute num[i] = sum flag[i][j]
            for i in 0..n {
                let mut sum = 0;
                for j in 0..n {
                    sum += flag[i][j];
                }
                num[i] = sum;
            }

            let mut cont_flag = 0;
            for i in 0..n {
                if num[i] == 2 {
                    max_overlap = 2;
                }
                if num[i] == 1 || num[i] == 2 {
                    // remove node i from graph
                    for j in 0..n {
                        flag[i][j] = 0;
                        flag[j][i] = 0;
                    }
                    cont_flag = 1;
                }
            }

            if cont_flag == 0 {
                break;
            }
            // else repeat (this matches do-while: compute then possibly repeat)
        }

        // overlap matrix initialized to zero
        let mut overlap: Vec<Vec<i32>> = vec![vec![0; n]; n];

        // Count triangles and increment overlap counts per the C logic
        for i in 0..n {
            if num[i] > 0 {
                for j in 0..n {
                    if flag[i][j] == 1 {
                        let mut k = j + 1;
                        while k < n {
                            if flag[i][k] == 1 {
                                if flag[j][k] == 1 {
                                    overlap[i][j] += 1;
                                    overlap[i][k] += 1;
                                }
                            }
                            k += 1;
                        }
                    }
                }
            }
        }

        // compute max[i] and min[i] per row
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

        // Final selection to possibly increase max_overlap
        for i in 0..n {
            for j in 0..n {
                if overlap[i][j] < overlap[i][i] || maxv[i] == minv[i] {
                    if max_overlap < overlap[i][j] {
                        max_overlap = overlap[i][j];
                    }
                }
            }
        }

        // Print result: max_overlap + 1
        println!("{}", max_overlap + 1);
    }
}
