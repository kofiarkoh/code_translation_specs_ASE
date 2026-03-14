use std::io::{self, BufRead};

const MAX_N: usize = 10;
const MAX_M: usize = 10;
const MAX_G: usize = 9;

fn b_survival(sx: usize, sy: usize, scx: &[usize], scy: &[usize], n: usize) -> bool {
    let mut field = [[true; MAX_M]; MAX_M];
    let mut px = [0; MAX_G];
    let mut py = [0; MAX_G];
    let dx2: [isize; 12] = [-1, 0, 1, 2, 2, 2, -1, 0, 1, -2, -2, -2];
    let dy2: [isize; 12] = [-2, -2, -2, -1, 0, 1, 2, 2, 2, -1, 0, 1];

    let mut ix = 0;
    px[ix] = sx;
    py[ix] = sy;
    ix += 1;

    for u in 0..n {
        // Mark obstacles in the field
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx[u] as isize + dx;
                let ny = scy[u] as isize + dy;
                if (0..MAX_M as isize).contains(&nx) && (0..MAX_M as isize).contains(&ny) {
                    field[ny as usize][nx as usize] = false;
                }
            }
        }

        // Update possible positions
        for i in 0..ix {
            for j in 0..12 {
                let nx = px[i] as isize + dx2[j];
                let ny = py[i] as isize + dy2[j];
                if (0..MAX_M as isize).contains(&nx) && (0..MAX_M as isize).contains(&ny) {
                    field[ny as usize][nx as usize] = true;
                }
            }
        }

        ix = 0; // Reset index for next round
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx[u] as isize + dx;
                let ny = scy[u] as isize + dy;
                if (0..MAX_M as isize).contains(&nx) && (0..MAX_M as isize).contains(&ny) && field[ny as usize][nx as usize] {
                    px[ix] = nx as usize;
                    py[ix] = ny as usize;
                    ix += 1;
                }
            }
        }

        if ix == 0 {
            return false; // No possible moves left
        }
    }

    true // Survival is possible
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    loop {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (sx, sy) = (coords[0], coords[1]);

        if sx == 0 && sy == 0 {
            break; // Terminate on (0, 0)
        }

        let line = lines.next().unwrap().unwrap();
        let n: usize = line.trim().parse().unwrap();

        let mut scx = vec![0; n];
        let mut scy = vec![0; n];

        for i in 0..n {
            let line = lines.next().unwrap().unwrap();
            let coords: Vec<usize> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            scx[i] = coords[0];
            scy[i] = coords[1];
        }

        println!("{}", if b_survival(sx, sy, &scx, &scy, n) { "OK" } else { "NA" });
    }
}
