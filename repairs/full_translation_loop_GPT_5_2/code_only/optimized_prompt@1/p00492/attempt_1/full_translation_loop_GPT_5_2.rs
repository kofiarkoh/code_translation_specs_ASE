use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers (matches scanf behavior for ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // no input
    };
    let h: i32 = it.next().unwrap().parse().unwrap();

    // Neighbor offsets (hex grid in "doubled x" coordinates with row parity).
    let x_off: [i32; 6] = [1, 2, 1, -1, -2, -1];
    let y_off: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // C: int d[110][210] = {2}; => d[0][0] = 2, everything else 0.
    // We'll allocate the same fixed sizes to preserve indexing behavior.
    let mut d = vec![vec![0i32; 210]; 110];
    d[0][0] = 2;

    // Read grid into d[i][j*2 + i%2] for i=1..=h, j=1..=w.
    for i in 1..=h {
        let parity = i & 1;
        for j in 1..=w {
            let val: i32 = it.next().unwrap().parse().unwrap();
            let col = j * 2 + parity;
            d[i as usize][col as usize] = val;
        }
    }

    // BFS flood fill from (0,0) over cells where d==0, marking them as 2.
    // C uses x[10000], y[10000] and indices r,t.
    let mut xq = vec![0i32; 10000];
    let mut yq = vec![0i32; 10000];
    let mut s: i32 = 0;

    let mut t: i32 = 0;
    let mut r: i32 = 0;
    xq[0] = 0;
    yq[0] = 0;

    // C loop: for(r=1; r-t; t++) { ... push to x[r], y[r], r++ ... }
    r = 1;
    while r - t != 0 {
        for i in 0..6 {
            let nx = xq[t as usize] + x_off[i];
            let ny = yq[t as usize] + y_off[i];

            // Bounds check exactly as in C.
            if nx < 0 || ny < 0 || h + 1 < ny || w * 2 + 4 < nx {
                continue;
            }
            let (uy, ux) = (ny as usize, nx as usize);
            if d[uy][ux] != 0 {
                continue;
            }
            d[uy][ux] = 2;
            xq[r as usize] = nx;
            yq[r as usize] = ny;
            r += 1;
        }
        t += 1;
    }

    // Count exposed sides: for each original cell with value 1, count adjacent d==2.
    for i in 1..=h {
        let parity = i & 1;
        for j in 1..=w {
            let cx = j * 2 + parity;
            // C: if(d[...] - 1) continue;  => only proceed when d == 1.
            if d[i as usize][cx as usize] != 1 {
                continue;
            }
            for k in 0..6 {
                let ny = i + y_off[k];
                let nx = cx + x_off[k];
                if d[ny as usize][nx as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    print!("{s}\n");
}
