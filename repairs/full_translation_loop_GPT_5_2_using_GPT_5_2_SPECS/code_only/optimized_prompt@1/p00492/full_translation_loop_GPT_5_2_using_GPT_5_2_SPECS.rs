use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: i32 = it.next().unwrap().parse().unwrap();

    // Neighbor offsets (hex grid style)
    let x_off: [i32; 6] = [1, 2, 1, -1, -2, -1];
    let y_off: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // d[110][210] = {2}; in C means d[0][0]=2, rest 0.
    // We'll mirror that initialization.
    let mut d = vec![vec![0i32; 210]; 110];
    d[0][0] = 2;

    // Read grid values into d[i][j*2 + i%2], with i,j starting at 1.
    for i in 1..=h {
        for j in 1..=w {
            let val: i32 = it.next().unwrap().parse().unwrap();
            let col = j * 2 + (i % 2);
            d[i as usize][col as usize] = val;
        }
    }

    // BFS-like flood fill from (0,0) over zeros, marking visited as 2.
    // Arrays sized as in C.
    let mut x = vec![0i32; 10000];
    let mut y = vec![0i32; 10000];

    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut r: i32 = 1;

    x[0] = 0;
    y[0] = 0;

    // for(r=1; r-t; t++) { ... }
    while r - t != 0 {
        for i in 0..6 {
            let nx = x[t as usize] + x_off[i];
            let ny = y[t as usize] + y_off[i];

            // Bounds exactly as in C:
            // if(nx<0||ny<0||h+1<ny||w*2+4<nx)continue;
            if nx < 0 || ny < 0 || (h + 1) < ny || (w * 2 + 4) < nx {
                continue;
            }

            let nyu = ny as usize;
            let nxu = nx as usize;

            if d[nyu][nxu] != 0 {
                continue;
            }
            d[nyu][nxu] = 2;
            x[r as usize] = nx;
            y[r as usize] = ny;
            r += 1;
        }
        t += 1;
    }

    // Count edges from building cells (value==1) to outside-reachable cells (value==2).
    for i in 1..=h {
        for j in 1..=w {
            let base_col = j * 2 + (i % 2);
            // if(d[...] - 1) continue;  => only proceed when d == 1
            if d[i as usize][base_col as usize] - 1 != 0 {
                continue;
            }
            for k in 0..6 {
                let ny = i + y_off[k];
                let nx = base_col + x_off[k];
                if d[ny as usize][nx as usize] == 2 {
                    s += 1;
                }
            }
        }
    }

    print!("{s}\n");
}
