use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Cell {
    level: i32,
    row: usize,
    col: usize,
}

// Implementing Ord and PartialOrd to use BinaryHeap as a min-heap
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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

impl Eq for Cell {}

fn min_combined_authentication_level(r: usize, w: usize, h: usize, x1: usize, y1: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; w]; h];
    let mut heap = BinaryHeap::new();
    
    // Convert 1-based indices to 0-based
    let start_x = x1 - 1;
    let start_y = y1 - 1;

    // Push the starting cell into the heap
    heap.push(Cell { level: grid[start_y][start_x], row: start_y, col: start_x });
    visited[start_y][start_x] = true;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up
    let mut min_level = i32::MAX;

    while let Some(Cell { level, row, col }) = heap.pop() {
        // Check if we have reached a valid cell
        if level < min_level {
            min_level = level;
        }

        for &(dx, dy) in &directions {
            let new_row = row as isize + dy;
            let new_col = col as isize + dx;

            if new_row >= 0 && new_row < h as isize && new_col >= 0 && new_col < w as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] {
                    visited[new_row][new_col] = true;
                    heap.push(Cell { level: grid[new_row][new_col], row: new_row, col: new_col });
                }
            }
        }
    }

    min_level
}

fn main() {
    let test_cases = vec![
        (1, 3, 3, 1, 1, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        // Add more test cases as needed
    ];

    for (r, w, h, x, y, grid) in test_cases {
        let result = min_combined_authentication_level(r, w, h, x, y, &grid);
        println!("{}", result);
    }
}
