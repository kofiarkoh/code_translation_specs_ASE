use std::io::{self, Read};

fn main() {
    // Read entire stdin as tokens (matches scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // mir[a][b][c][d] where each index is 0..=8
    // C uses char array and memset to 0; we use bool.
    let mut mir = [[[[false; 9]; 9]; 9]; 9];

    // Movement vectors for diagonal directions.
    let mv: [[i32; 2]; 4] = [[50, 50], [-50, 50], [-50, -50], [50, -50]];
    // Direction transform on reflection; indexed by v (0/1) and d (0..3).
    let tr: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n < 0 {
            break;
        }

        // Reset mirrors for each test case.
        mir = [[[[false; 9]; 9]; 9]; 9];

        for _ in 0..n {
            let s = it.next().unwrap();
            let x: usize = it.next().unwrap().parse().unwrap();
            let y: usize = it.next().unwrap().parse().unwrap();
            if s.as_bytes()[0] == b'x' {
                // Segment between (x,y) and (x+1,y)
                mir[x][y][x + 1][y] = true;
            } else {
                // Segment between (x,y) and (x,y+1)
                mir[x][y][x][y + 1] = true;
            }
        }

        let mut x: i32 = 50;
        let mut y: i32 = 0;
        let mut d: usize = 0;
        let mut wall: i32 = 1;

        let mut v: usize = 1;
        let mut i: i32 = 0;
        loop {
            if i >= 2 {
                // Cycle-detection escape condition.
                if x == 100 && y == 50 && d == 0 {
                    x = 75;
                    y = 25;
                    break;
                }
                // Boundary termination condition (only if last step did not reflect).
                if wall != 0 && (x == 0 || y == 0 || x == 800 || y == 800) {
                    break;
                }
            }

            // Advance one step.
            x += mv[d][0];
            y += mv[d][1];
            wall = 1;

            if v == 1 {
                // Check "vertical" oriented segment (as per C indexing).
                let a = (x / 100) as usize;
                let b = ((y - 50) / 100) as usize;
                let c = (x / 100) as usize;
                let dd = ((y + 50) / 100) as usize;
                if mir[a][b][c][dd] {
                    d = tr[v][d];
                    wall = 0;
                }
            } else {
                // Check "horizontal" oriented segment (as per C indexing).
                let a = ((x - 50) / 100) as usize;
                let b = (y / 100) as usize;
                let c = ((x + 50) / 100) as usize;
                let dd = (y / 100) as usize;
                if mir[a][b][c][dd] {
                    d = tr[v][d];
                    wall = 0;
                }
            }

            i += 1;
            v ^= 1; // alternate orientation check each step
        }

        println!("{x} {y}");
    }
}
