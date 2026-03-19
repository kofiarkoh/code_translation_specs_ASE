use std::io::{self, Read};

fn main() {
    // Neighbor offsets in the padded coordinate system used by the C code.
    // This encodes hex adjacency with row-parity handled by the x-coordinate layout.
    let x_off: [i32; 6] = [1, 2, 1, -1, -2, -1];
    let y_off: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // Read all input as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = it.next().unwrap().parse().unwrap();
    let h: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size storage as in the C code: d[110][210]
    // Values:
    //   0 = empty (unvisited)
    //   1 = filled (or any non-zero input; treated as blocking for flood fill)
    //   2 = exterior-reachable empty (marked by flood fill)
    let mut d = vec![vec![0i32; 210]; 110];

    // Load input cells into the staggered x-coordinate: j*2 + (i%2)
    for i in 1..=h {
        let parity = i % 2;
        for j in 1..=w {
            let v: i32 = it.next().unwrap().parse().unwrap();
            let x = j * 2 + parity;
            d[i][x] = v;
        }
    }

    // Flood fill from (0,0) over empty cells to mark exterior-reachable empties as 2.
    // Queue arrays sized like the C code (10000), but use Vec for safety.
    let mut qx: Vec<i32> = Vec::with_capacity(10000);
    let mut qy: Vec<i32> = Vec::with_capacity(10000);
    qx.push(0);
    qy.push(0);

    let mut t: usize = 0;
    d[0][0] = 2; // mark start as exterior

    // Bounds used by the C code:
    // ny in [0, h+1], nx in [0, w*2+4]
    let max_ny: i32 = (h + 1) as i32;
    let max_nx: i32 = (w * 2 + 4) as i32;

    while t < qx.len() {
        let cx = qx[t];
        let cy = qy[t];
        t += 1;

        for k in 0..6 {
            let nx = cx + x_off[k];
            let ny = cy + y_off[k];
            if nx < 0 || ny < 0 || ny > max_ny || nx > max_nx {
                continue;
            }
            let (nyu, nxu) = (ny as usize, nx as usize);
            if d[nyu][nxu] != 0 {
                continue; // blocks on filled (non-zero) and already-marked exterior (2)
            }
            d[nyu][nxu] = 2;
            qx.push(nx);
            qy.push(ny);
        }
    }

    // Count perimeter edges: for each filled cell (value exactly 1),
    // count adjacent edges that touch exterior-reachable empty (value 2).
    // Note: per spec, non-1 non-zero values are treated as "not filled" for counting,
    // but still block flood fill (already satisfied above).
    let mut s: i32 = 0;
    for i in 1..=h {
        let parity = i % 2;
        for j in 1..=w {
            let cx = (j * 2 + parity) as i32;
            if d[i][cx as usize] != 1 {
                continue;
            }
            for k in 0..6 {
                let nx = cx + x_off[k];
                let ny = i as i32 + y_off[k];
                // These neighbor indices are within the padded array for valid inputs
                // (matching the C code's assumptions / preconditions).
                if d[ny as usize][nx as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    print!("{s}\n");
}
