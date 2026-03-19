use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let states: usize = it.next().unwrap().parse().unwrap(); // number of evaluations (initial + batches)
    let n: usize = it.next().unwrap().parse().unwrap(); // grid dimension

    // Store grid as bytes 0/1
    let mut grid: Vec<Vec<u8>> = vec![vec![0u8; n]; n];
    for r in 0..n {
        let row = it.next().unwrap().as_bytes();
        // Preconditions guarantee correct length and only '0'/'1'
        for c in 0..n {
            grid[r][c] = row[c] - b'0';
        }
    }

    // Maintain counts of mismatches for vertical and horizontal symmetry.
    // A state is valid iff both mismatch counts are zero.
    //
    // Vertical symmetry: grid[r][c] == grid[n-1-r][c]
    // Horizontal symmetry: grid[r][c] == grid[r][n-1-c]
    //
    // We count each symmetric pair once:
    // - vertical pairs: r in [0, n/2)
    // - horizontal pairs: c in [0, n/2)
    let mut v_mis: i32 = 0;
    for r in 0..(n / 2) {
        let rr = n - 1 - r;
        for c in 0..n {
            if grid[r][c] != grid[rr][c] {
                v_mis += 1;
            }
        }
    }

    let mut h_mis: i32 = 0;
    for r in 0..n {
        for c in 0..(n / 2) {
            let cc = n - 1 - c;
            if grid[r][c] != grid[r][cc] {
                h_mis += 1;
            }
        }
    }

    let mut valid_count: i32 = 0;
    if v_mis == 0 && h_mis == 0 {
        valid_count += 1;
    }

    // Helper closures to update mismatch counts for affected pairs when toggling a cell.
    // We adjust counts by removing the old mismatch status and adding the new one.
    let mut apply_toggle = |r: usize, c: usize,
                            grid: &mut Vec<Vec<u8>>,
                            v_mis: &mut i32,
                            h_mis: &mut i32| {
        // Vertical pair update (if not on the middle row when n is odd)
        let vr = n - 1 - r;
        if vr != r {
            let a = grid[r][c];
            let b = grid[vr][c];
            let old_mis = (a != b) as i32;
            let new_a = a ^ 1;
            let new_mis = (new_a != b) as i32;
            *v_mis += new_mis - old_mis;
        }

        // Horizontal pair update (if not on the middle column when n is odd)
        let hc = n - 1 - c;
        if hc != c {
            let a = grid[r][c];
            let b = grid[r][hc];
            let old_mis = (a != b) as i32;
            let new_a = a ^ 1;
            let new_mis = (new_a != b) as i32;
            *h_mis += new_mis - old_mis;
        }

        // Apply the toggle
        grid[r][c] ^= 1;
    };

    // Process update batches: total batches = states - 1
    for _ in 1..states {
        let k: usize = it.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let r1: usize = it.next().unwrap().parse().unwrap();
            let c1: usize = it.next().unwrap().parse().unwrap();
            // Convert 1-based to 0-based
            let r = r1 - 1;
            let c = c1 - 1;
            apply_toggle(r, c, &mut grid, &mut v_mis, &mut h_mis);
        }
        if v_mis == 0 && h_mis == 0 {
            valid_count += 1;
        }
    }

    print!("{}", valid_count);
}
