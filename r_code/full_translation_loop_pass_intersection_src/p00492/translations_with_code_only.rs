use std::io::{self, Read};

fn main() {
    // Neighbor offsets (hex grid in offset coordinates), exactly as in C.
    let x_off: [i32; 6] = [1, 2, 1, -1, -2, -1];
    let y_off: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: i32 = it.next().unwrap().parse().unwrap();

    // d[110][210] = {2}; in C sets d[0][0]=2 and the rest to 0.
    // We'll allocate the same fixed size and set only [0][0]=2.
    let mut d = vec![vec![0i32; 210]; 110];
    d[0][0] = 2;

    // Read grid values into d[i][j*2 + i%2] for i=1..=h, j=1..=w.
    for i in 1..=h {
        for j in 1..=w {
            let val: i32 = it.next().unwrap().parse().unwrap();
            let col = (j * 2 + (i % 2)) as usize;
            d[i as usize][col] = val;
        }
    }

    // BFS queue arrays x[10000], y[10000]
    let mut qx = vec![0i32; 10000];
    let mut qy = vec![0i32; 10000];

    // x[0]=y[0]=t=s=0; r starts at 1.
    let mut t: i32 = 0;
    let mut r: i32 = 1;
    let mut s: i32 = 0;

    // for(r=1; r-t; t++) { ... }
    // This is a typical queue loop: while (t < r) { ...; t++; }
    while r - t != 0 {
        let cx = qx[t as usize];
        let cy = qy[t as usize];

        for i in 0..6 {
            let nx = cx + x_off[i];
            let ny = cy + y_off[i];

            // if(nx<0||ny<0||h+1<ny||w*2+4<nx)continue;
            if nx < 0 || ny < 0 || (h + 1) < ny || (w * 2 + 4) < nx {
                continue;
            }

            let nyu = ny as usize;
            let nxu = nx as usize;

            // if(d[ny][nx])continue;
            if d[nyu][nxu] != 0 {
                continue;
            }

            // d[ny][nx]=2; enqueue
            d[nyu][nxu] = 2;
            qx[r as usize] = nx;
            qy[r as usize] = ny;
            r += 1;
        }

        t += 1;
    }

    // Count edges between "1" cells and outside-reachable "2" cells.
    for i in 1..=h {
        for j in 1..=w {
            let base_x = j * 2 + (i % 2);
            // if(d[i][base_x]-1)continue;  => continue unless d == 1
            if d[i as usize][base_x as usize] - 1 != 0 {
                continue;
            }
            for k in 0..6 {
                let ny = i + y_off[k];
                let nx = base_x + x_off[k];
                // Indices are safe due to the padding and bounds used in BFS;
                // this mirrors the C code's reliance on the fixed array size.
                if d[ny as usize][nx as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    print!("{s}\n");
}
