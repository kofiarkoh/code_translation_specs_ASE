use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Cell {
    level: i32,
    x: usize,
    y: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.level.cmp(&self.level)
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

fn min_combined_authentication_level(test_cases: Vec<(usize, usize, (usize, usize), Vec<Vec<i32>>)>) -> Vec<i32> {
    let mut results = Vec::new();

    for (width, height, start_coords, grid) in test_cases {
        let (start_x, start_y) = start_coords;
        let mut min_heap = BinaryHeap::new();
        let mut visited = vec![vec![false; width]; height];

        // Directions for moving in the grid (right, down, left, up)
        let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];

        // Start from the initial coordinates
        min_heap.push(Cell { level: grid[start_y][start_x], x: start_x, y: start_y });
        visited[start_y][start_x] = true;

        let mut min_level = i32::MAX;

        while let Some(Cell { level, x, y }) = min_heap.pop() {
            // Update the minimum level found
            min_level = min_level.min(level);

            // Explore neighbors
            for (dx, dy) in &directions {
                let new_x = if *dx == usize::MAX { x.wrapping_sub(1) } else { x.wrapping_add(*dx) };
                let new_y = if *dy == usize::MAX { y.wrapping_sub(1) } else { y.wrapping_add(*dy) };

                if new_x < width && new_y < height && !visited[new_y][new_x] {
                    visited[new_y][new_x] = true;
                    min_heap.push(Cell { level: grid[new_y][new_x], x: new_x, y: new_y });
                }
            }
        }

        results.push(min_level);
    }

    results
}

fn main() {
    // Example usage
    let test_cases = vec![
        (3, 3, (0, 0), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        (2, 2, (0, 0), vec![vec![1, 2], vec![3, 4]]),
    ];

    let results = min_combined_authentication_level(test_cases);
    for result in results {
        println!("{}", result);
    }
}
