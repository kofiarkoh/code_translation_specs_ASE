// AOJ 1212: Mirror Illusion
// 2018.1.23 bal4u@uu

use std::io::{self, BufRead};

const SIZE: usize = 9;
const MV: [[i32; 2]; 4] = [[50, 50], [-50, 50], [-50, -50], [50, -50]];
const TR: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

fn main() {
    let stdin = io::stdin();
    let mut mir = [[[0; SIZE]; SIZE]; SIZE]; // 4D array initialized to 0
    let mut input = String::new();

    while stdin.lock().read_line(&mut input).unwrap() > 0 {
        let n: i32 = input.trim().parse().unwrap();
        if n < 0 {
            break;
        }
        mir = [[[0; SIZE]; SIZE]; SIZE]; // Reset the mirror array

        for _ in 0..n {
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let s = parts[0];
            let x: usize = parts[1].parse().unwrap();
            let y: usize = parts[2].parse().unwrap();
            if s == "x" {
                mir[x][y][x + 1][y] = 1;
            } else {
                mir[x][y][x][y + 1] = 1;
            }
        }

        let mut x = 50;
        let mut y = 0;
        let mut d = 0;
        let mut wall = 1;
        let mut v = 1;

        loop {
            if x == 100 && y == 50 && d == 0 {
                x = 75;
                y = 25;
                break;
            }
            if wall != 0 && (x == 0 || y == 0 || x == 800 || y == 800) {
                break;
            }

            x += MV[d][0];
            y += MV[d][1];
            wall = 1;

            if v != 0 {
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
        }
        println!("{} {}", x, y);
    }
}
