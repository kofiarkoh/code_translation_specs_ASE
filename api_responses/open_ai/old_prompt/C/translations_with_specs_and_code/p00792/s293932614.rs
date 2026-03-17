// AOJ 1212: Mirror Illusion
// Translated from C to Rust

use std::io::{self, BufRead};

const GRID_SIZE: usize = 800;
const MV: [[i32; 2]; 4] = [[50, 50], [-50, 50], [-50, -50], [50, -50]];
const TR: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().expect("Invalid input");

        if n < 0 {
            break;
        }

        let mut mir = [[[0; 9]; 9]; 9]; // 4D array initialized to zero

        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let s = parts[0];
            let x: usize = parts[1].parse().expect("Invalid input");
            let y: usize = parts[2].parse().expect("Invalid input");

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

        for (i, v) in (0..).enumerate() {
            if i >= 2 {
                if x == 100 && y == 50 && d == 0 {
                    x = 75;
                    y = 25;
                    break;
                }
                if wall != 0 && (x == 0 || y == 0 || x == GRID_SIZE as usize || y == GRID_SIZE as usize) {
                    break;
                }
            }

            x = (x as i32 + MV[d][0]) as usize;
            y = (y as i32 + MV[d][1]) as usize;
            wall = 1;

            if v % 2 == 1 {
                if mir[x / 100][(y - 50) / 100][x / 100][(y + 50) / 100] != 0 {
                    d = TR[v % 2][d];
                    wall = 0;
                }
            } else {
                if mir[(x - 50) / 100][y / 100][(x + 50) / 100][y / 100] != 0 {
                    d = TR[v % 2][d];
                    wall = 0;
                }
            }
        }

        println!("{} {}", x, y);
    }
}
