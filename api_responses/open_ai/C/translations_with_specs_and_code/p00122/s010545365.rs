// survival.rs
//
// Translation of the provided C program into Rust, following the given specification.
// Behavior-preserving: reads test cases from stdin until the sentinel "0 0" start coordinate,
// for each case reads n and n scout coordinates, and prints "OK" or "NA" per case.
//
// Notes on translation choices:
// - The original C code used an uninitialized local boolean array and only touched the 3x3
//   neighborhood cells; here we initialize the whole field to false each step. This does
//   not change observable behavior under the spec (only neighborhood cells are inspected).
// - Uses safe, idiomatic Rust (no unsafe).
// - On malformed input the program will panic (matching the C program's unspecified behavior
//   in that situation per the specification).

use std::io::{self, Read};

const MAX_N: usize = 10;
const MAX_M: i32 = 10; // grid size (0..9)
const MAX_G: usize = 9; // max candidate positions (3x3 = 9)

const DX2: [i32; 12] = [-1, 0, 1, 2, 2, 2, -1, 0, 1, -2, -2, -2];
const DY2: [i32; 12] = [-2, -2, -2, -1, 0, 1, 2, 2, 2, -1, 0, 1];

fn in_bounds(x: i32, y: i32) -> bool {
    (0..MAX_M).contains(&x) && (0..MAX_M).contains(&y)
}

fn b_survival(sx: i32, sy: i32, sc: &[(i32, i32)]) -> bool {
    // field[y][x] where 0 <= x,y < MAX_M
    let mut field = [[false; MAX_M as usize]; MAX_M as usize];

    // candidate positions (px, py). Use Vec for simplicity; at most 9 elements per spec.
    let mut candidates: Vec<(i32, i32)> = Vec::with_capacity(MAX_G);
    candidates.push((sx, sy));

    let n = sc.len();
    for u in 0..n {
        let (scx, scy) = sc[u];

        // Clear the 3x3 neighborhood entries to false (explicitly).
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx + dx;
                let ny = scy + dy;
                if in_bounds(nx, ny) {
                    field[ny as usize][nx as usize] = false;
                }
            }
        }

        // Mark reachable cells from current candidates using the 12 move offsets.
        for &(px, py) in &candidates {
            for j in 0..DX2.len() {
                let nx = px + DX2[j];
                let ny = py + DY2[j];
                if in_bounds(nx, ny) {
                    field[ny as usize][nx as usize] = true;
                }
            }
        }

        // Build next candidate list: the intersection of the scout's 3x3 neighborhood
        // with the reachable cells (field == true).
        candidates.clear();
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = scx + dx;
                let ny = scy + dy;
                if in_bounds(nx, ny) && field[ny as usize][nx as usize] {
                    candidates.push((nx, ny));
                }
            }
        }

        if candidates.is_empty() {
            return false;
        }
    }

    true
}

fn main() {
    // Read all stdin and parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut tokens = input.split_whitespace();
    // Helper to parse next integer or panic (mirrors undefined behavior on malformed input).
    let mut next_i32 = || -> i32 {
        tokens
            .next()
            .expect("unexpected end of input")
            .parse::<i32>()
            .expect("failed to parse integer")
    };

    loop {
        // Read start coordinates
        let sx = next_i32();
        let sy = next_i32();
        if sx == 0 && sy == 0 {
            break;
        }

        let n = next_i32();
        // Per specification, n is expected to be within 0..=10. We trust input conforms.
        let n_usize = n as usize;

        let mut scouts: Vec<(i32, i32)> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let scx = next_i32();
            let scy = next_i32();
            scouts.push((scx, scy));
        }

        let ok = b_survival(sx, sy, &scouts);
        if ok {
            println!("OK");
        } else {
            println!("NA");
        }
    }
}
