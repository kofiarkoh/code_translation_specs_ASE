use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();
    while reader.read_line(&mut input).unwrap() > 0 {
        let n: usize = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut grid = [[0; 25]; 25];
        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let coords: Vec<usize> = input
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (x, y) = (coords[0], coords[1]);
            grid[x][y] = 1;
        }

        input.clear();
        reader.read_line(&mut input).unwrap();
        let m: usize = input.trim().parse().unwrap();

        let mut px = 10;
        let mut py = 10;

        // Check if the starting position has an obstacle
        if grid[px][py] == 1 {
            grid[px][py] = 0;
            n -= 1;
        }

        for _ in 0..m {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            let direction = parts[0];
            let steps: usize = parts[1].parse().unwrap();

            let (dx, dy) = match direction {
                "N" => (0, -1),
                "E" => (1, 0),
                "S" => (0, 1),
                "W" => (-1, 0),
                _ => (0, 0), // This case should not happen as per the spec
            };

            for _ in 0..steps {
                px = (px as isize + dx) as usize;
                py = (py as isize + dy) as usize;

                // Check if the new position has an obstacle
                if grid[px][py] == 1 {
                    grid[px][py] = 0;
                    n -= 1;
                }
            }
        }

        // Output the result based on the number of remaining obstacles
        if n > 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
