// translation.rs
// Translated from the provided C code. Preserves behavior closely.

use std::io::{self, Read};

const MAX_N: usize = 10;
const MAX_M: i32 = 10;
const MAX_G: usize = 9;

fn b_survival(sx: i32, sy: i32, n: usize, scx: &[i32], scy: &[i32]) -> bool {
    // In the original C code `field` was an automatic array (uninitialized),
    // but every cell that is read is written in the same loop before being read.
    // Initializing to false here does not change the observable behavior for
    // valid inputs, and avoids undefined behavior in Rust.
    let mut field = [[false; MAX_M as usize]; MAX_M as usize];

    let mut px: [i32; MAX_G] = [0; MAX_G];
    let mut py: [i32; MAX_G] = [0; MAX_G];

    // dx2/dy2 arrays as in C, order preserved
    let dx2: [i32; 12] = [-1, 0, 1, 2, 2, 2, -1, 0, 1, -2, -2, -2];
    let dy2: [i32; 12] = [-2, -2, -2, -1, 0, 1, 2, 2, 2, -1, 0, 1];

    let mut ix: usize = 0;
    px[ix] = sx;
    py[ix] = sy;
    ix += 1;

    for u in 0..n {
        // set 3x3 around obstacle to false (if in bounds)
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx[u] + dx;
                let ny = scy[u] + dy;
                if (0 <= nx && nx < MAX_M) && (0 <= ny && ny < MAX_M) {
                    field[ny as usize][nx as usize] = false;
                }
            }
        }

        // mark reachable knight-like moves from current px/py as true
        for i in 0..ix {
            for j in 0..12 {
                let nx = px[i] + dx2[j];
                let ny = py[i] + dy2[j];
                if (0 <= nx && nx < MAX_M) && (0 <= ny && ny < MAX_M) {
                    field[ny as usize][nx as usize] = true;
                }
            }
        }

        // collect positions in the 3x3 around the obstacle that are true
        ix = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx[u] + dx;
                let ny = scy[u] + dy;
                if (0 <= nx && nx < MAX_M)
                    && (0 <= ny && ny < MAX_M)
                    && field[ny as usize][nx as usize]
                {
                    // px/py capacity is MAX_G (9). The C code assumes this is sufficient.
                    px[ix] = nx;
                    py[ix] = ny;
                    ix += 1;
                }
            }
        }

        if ix == 0 {
            return false;
        }
    }

    true
}

fn main() {
    // Read all input as whitespace-separated integers, like repeated scanf calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace().map(|s| s.parse::<i32>().expect("parse int"));

    loop {
        // Read sx, sy
        let sx = match it.next() {
            Some(v) => v,
            None => break, // EOF
        };
        let sy = match it.next() {
            Some(v) => v,
            None => break,
        };

        if sx == 0 && sy == 0 {
            break;
        }

        let n_i32 = match it.next() {
            Some(v) => v,
            None => break,
        };
        let n = n_i32 as usize;

        // read n pairs into arrays
        let mut scx = [0i32; MAX_N];
        let mut scy = [0i32; MAX_N];
        for i in 0..n {
            scx[i] = it.next().expect("missing scx") as i32;
            scy[i] = it.next().expect("missing scy") as i32;
        }

        let ok = b_survival(sx, sy, n, &scx, &scy);
        if ok {
            println!("OK");
        } else {
            println!("NA");
        }
    }
}
