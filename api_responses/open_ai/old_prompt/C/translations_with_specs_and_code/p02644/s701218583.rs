use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = 1001001001;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read h, w, k
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let h = params.next().unwrap();
    let w = params.next().unwrap();
    let k = params.next().unwrap();

    // Read starting and goal positions
    let second_line = lines.next().unwrap().unwrap();
    let mut positions = second_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let s = [positions.next().unwrap() - 1, positions.next().unwrap() - 1];
    let g = [positions.next().unwrap() - 1, positions.next().unwrap() - 1];

    // Read the grid
    let mut grid = vec![String::new(); h as usize];
    let mut d = vec![vec![INF; w as usize]; h as usize];

    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid[i as usize] = line;
    }

    // BFS initialization
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((s[0], s[1]));
    d[s[0] as usize][s[1] as usize] = 0;

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &directions {
            for j in 1..=k {
                let new_x = x + j * dx;
                let new_y = y + j * dy;

                if new_x >= 0 && new_x < h && new_y >= 0 && new_y < w &&
                   grid[new_x as usize].chars().nth(new_y as usize).unwrap() == '.' &&
                   d[new_x as usize][new_y as usize] >= d[x as usize][y as usize] + 1 {
                    
                    if d[new_x as usize][new_y as usize] == d[x as usize][y as usize] + 1 {
                        continue;
                    }

                    d[new_x as usize][new_y as usize] = d[x as usize][y as usize] + 1;
                    queue.push_back((new_x, new_y));
                } else {
                    break; // No further movement in this direction
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
