use std::io::{self, Read};

fn main() {
    // Neighbor offsets (hex-grid style as in the C code)
    let x_off: [i32; 6] = [1, 2, 1, -1, -2, -1];
    let y_off: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // Read all input as integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size grid as in C: d[110][210] initialized to 0, except d[0][0]=2.
    // Note: C uses `int d[110][210]={2};` which sets only d[0][0]=2.
    let mut d = vec![vec![0i32; 210]; 110];
    d[0][0] = 2;

    // Read the interior cells into d[i][j*2 + i%2] for i=1..=h, j=1..=w
    for i in 1..=h {
        for j in 1..=w {
            let val: i32 = it.next().unwrap().parse().unwrap();
            let col = j * 2 + (i % 2);
            d[i][col] = val;
        }
    }

    // BFS/queue over empty cells (value 0), marking reachable as 2 starting from (0,0)
    // C uses arrays x[10000], y[10000] and indices r (write), t (read).
    let mut qx: Vec<i32> = Vec::with_capacity(10000);
    let mut qy: Vec<i32> = Vec::with_capacity(10000);
    qx.push(0);
    qy.push(0);
    let mut t: usize = 0;

    // Bounds used by C:
    // nx in [0, w*2+4], ny in [0, h+1]
    let max_x: i32 = (w * 2 + 4) as i32;
    let max_y: i32 = (h + 1) as i32;

    while t < qx.len() {
        let cx = qx[t];
        let cy = qy[t];
        t += 1;

        for k in 0..6 {
            let nx = cx + x_off[k];
            let ny = cy + y_off[k];
            if nx < 0 || ny < 0 || ny > max_y || nx > max_x {
                continue;
            }
            let nyu = ny as usize;
            let nxu = nx as usize;
            if d[nyu][nxu] != 0 {
                continue;
            }
            d[nyu][nxu] = 2;
            qx.push(nx);
            qy.push(ny);
        }
    }

    // Count, for each original cell position (i, j*2+i%2) that equals 1,
    // how many of its 6 neighbors are marked 2.
    let mut s: i32 = 0;
    for i in 1..=h {
        for j in 1..=w {
            let cx = (j * 2 + (i % 2)) as i32;
            let cy = i as i32;

            // C condition: if(d[...] - 1) continue;  => proceed only when d == 1
            if d[i][(j * 2 + (i % 2))] != 1 {
                continue;
            }

            for k in 0..6 {
                let nx = cx + x_off[k];
                let ny = cy + y_off[k];
                // In C, these indices are assumed in-bounds due to padding/limits.
                // We keep the same assumption (precondition/undefined behavior otherwise).
                if d[ny as usize][nx as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    print!("{s}\n");
}
