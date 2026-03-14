use std::io::{self, BufRead};

const MAX_N: usize = 10;
const MAX_M: usize = 10;
const MAX_G: usize = 9;

static mut SX: i32 = 0; // Using unsafe to maintain global state similar to C
static mut SY: i32 = 0; // Using unsafe to maintain global state similar to C
static mut SCX: [i32; MAX_N] = [0; MAX_N];
static mut SCY: [i32; MAX_N] = [0; MAX_N];

fn b_survival(n: usize) -> bool {
    let mut field = [[false; MAX_M]; MAX_M];
    let mut px = [0; MAX_G];
    let mut py = [0; MAX_G];
    let dx2: [i32; 12] = [-1, 0, 1, 2, 2, 2, -1, 0, 1, -2, -2, -2];
    let dy2: [i32; 12] = [-2, -2, -2, -1, 0, 1, 2, 2, 2, -1, 0, 1];

    let mut ix = 0;
    unsafe {
        px[ix] = SX;
        py[ix] = SY;
        ix += 1;
    }

    for u in 0..n {
        // Clear the field based on scx and scy
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = unsafe { SCX[u] + dx };
                let ny = unsafe { SCY[u] + dy };
                if (0..MAX_M).contains(&nx) && (0..MAX_M).contains(&ny) {
                    field[ny as usize][nx as usize] = false;
                }
            }
        }

        // Mark reachable positions
        for i in 0..ix {
            for j in 0..12 {
                let nx = px[i] + dx2[j];
                let ny = py[i] + dy2[j];
                if (0..MAX_M).contains(&nx) && (0..MAX_M).contains(&ny) {
                    field[ny as usize][nx as usize] = true;
                }
            }
        }

        ix = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = unsafe { SCX[u] + dx };
                let ny = unsafe { SCY[u] + dy };
                if (0..MAX_M).contains(&nx) && (0..MAX_M).contains(&ny) && field[ny as usize][nx as usize] {
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut lineno = 0;

    loop {
        lineno += 1;
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let sx = coords.next().unwrap();
        let sy = coords.next().unwrap();
        if sx == 0 && sy == 0 {
            break;
        }

        lineno += 1;
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

        lineno += 1;
        for i in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut sc_coords = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            unsafe {
                SCX[i] = sc_coords.next().unwrap();
                SCY[i] = sc_coords.next().unwrap();
            }
        }

        unsafe {
            SX = sx;
            SY = sy;
        }

        println!("{}", if b_survival(n) { "OK" } else { "NA" });
    }
}
