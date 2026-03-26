use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    level: i32,
    row: usize,
    col: usize,
}

// Implementing Ord to create a min-heap based on the authentication level
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.level.cmp(&self.level) // Reverse order for min-heap
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_combined_authentication_level(test_cases: Vec<(usize, usize, usize, usize, Vec<Vec<i32>>)>) -> Vec<i32> {
    let mut results = Vec::new();

    for (w, h, x, y, grid) in test_cases {
        let mut visited = vec![vec![false; w]; h];
        let mut heap = BinaryHeap::new();
        
        // Convert 1-based index to 0-based
        let start_row = y - 1;
        let start_col = x - 1;

        // Push the starting cell into the heap
        heap.push(Cell { level: grid[start_row][start_col], row: start_row, col: start_col });
        visited[start_row][start_col] = true;

        let mut max_level = grid[start_row][start_col];

        // Directions for moving in the grid (up, down, left, right)
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Cell { level, row, col }) = heap.pop() {
            max_level = max_level.max(level);

            for &(dr, dc) in &directions {
                let new_row = (row as isize + dr) as usize;
                let new_col = (col as isize + dc) as usize;

                if new_row < h && new_col < w && !visited[new_row][new_col] {
                    visited[new_row][new_col] = true;
                    heap.push(Cell { level: grid[new_row][new_col], row: new_row, col: new_col });
                }
            }
        }

        results.push(max_level);
    }

    results
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let r: usize = lines.next().unwrap().parse().unwrap();
    let mut test_cases = Vec::new();

    for _ in 0..r {
        let (w, h, x, y): (usize, usize, usize, usize) = {
            let parts: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2], parts[3])
        };

        let mut grid = Vec::new();
        for _ in 0..h {
            let row: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            grid.push(row);
        }

        test_cases.push((w, h, x, y, grid));
    }

    let results = min_combined_authentication_level(test_cases);
    for result in results {
        println!("{}", result);
    }
}
