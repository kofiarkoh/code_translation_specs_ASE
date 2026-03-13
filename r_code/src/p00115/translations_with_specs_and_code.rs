// translate_hit_miss.rs
// Translated from the provided C program following the SPEC.
// Behavior: reads 5 points (15 doubles) from stdin, solves a 3x3 linear system
// using Gaussian elimination with partial pivoting and tolerance EPS = 1e-7,
// then prints either "HIT\n" or "MISS\n" according to the SPEC.

use std::io::{self, Read};

const EPS: f64 = 1e-7;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Perform Gaussian elimination with partial pivoting on a 3x4 matrix in place.
/// Returns true on success (non-singular), false if a pivot is effectively zero (< EPS).
fn gauss(matrix: &mut [[f64; 4]; 3]) -> bool {
    for i in 0..3 {
        // Partial pivot: find row with largest absolute value in column i among rows i..2
        let mut hiindex = i;
        let mut hiscore = matrix[i][i].abs();
        for j in (i + 1)..3 {
            let val = matrix[j][i].abs();
            if val > hiscore {
                hiscore = val;
                hiindex = j;
            }
        }
        // Swap rows i and hiindex
        if hiindex != i {
            matrix.swap(i, hiindex);
        }
        // If pivot is too small, singular
        if matrix[i][i].abs() < EPS {
            return false;
        }
        // Normalize pivot row
        let pivot = matrix[i][i];
        for j in 0..4 {
            matrix[i][j] /= pivot;
        }
        // Eliminate column i in other rows
        for j in 0..3 {
            if j == i {
                continue;
            }
            let factor = matrix[j][i];
            for k in 0..4 {
                matrix[j][k] -= factor * matrix[i][k];
            }
        }
    }
    true
}

fn main() {
    // Read all stdin, split by whitespace, parse as f64
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        // Per SPEC preconditions, input should be valid; on read error we just exit.
        return;
    }
    let mut vals: Vec<f64> = Vec::with_capacity(15);
    for tok in buf.split_whitespace().take(15) {
        match tok.parse::<f64>() {
            Ok(v) => vals.push(v),
            Err(_) => {
                // SPEC assumes valid input; on parse failure we exit silently (no defined behavior).
                return;
            }
        }
    }
    if vals.len() < 15 {
        // Insufficient tokens: SPEC does not define behavior; exit.
        return;
    }

    // Construct points in the expected order: me, enemy, p1, p2, p3
    let me = Point { x: vals[0], y: vals[1], z: vals[2] };
    let enemy = Point { x: vals[3], y: vals[4], z: vals[5] };
    let p1 = Point { x: vals[6], y: vals[7], z: vals[8] };
    let p2 = Point { x: vals[9], y: vals[10], z: vals[11] };
    let p3 = Point { x: vals[12], y: vals[13], z: vals[14] };

    let vec1 = enemy.sub(me);
    let vec2 = p2.sub(p1);
    let vec3 = p3.sub(p1);

    // Build the augmented matrix for [vec1 -vec2 -vec3] * [x y z]^T = p1 - me
    let mut matrix: [[f64; 4]; 3] = [[0.0; 4]; 3];
    matrix[0][0] = vec1.x;
    matrix[0][1] = -vec2.x;
    matrix[0][2] = -vec3.x;
    matrix[1][0] = vec1.y;
    matrix[1][1] = -vec2.y;
    matrix[1][2] = -vec3.y;
    matrix[2][0] = vec1.z;
    matrix[2][1] = -vec2.z;
    matrix[2][2] = -vec3.z;

    matrix[0][3] = p1.x - me.x;
    matrix[1][3] = p1.y - me.y;
    matrix[2][3] = p1.z - me.z;

    let gauss_ret = gauss(&mut matrix);

    // After gauss, matrix[i][3] holds the solution for x,y,z when gauss_ret == true.
    // Apply the SPEC decision logic. Note: C code used (y + EPS < 0.0) meaning y < -EPS, etc.
    let hit = (!gauss_ret)
        || (matrix[0][3] < EPS)
        || (matrix[0][3] > 1.0 + EPS)
        || (matrix[1][3] + EPS < 0.0) // y < -EPS
        || (matrix[2][3] + EPS < 0.0) // z < -EPS
        || (matrix[1][3] + matrix[2][3] > 1.0 + EPS);

    if hit {
        println!("HIT");
    } else {
        println!("MISS");
    }
}
