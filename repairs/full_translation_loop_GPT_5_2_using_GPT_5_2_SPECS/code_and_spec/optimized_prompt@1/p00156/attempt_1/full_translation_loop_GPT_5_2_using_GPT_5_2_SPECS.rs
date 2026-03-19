use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens.
    // This matches the C code's use of scanf with " %c" (skips whitespace).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Worklists sized as in the C code (up to 200,000 entries).
    let mut qx = vec![0i32; 200_000];
    let mut qy = vec![0i32; 200_000];

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: usize = w_opt.unwrap().parse().unwrap();
        let h: usize = it.next().unwrap().parse().unwrap();
        if w == 0 && h == 0 {
            break;
        }

        // Grid and cost arrays (fixed max 101x101 in C; spec says <= 100).
        let mut m = vec![vec!['\0'; w]; h];
        let mut c = vec![vec![0i32; w]; h];

        // Find start '&', replace with '.', initialize queue with it and cost=1.
        // Note: C initializes cost to 1 (not 0). We preserve that exactly.
        let mut t: usize = 0;
        let mut r: usize = 0;

        for i in 0..h {
            for j in 0..w {
                let tok = it.next().unwrap();
                let ch = tok.chars().next().unwrap();
                if ch == '&' {
                    m[i][j] = '.';
                    qx[0] = j as i32;
                    qy[0] = i as i32;
                    t = 0;
                    r = 1;
                    c[i][j] = 1;
                } else {
                    m[i][j] = ch;
                }
            }
        }

        let dx = [0i32, 1, 0, -1];
        let dy = [1i32, 0, -1, 0];

        // Relaxation loop as in C: for (r=1; r-t; t++) { ... push to [r++] ... }
        while r != t {
            let cx = qx[t];
            let cy = qy[t];
            t += 1;

            for k in 0..4 {
                let nx = cx + dx[k];
                let ny = cy + dy[k];
                if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                    continue;
                }
                let (nxu, nyu) = (nx as usize, ny as usize);
                let (cxu, cyu) = (cx as usize, cy as usize);

                let cur_cost = c[cyu][cxu];
                let next_cost = if m[cyu][cxu] == m[nyu][nxu] {
                    cur_cost
                } else {
                    cur_cost + 1
                };

                // C condition differs for equal vs different:
                // equal: if (cur < c[next] || c[next]==0) set to cur
                // diff : if (cur+1 < c[next] || c[next]==0) set to cur+1
                // We preserve that exact behavior.
                let should_update = if m[cyu][cxu] == m[nyu][nxu] {
                    c[nyu][nxu] == 0 || cur_cost < c[nyu][nxu]
                } else {
                    c[nyu][nxu] == 0 || next_cost < c[nyu][nxu]
                };

                if should_update {
                    c[nyu][nxu] = next_cost;
                    // Assumes queue capacity is sufficient as per spec/C constraints.
                    qx[r] = nx;
                    qy[r] = ny;
                    r += 1;
                }
            }
        }

        // Compute minimum over boundary cells of (c / 2), integer division.
        // C initializes min = w (not a large sentinel). Preserve.
        let mut minv: i32 = w as i32;

        for i in 0..w {
            let top = c[0][i] / 2;
            if top < minv {
                minv = top;
            }
            let bottom = c[h - 1][i] / 2;
            if bottom < minv {
                minv = bottom;
            }
        }
        for i in 0..h {
            let left = c[i][0] / 2;
            if left < minv {
                minv = left;
            }
            let right = c[i][w - 1] / 2;
            if right < minv {
                minv = right;
            }
        }

        println!("{minv}");
    }
}
