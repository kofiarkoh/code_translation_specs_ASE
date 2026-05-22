use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = it.next().unwrap().parse().unwrap();
    let h: usize = it.next().unwrap().parse().unwrap();

    // We model an "outside padding" border around the grid to allow BFS from outside.
    // Grid indices:
    //   original cells: x in 1..=w, y in 1..=h
    //   padded area:    x in 0..=w+1, y in 0..=h+1
    let pw = w + 2;
    let ph = h + 2;

    // occ[y][x] == true means occupied (value 1). Everything else is empty.
    let mut occ = vec![vec![false; pw]; ph];
    for y in 1..=h {
        for x in 1..=w {
            let v: i32 = it.next().unwrap().parse().unwrap();
            occ[y][x] = v == 1;
        }
    }

    // Mark exterior-reachable empty cells via BFS starting from the padded outside (0,0).
    let mut ext = vec![vec![false; pw]; ph];
    let mut q = VecDeque::new();

    // Start from (0,0) which is guaranteed to be outside the original grid and empty.
    ext[0][0] = true;
    q.push_back((0usize, 0usize));

    // Hex grid neighbor offsets depend on row parity.
    // We use "odd-r" horizontal layout:
    //   even row: neighbors include (x-1,y-1),(x,y-1),(x-1,y),(x+1,y),(x-1,y+1),(x,y+1)
    //   odd row:  neighbors include (x,y-1),(x+1,y-1),(x-1,y),(x+1,y),(x,y+1),(x+1,y+1)
    const DX_EVEN: [isize; 6] = [-1, 0, -1, 1, -1, 0];
    const DY_EVEN: [isize; 6] = [-1, -1, 0, 0, 1, 1];
    const DX_ODD: [isize; 6] = [0, 1, -1, 1, 0, 1];
    const DY_ODD: [isize; 6] = [-1, -1, 0, 0, 1, 1];

    while let Some((x, y)) = q.pop_front() {
        let (dxs, dys) = if (y & 1) == 0 {
            (&DX_EVEN, &DY_EVEN)
        } else {
            (&DX_ODD, &DY_ODD)
        };

        for i in 0..6 {
            let nx = x as isize + dxs[i];
            let ny = y as isize + dys[i];
            if nx < 0 || ny < 0 || nx >= pw as isize || ny >= ph as isize {
                continue;
            }
            let nxu = nx as usize;
            let nyu = ny as usize;

            // Only traverse empty cells; occupied cells block reachability.
            if !ext[nyu][nxu] && !occ[nyu][nxu] {
                ext[nyu][nxu] = true;
                q.push_back((nxu, nyu));
            }
        }
    }

    // Count boundary edges: for each occupied cell, count neighbors that are exterior empty.
    let mut boundary: u64 = 0;
    for y in 1..=h {
        let (dxs, dys) = if (y & 1) == 0 {
            (&DX_EVEN, &DY_EVEN)
        } else {
            (&DX_ODD, &DY_ODD)
        };
        for x in 1..=w {
            if !occ[y][x] {
                continue;
            }
            for i in 0..6 {
                let nx = x as isize + dxs[i];
                let ny = y as isize + dys[i];
                if nx < 0 || ny < 0 || nx >= pw as isize || ny >= ph as isize {
                    continue;
                }
                let nxu = nx as usize;
                let nyu = ny as usize;

                // Neighbor contributes if it is empty and exterior-reachable.
                if !occ[nyu][nxu] && ext[nyu][nxu] {
                    boundary += 1;
                }
            }
        }
    }

    println!("{}", boundary);
}
