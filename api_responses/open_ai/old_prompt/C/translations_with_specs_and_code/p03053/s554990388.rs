use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = 1001001001;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read height and width
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    // Initialize the grid and distance array
    let mut a = vec![vec!['.'; w]; h];
    let mut b = vec![vec![INF; w]; h];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Read the grid and initialize distances
    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        for j in 0..w {
            a[i][j] = line.chars().nth(j).unwrap();
            if a[i][j] == '#' {
                b[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // Directions for BFS (right, down, left, up)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    // BFS to calculate distances
    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < h && ny < w && a[nx][ny] == '.' && b[nx][ny] > b[x][y] + 1 {
                b[nx][ny] = b[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    // Find the maximum distance
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(b[i][j]);
        }
    }

    // Print the result
    println!("{}", ans);
}
