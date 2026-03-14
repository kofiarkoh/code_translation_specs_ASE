use std::io::{self, BufRead};

const EPS: f64 = 1e-7;

#[derive(Debug)]
struct Zahyo {
    x: f64,
    y: f64,
    z: f64,
}

fn read_zahyo() -> Zahyo {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let coords: Vec<f64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Zahyo { x: coords[0], y: coords[1], z: coords[2] }
}

fn sub_zahyo(dest: Zahyo, src: Zahyo) -> Zahyo {
    Zahyo {
        x: dest.x - src.x,
        y: dest.y - src.y,
        z: dest.z - src.z,
    }
}

fn zettaiti(a: f64) -> f64 {
    if a < 0.0 { -a } else { a }
}

const WIDTH: usize = 4;
const HEIGHT: usize = 3;

fn gausu(matrix: &mut [[f64; WIDTH]; HEIGHT]) -> bool {
    for i in 0..HEIGHT {
        let mut hiscore = zettaiti(matrix[i][i]);
        let mut hiindex = i;

        for j in (i + 1)..HEIGHT {
            if zettaiti(matrix[j][i]) > hiscore {
                hiscore = zettaiti(matrix[j][i]);
                hiindex = j;
            }
        }

        // Swap rows
        matrix.swap(i, hiindex);

        if zettaiti(matrix[i][i]) < EPS {
            return false;
        }

        let atai = matrix[i][i];
        for j in 0..WIDTH {
            matrix[i][j] /= atai;
        }

        for j in 0..HEIGHT {
            if j == i {
                continue;
            }
            let keisu = matrix[j][i];
            for k in 0..WIDTH {
                matrix[j][k] -= keisu * matrix[i][k];
            }
        }
    }
    true
}

fn main() {
    let me = read_zahyo();
    let enemy = read_zahyo();
    let p1 = read_zahyo();
    let p2 = read_zahyo();
    let p3 = read_zahyo();

    let vec1 = sub_zahyo(enemy, me);
    let vec2 = sub_zahyo(p2, p1);
    let vec3 = sub_zahyo(p3, p1);

    let mut matrix: [[f64; WIDTH]; HEIGHT] = [
        [vec1.x, -vec2.x, -vec3.x, p1.x - me.x],
        [vec1.y, -vec2.y, -vec3.y, p1.y - me.y],
        [vec1.z, -vec2.z, -vec3.z, p1.z - me.z],
    ];

    let gausu_ret = gausu(&mut matrix);

    let result = if !gausu_ret ||
        matrix[0][3] < EPS || matrix[0][3] > 1.0 + EPS ||
        matrix[1][3] + EPS < 0.0 || matrix[2][3] + EPS < 0.0 ||
        matrix[1][3] + matrix[2][3] > 1.0 + EPS {
        "HIT"
    } else {
        "MISS"
    };

    println!("{}", result);
}
