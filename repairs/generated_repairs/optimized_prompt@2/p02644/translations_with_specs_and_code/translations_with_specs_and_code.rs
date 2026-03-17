use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = 1001001001;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut dimensions = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let h = dimensions.next().unwrap();
    let w = dimensions.next().unwrap();
    let k = dimensions.next().unwrap();

    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut coordinates = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let s_x = coordinates.next().unwrap() - 1;
    let s_y = coordinates.next().unwrap() - 1;
    let g_x = coordinates.next().unwrap() - 1;
    let g_y = coordinates.next().unwrap() - 1;

    let mut grid = vec![String::new(); h as usize];
    for i in 0..h {
        input.clear();
        reader.read_line(&mut input).unwrap();
        grid[i as usize] = input.trim().to_string();
    }

    let mut distance = vec![vec![INF; w as usize]; h as usize];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((s_x, s_y));
    distance[s_x as usize][s_y as usize] = 0;

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &directions {
            for j in 1..=k {
                let new_x = x + j * dx;
                let new_y = y + j * dy;

                if new_x >= 0 && new_x < h && new_y >= 0 && new_y < w {
                    if grid[new_x as usize].chars().nth(new_y as usize).unwrap() == '.' {
                        if distance[new_x as usize][new_y as usize] > distance[x as usize][y as usize] + 1 {
                            distance[new_x as usize][new_y as usize] = distance[x as usize][y as usize] + 1;
                            queue.push_back((new_x, new_y));
                        }
                    } else {
                        break; // Stop if we hit an obstacle
                    }
                } else {
                    break; // Stop if out of bounds
                }
            }
        }
    }

    let result = if distance[g_x as usize][g_y as usize] == INF {
        -1
    } else {
        distance[g_x as usize][g_y as usize]
    };

    println!("{}", result);
}
