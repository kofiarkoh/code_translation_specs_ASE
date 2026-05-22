use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens (matches scanf("%d"/"%s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Fixed movement and reflection tables per specification.
    let mv: [[i32; 2]; 4] = [[50, 50], [-50, 50], [-50, -50], [50, -50]];
    let tr: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n < 0 {
            break;
        }

        // mir[a][b][c][d] with indices 0..8 inclusive.
        let mut mir = [[[[false; 9]; 9]; 9]; 9];

        for _ in 0..n {
            let s = it.next().unwrap();
            let x: usize = it.next().unwrap().parse().unwrap();
            let y: usize = it.next().unwrap().parse().unwrap();
            if s.as_bytes()[0] == b'x' {
                mir[x][y][x + 1][y] = true;
            } else {
                mir[x][y][x][y + 1] = true;
            }
        }

        // Initial state.
        let mut x: i32 = 50;
        let mut y: i32 = 0;
        let mut d: usize = 0;
        let mut wall: bool = true;

        let mut v: usize = 1; // alternation flag, used for this iteration's mirror test
        let mut i: i32 = 0;

        loop {
            // Termination checks at start of iteration, enabled only once i >= 2.
            if i >= 2 {
                if x == 100 && y == 50 && d == 0 {
                    x = 75;
                    y = 25;
                    break;
                }
                if wall && (x == 0 || y == 0 || x == 800 || y == 800) {
                    break;
                }
            }

            // Move.
            x += mv[d][0];
            y += mv[d][1];
            wall = true;

            // Mirror-crossing test (exactly one per step), chosen by current v.
            if v == 1 {
                let a = (x / 100) as usize;
                let b = ((y - 50) / 100) as usize;
                let c = (x / 100) as usize;
                let dd = ((y + 50) / 100) as usize;
                if mir[a][b][c][dd] {
                    d = tr[v][d];
                    wall = false;
                }
            } else {
                let a = ((x - 50) / 100) as usize;
                let b = (y / 100) as usize;
                let c = ((x + 50) / 100) as usize;
                let dd = (y / 100) as usize;
                if mir[a][b][c][dd] {
                    d = tr[v][d];
                    wall = false;
                }
            }

            // End of iteration: increment i and flip v.
            i += 1;
            v ^= 1;
        }

        out.push_str(&format!("{x} {y}\n"));
    }

    print!("{out}");
}
