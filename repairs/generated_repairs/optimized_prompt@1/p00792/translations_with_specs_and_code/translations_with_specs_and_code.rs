use std::io::{self, BufRead};

const MV: [[i32; 2]; 4] = [[50, 50], [-50, 50], [-50, -50], [50, -50]];
const TR: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();
        if n < 0 {
            break;
        }

        let mut mir = [[[0; 9]; 9]; 9]; // 4D array initialized to 0

        for _ in 0..n {
            let mut mirror_input = String::new();
            reader.read_line(&mut mirror_input).unwrap();
            let parts: Vec<&str> = mirror_input.trim().split_whitespace().collect();
            let s = parts[0];
            let x: usize = parts[1].parse().unwrap();
            let y: usize = parts[2].parse().unwrap();

            if s == "x" {
                mir[x / 100][y / 100][(x + 1) / 100][y / 100] = 1;
            } else {
                mir[x / 100][y / 100][x / 100][(y + 1) / 100] = 1;
            }
        }

        let mut x = 50;
        let mut y = 0;
        let mut d = 0;
        let mut wall = 1;

        let mut v = 1;
        let mut i = 0;

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

            x += MV[d][0];
            y += MV[d][1];
            wall = 1;

            if v == 1 {
                if mir[x as usize / 100][(y - 50) as usize / 100][x as usize / 100][(y + 50) as usize / 100] != 0 {
                    d = TR[v][d];
                    wall = 0;
                }
            } else {
                if mir[(x - 50) as usize / 100][y as usize / 100][(x + 50) as usize / 100][y as usize / 100] != 0 {
                    d = TR[v][d];
                    wall = 0;
                }
            }

            v = !v;
            i += 1;
        }

        println!("{} {}", x, y);
    }
}
