// Translated from the provided C code. Semantics (including floating-point
// comparisons and pivoting behavior) are preserved as closely as possible.

use std::io::{self, Read};

const EPS: f64 = 1e-7;
const WIDTH: usize = 4;
const HEIGHT: usize = 3;

#[derive(Clone, Copy)]
struct Zahyo {
    x: f64,
    y: f64,
    z: f64,
}

impl Zahyo {
    fn sub(self, other: Zahyo) -> Zahyo {
        Zahyo {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn zettaiti(a: f64) -> f64 {
    // Match C implementation: return a<0 ? -a : a
    // This preserves the behavior for negative zero (returns -0.0 as in C).
    if a < 0.0 { -a } else { a }
}

fn gausu(matrix: &mut [[f64; WIDTH]; HEIGHT]) -> i32 {
    for i in 0..HEIGHT {
        // pipotto-sentaku (partial pivot selection)
        let mut hiscore = zettaiti(matrix[i][i]);
        let mut hiindex = i;
        for j in (i + 1)..HEIGHT {
            if zettaiti(matrix[j][i]) > hiscore {
                hiscore = zettaiti(matrix[j][i]);
                hiindex = j;
            }
        }
        // koukan (swap rows i and hiindex)
        for j in 0..WIDTH {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[hiindex][j];
            matrix[hiindex][j] = tmp;
        }
        if zettaiti(matrix[i][i]) < EPS {
            return 0;
        }
        // syoukyo (elimination)
        let atai = matrix[i][i];
        for j in 0..WIDTH {
            matrix[i][j] /= atai;
        }
        for j in 0..HEIGHT {
            let keisu = matrix[j][i];
            if j == i {
                continue;
            }
            for k in 0..WIDTH {
                matrix[j][k] -= keisu * matrix[i][k];
            }
        }
    }
    1
}

fn read_zahyo_from_iter<'a, I>(iter: &mut I) -> Zahyo
where
    I: Iterator<Item = &'a str>,
{
    // In the C code, scanf("%lf%lf%lf",...) is used. Here we mirror that by
    // parsing three whitespace-separated tokens as f64. If input is malformed
    // or incomplete, this will panic (similar to the original program's
    // reliance on correct input).
    let x = iter
        .next()
        .expect("insufficient input")
        .parse::<f64>()
        .expect("bad float");
    let y = iter
        .next()
        .expect("insufficient input")
        .parse::<f64>()
        .expect("bad float");
    let z = iter
        .next()
        .expect("insufficient input")
        .parse::<f64>()
        .expect("bad float");
    Zahyo { x, y, z }
}

fn main() {
    // Read all input tokens (whitespace-separated), like scanf would.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");
    let mut iter = input.split_whitespace();

    let me = read_zahyo_from_iter(&mut iter);
    let enemy = read_zahyo_from_iter(&mut iter);
    let p1 = read_zahyo_from_iter(&mut iter);
    let p2 = read_zahyo_from_iter(&mut iter);
    let p3 = read_zahyo_from_iter(&mut iter);

    let vec1 = enemy.sub(me);
    let vec2 = p2.sub(p1);
    let vec3 = p3.sub(p1);

    // matrix is 3x4
    let mut matrix = [[0.0f64; WIDTH]; HEIGHT];

    // me + x*vec1 == p1 + y*vec2 + z*vec3
    // [vec1 -vec2 -vec3] X = p1 - me
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

    let gausu_ret = gausu(&mut matrix);

    // Reproduce the exact logical checks from the C code.
    let miss_condition = (! (gausu_ret != 0)) // gausu_ret == 0 => no solution
        || (matrix[0][3] < EPS || matrix[0][3] > 1.0 + EPS) // x not in [0,1]
        || (matrix[1][3] + EPS < 0.0
            || matrix[2][3] + EPS < 0.0
            || matrix[1][3] + matrix[2][3] > 1.0 + EPS); // y,z not in triangle

    // The C code prints "HIT" when any of those failure conditions is true.
    // It used puts(... ? "HIT":"MISS"), which prints with a trailing newline.
    if miss_condition {
        println!("HIT");
    } else {
        println!("MISS");
    }
}
