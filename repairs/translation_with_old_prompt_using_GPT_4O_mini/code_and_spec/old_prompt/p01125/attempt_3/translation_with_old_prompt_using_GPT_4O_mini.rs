use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(line) = input.next() {
        let n: i64 = line.unwrap().trim().parse().expect("Invalid input");
        if n == 0 {
            println!("Yes");
            continue;
        }

        let mut grid = [[0; 25]; 25];
        for _ in 0..n {
            let coords: Vec<i64> = input.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.trim().parse().expect("Invalid input"))
                .collect();
            let x = coords[0] as usize;
            let y = coords[1] as usize;
            grid[x][y] = 1;
        }

        let m: i64 = input.next().unwrap().unwrap().trim().parse().expect("Invalid input");
        let mut px = 10;
        let mut py = 10;

        // Collect item if present at starting position
        if grid[px][py] == 1 {
            grid[px][py] = 0;
        }

        for _ in 0..m {
            let move_line = input.next().unwrap().unwrap();
            let parts: Vec<&str> = move_line.split_whitespace().collect();
            let direction = parts[0];
            let q: i64 = parts[1].parse().expect("Invalid input");

            let d = match direction {
                "N" => 0,
                "E" => 1,
                "S" => 2,
                "W" => 3,
                _ => unreachable!(),
            };

            for _ in 0..q {
                // Move the player
                let new_px = (px as i64 + D4[d]) as usize;
                let new_py = (py as i64 + D4[d + 1]) as usize;

                // Check bounds before updating the position
                if new_px < 25 && new_py < 25 {
                    px = new_px;
                    py = new_py;

                    // Collect the item if present
                    if grid[px][py] == 1 {
                        grid[px][py] = 0; // Collect the item
                    }
                } else {
                    // If out of bounds, stop moving further in this direction
                    break;
                }
            }
        }

        // Check if all items have been collected
        println!("{}", if grid.iter().flat_map(|row| row.iter()).sum::<i64>() == 0 { "Yes" } else { "No" });
    }
}

const D4: [i64; 4] = [0, 1, 0, -1]; // Directions: N, E, S, W
