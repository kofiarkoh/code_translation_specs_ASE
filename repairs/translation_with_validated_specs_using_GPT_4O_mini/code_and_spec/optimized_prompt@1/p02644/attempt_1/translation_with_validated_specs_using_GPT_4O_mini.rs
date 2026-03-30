use std::io::{self, BufRead};

const INF: i32 = 1001001001;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read grid dimensions and max move distance
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let h = params.next().unwrap();
    let w = params.next().unwrap();
    let k = params.next().unwrap();

    // Read starting and target coordinates
    let second_line = lines.next().unwrap().unwrap();
    let mut coords = second_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let s = [coords.next().unwrap() - 1, coords.next().unwrap() - 1];
    let g = [coords.next().unwrap() - 1, coords.next().unwrap() - 1];

    // Initialize the grid and distance array
    let mut grid = vec![String::new(); h as usize];
    let mut d = vec![vec![INF; w as usize]; h as usize];

    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid[i as usize] = line;
        for j in 0..w {
            d[i as usize][j as usize] = INF;
        }
    }

    // Directions for movement
    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    let mut queue = vec![(s[0], s[1])];
    d[s[0] as usize][s[1] as usize] = 0;

    let mut l = 0;

    while l < queue.len() {
        let (x, y) = queue[l];
        l += 1;

        for i in 0..4 {
            for j in 1..=k {
                let new_x = x + j * dx[i];
                let new_y = y + j * dy[i];

                if new_x >= 0 && new_x < h && new_y >= 0 && new_y < w && grid[new_x as usize].chars().nth(new_y as usize).unwrap() == '.' {
                    if d[new_x as usize][new_y as usize] >= d[x as usize][y as usize] + 1 {
                        if d[new_x as usize][new_y as usize] == d[x as usize][y as usize] + 1 {
                            continue;
                        }
                        d[new_x as usize][new_y as usize] = d[x as usize][y as usize] + 1;
                        queue.push((new_x, new_y));
                    }
                } else {
                    break; // Stop if we hit an obstacle or out of bounds
                }
            }
        }
    }

    let result = if d[g[0] as usize][g[1] as usize] == INF {
        -1
    } else {
        d[g[0] as usize][g[1] as usize]
    };

    println!("{}", result);
}
