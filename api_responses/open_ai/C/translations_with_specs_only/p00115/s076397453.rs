use std::io::{self, Read};

const EPS: f64 = 1e-7;

fn main() {
    // Read all whitespace-separated tokens from stdin
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    // Parse tokens as f64. The specification assumes exactly 15 valid doubles are provided.
    // To remain robust while staying close to the spec we parse what is present and
    // use 0.0 for any missing values (the original C code did not check scanf return).
    let mut vals: [f64; 15] = [0.0; 15];
    for (i, tok) in input.split_whitespace().take(15).enumerate() {
        if let Ok(v) = tok.parse::<f64>() {
            vals[i] = v;
        } else {
            // If a token fails to parse, fall back to 0.0 (narrow/benign behavior).
            vals[i] = 0.0;
        }
    }

    // Unpack points: me, enemy, p1, p2, p3
    let me = [vals[0], vals[1], vals[2]];
    let enemy = [vals[3], vals[4], vals[5]];
    let p1 = [vals[6], vals[7], vals[8]];
    let p2 = [vals[9], vals[10], vals[11]];
    let p3 = [vals[12], vals[13], vals[14]];

    // Build coefficient matrix A and right-hand side b for:
    // me + x*(enemy-me) = p1 + y*(p2-p1) + z*(p3-p1)
    // => x*(enemy-me) - y*(p2-p1) - z*(p3-p1) = p1 - me
    //
    // We form augmented matrix mat[row][0..2] = A, mat[row][3] = b
    let mut mat = [[0.0f64; 4]; 3];

    // Columns: A[:,0] = (enemy-me)
    //          A[:,1] = -(p2-p1)
    //          A[:,2] = -(p3-p1)
    // b = p1 - me
    for i in 0..3 {
        mat[i][0] = enemy[i] - me[i];
        mat[i][1] = -(p2[i] - p1[i]);
        mat[i][2] = -(p3[i] - p1[i]);
        mat[i][3] = p1[i] - me[i];
    }

    // Gaussian elimination with partial pivoting
    let mut singular = false;
    for i in 0..3 {
        // find pivot row
        let mut pivot_row = i;
        let mut max_abs = mat[i][i].abs();
        for r in (i + 1)..3 {
            let a = mat[r][i].abs();
            if a > max_abs {
                max_abs = a;
                pivot_row = r;
            }
        }
        // swap if needed
        if pivot_row != i {
            mat.swap(pivot_row, i);
        }
        // check pivot
        if mat[i][i].abs() < EPS {
            singular = true;
            break;
        }
        // eliminate below
        for r in (i + 1)..3 {
            let factor = mat[r][i] / mat[i][i];
            // subtract factor * row i from row r
            for c in i..4 {
                mat[r][c] -= factor * mat[i][c];
            }
        }
    }

    if singular {
        println!("HIT");
        return;
    }

    // Back substitution to get x, y, z
    let mut sol = [0.0f64; 3];
    for i in (0..3).rev() {
        let mut sum = mat[i][3];
        for j in (i + 1)..3 {
            sum -= mat[i][j] * sol[j];
        }
        // Pivot should not be near zero here (we checked during elimination)
        sol[i] = sum / mat[i][i];
    }

    let x = sol[0];
    let y = sol[1];
    let z = sol[2];

    // Apply the tolerance and bounds checks per specification
    let hit = x < EPS
        || x > 1.0 + EPS
        || y < -EPS
        || z < -EPS
        || (y + z) > 1.0 + EPS;

    if hit {
        println!("HIT");
    } else {
        println!("MISS");
    }
}
