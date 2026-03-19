use std::io::{self, Read};

fn main() {
    // Neighbor offsets (same as C)
    const X: [i32; 6] = [1, 2, 1, -1, -2, -1];
    const Y: [i32; 6] = [1, 0, -1, -1, 0, 1];

    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let w: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size grid like the C code.
    // 0 = empty, 1 = filled (or other non-zero), 2 = exterior-reachable empty
    let mut d = vec![vec![0i32; 210]; 110];

    for i in 1..=h {
        let parity = i & 1;
        for j in 1..=w {
            let v: i32 = it.next().unwrap().parse().unwrap();
            let x = j * 2 + parity;
            d[i][x] = v;
        }
    }

    // Flood fill from (0,0) over empty cells.
    // To avoid timeout from repeatedly re-enqueuing (0,0), mark it as exterior first.
    // This does not change the intended perimeter counting (only cells with value 2 matter),
    // and matches the conceptual "outside is exterior" definition in the spec.
    d[0][0] = 2;

    let max_ny: i32 = (h + 1) as i32;
    let max_nx: i32 = (w * 2 + 4) as i32;

    // Use a simple ring-buffer queue (faster than VecDeque for this fixed workload).
    let mut qx: Vec<i32> = Vec::with_capacity(25000);
    let mut qy: Vec<i32> = Vec::with_capacity(25000);
    qx.push(0);
    qy.push(0);
    let mut head: usize = 0;

    while head < qx.len() {
        let cx = qx[head];
        let cy = qy[head];
        head += 1;

        for k in 0..6 {
            let nx = cx + X[k];
            let ny = cy + Y[k];
            if nx < 0 || ny < 0 || ny > max_ny || nx > max_nx {
                continue;
            }
            let (nyu, nxu) = (ny as usize, nx as usize);
            if d[nyu][nxu] != 0 {
                continue;
            }
            d[nyu][nxu] = 2;
            qx.push(nx);
            qy.push(ny);
        }
    }

    // Count perimeter edges adjacent to exterior-reachable empty.
    let mut ans: i32 = 0;
    for i in 1..=h {
        let parity = (i & 1) as i32;
        for j in 1..=w {
            let cx = (j as i32) * 2 + parity;
            if d[i][cx as usize] != 1 {
                continue;
            }
            for k in 0..6 {
                let nx = cx + X[k];
                let ny = i as i32 + Y[k];
                if d[ny as usize][nx as usize] == 2 {
                    ans += 1;
                }
            }
        }
    }

    print!("{ans}\n");
}
