use std::io::{self, Read};

fn main() {
    // Read all input; behavior for missing/invalid input is unspecified by the spec.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut it = s.split_whitespace();
    let mut m = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            m[i][j] = it.next().unwrap().parse::<i64>().unwrap();
        }
    }

    // Normalize by subtracting a row-specific minimum value from each row.
    // This preserves decomposability per the spec's invariant.
    for i in 0..3 {
        let row_min = m[i][0].min(m[i][1]).min(m[i][2]);
        for j in 0..3 {
            m[i][j] -= row_min;
        }
    }

    // After normalization, the matrix admits a decomposition iff each column
    // has identical values across all three rows.
    for j in 0..3 {
        if !(m[0][j] == m[1][j] && m[1][j] == m[2][j]) {
            println!("No");
            return; // terminate immediately on rejection
        }
    }

    println!("Yes");
}
