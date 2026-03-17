use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();
    while reader.read_line(&mut input).unwrap() > 0 {
        let n: i64 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut grid = [[0; 25]; 25];
        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let coords: Vec<i64> = input.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let x = coords[0] as usize;
            let y = coords[1] as usize;
            grid[x][y] = 1;
        }

        let mut m_input = String::new();
        reader.read_line(&mut m_input).unwrap();
        let m: i64 = m_input.trim().parse().unwrap();

        let mut px = 10;
        let mut py = 10;

        // Collect item if present at starting position
        if grid[px][py] == 1 {
            grid[px][py] = 0;
        }

        for _ in 0..m {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            let direction = parts[0].chars().next().unwrap();
            let q: i64 = parts[1].parse().unwrap();

            let (dx, dy) = match direction {
                'N' => (0, -1),
                'E' => (1, 0),
                'S' => (0, 1),
                'W' => (-1, 0),
                _ => (0, 0), // This case should not happen as per the spec
            };

            for _ in 0..q {
                px = (px as isize + dx) as usize;
                py = (py as isize + dy) as usize;

                // Check bounds before accessing grid
                if px < 25 && py < 25 && grid[px][py] == 1 {
                    grid[px][py] = 0;
                }
            }
        }

        // Output result based on whether all items have been collected
        let all_collected = grid.iter().flat_map(|row| row.iter()).all(|&cell| cell == 0);
        println!("{}", if all_collected { "Yes" } else { "No" });
    }
}
