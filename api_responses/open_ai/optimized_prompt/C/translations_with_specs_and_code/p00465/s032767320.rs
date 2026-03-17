// AOJ 0542: Authentication Level
// Translated from C to Rust

use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cell {
    t: i32,
    r: usize,
    c: usize,
}

// Implementing Ord for Cell to use it in a min-heap
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t) // Reverse order for min-heap
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut first_line = lines.next().unwrap();
    let test_cases: usize = first_line.trim().parse().unwrap();

    for _ in 0..test_cases {
        let dimensions: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (width, height) = (dimensions[0], dimensions[1]);

        let start_coords: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap() - 1) // Convert to 0-based index
            .collect();
        let (start_x, start_y) = (start_coords[0], start_coords[1]);

        let mut grid = vec![vec![0; width]; height];
        for r in 0..height {
            let levels: Vec<i32> = lines.next().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for c in 0..width {
                grid[r][c] = levels[c];
            }
        }

        let mut levels = vec![vec![0; 100001]; 2];
        let mut lsz = [1, 1];
        let mut max = 0;

        for i in 0..2 {
            let mut visited = vec![vec![false; width]; height];
            let mut heap = BinaryHeap::new();
            heap.push(Cell { t: grid[start_y][start_x], r: start_y, c: start_x });
            visited[start_y][start_x] = true;

            while let Some(Cell { t, r, c }) = heap.pop() {
                if t > max {
                    max = t;
                }
                levels[i][lsz[i]] = max;
                lsz[i] += 1;

                let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for &(dr, dc) in &directions {
                    let (new_r, new_c) = (r as isize + dr, c as isize + dc);
                    if new_r < 0 || new_r >= height as isize || new_c < 0 || new_c >= width as isize {
                        continue;
                    }
                    let (new_r, new_c) = (new_r as usize, new_c as usize);
                    if visited[new_r][new_c] {
                        continue;
                    }
                    visited[new_r][new_c] = true;
                    heap.push(Cell { t: grid[new_r][new_c], r: new_r, c: new_c });
                }
            }
        }

        let mut ans = i32::MAX;
        for i in 0..=test_cases {
            let j = test_cases - i;
            if i < lsz[0] && j < lsz[1] {
                let t = levels[0][i] + levels[1][j];
                if t < ans {
                    ans = t;
                }
            }
        }
        println!("{}", ans);
    }
}
