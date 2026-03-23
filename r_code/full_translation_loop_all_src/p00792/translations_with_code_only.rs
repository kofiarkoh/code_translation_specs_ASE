use std::io::{self, Read};

fn main() {
    // Read entire stdin and tokenize like scanf("%d"/"%s") would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // mir[9][9][9][9] as in C (indices 0..=8 used).
    let mut mir = [[[[0u8; 9]; 9]; 9]; 9];

    // Movement and turn tables.
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

        // memset(mir, 0, sizeof(mir));
        mir = [[[[0u8; 9]; 9]; 9]; 9];

        for _ in 0..n {
            let s = it.next().unwrap();
            let x: usize = it.next().unwrap().parse().unwrap();
            let y: usize = it.next().unwrap().parse().unwrap();
            if s.as_bytes()[0] == b'x' {
                mir[x][y][x + 1][y] = 1;
            } else {
                mir[x][y][x][y + 1] = 1;
            }
        }

        let mut x: i32 = 50;
        let mut y: i32 = 0;
        let mut d: usize = 0;
        let mut wall: i32 = 1;

        let mut v: i32 = 1;
        let mut i: i32 = 0;
        loop {
            if i >= 2 {
                if x == 100 && y == 50 && d == 0 {
                    x = 75;
                    y = 25;
                    break;
                }
                if wall != 0 && (x == 0 || y == 0 || x == 800 || y == 800) {
                    break;
                }
            }

            x += mv[d][0];
            y += mv[d][1];
            wall = 1;

            if v != 0 {
                // mir[x/100][(y-50)/100][x/100][(y+50)/100]
                let a = (x / 100) as usize;
                let b = ((y - 50) / 100) as usize;
                let c = (x / 100) as usize;
                let e = ((y + 50) / 100) as usize;
                if mir[a][b][c][e] != 0 {
                    d = tr[1][d];
                    wall = 0;
                }
            } else {
                // mir[(x-50)/100][y/100][(x+50)/100][y/100]
                let a = ((x - 50) / 100) as usize;
                let b = (y / 100) as usize;
                let c = ((x + 50) / 100) as usize;
                let e = (y / 100) as usize;
                if mir[a][b][c][e] != 0 {
                    d = tr[0][d];
                    wall = 0;
                }
            }

            i += 1;
            v = if v == 0 { 1 } else { 0 };
        }

        out.push_str(&format!("{} {}\n", x, y));
    }

    print!("{out}");
}
