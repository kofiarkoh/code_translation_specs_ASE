use std::io::{self, BufRead};

const MAX_COORD: usize = 800;
const INITIAL_X: i32 = 50;
const INITIAL_Y: i32 = 0;
const MOVEMENTS: [(i32, i32); 4] = [(50, 50), (-50, 50), (-50, -50), (50, -50)];
const TRANSITIONS: [[usize; 4]; 2] = [[3, 2, 1, 0], [1, 0, 3, 2]];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();

        if n < 0 {
            break; // Terminate if the input integer is negative
        }

        let mut mirrors = [[[0; 9]; 9]; 9]; // 4D array for mirrors
        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let orientation = parts[0];
            let x: usize = parts[1].parse().unwrap();
            let y: usize = parts[2].parse().unwrap();

            // Check bounds before placing mirrors
            if x < 9 && y < 9 {
                if orientation == "x" {
                    mirrors[x][y][x + 1][y] = 1;
                } else {
                    mirrors[x][y][x][y + 1] = 1;
                }
            }
        }

        let (mut x, mut y, mut d) = (INITIAL_X, INITIAL_Y, 0);
        let mut wall = true;

        loop {
            if x == 100 && y == 50 && d == 0 {
                x = 75;
                y = 25;
                break;
            }
            if wall && (x == 0 || y == 0 || x == MAX_COORD || y == MAX_COORD) {
                break; // Break on wall collision
            }

            x += MOVEMENTS[d].0;
            y += MOVEMENTS[d].1;
            wall = true;

            if (x / 100) < 9 && ((y - 50) / 100) < 9 && mirrors[x as usize / 100][(y - 50) as usize / 100][x as usize / 100][(y + 50) as usize / 100] != 0 {
                d = TRANSITIONS[1][d];
                wall = false;
            } else if ((x - 50) / 100) < 9 && (y / 100) < 9 && mirrors[(x - 50) as usize / 100][y as usize / 100][(x + 50) as usize / 100][y as usize / 100] != 0 {
                d = TRANSITIONS[0][d];
                wall = false;
            }
        }

        println!("{} {}", x, y);
    }
}
