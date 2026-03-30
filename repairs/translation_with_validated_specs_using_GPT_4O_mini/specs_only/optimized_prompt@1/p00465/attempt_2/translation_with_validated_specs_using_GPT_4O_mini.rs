use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq)]
struct Cell {
    level: i32,
    x: usize,
    y: usize,
}

// Implementing Ord to allow priority queue to sort by authentication level
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.level.cmp(&self.level) // Reverse order for min-heap behavior
    }
}

// Implementing PartialOrd for Cell
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implementing PartialEq for Cell
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

fn min_combined_authentication_level(test_cases: Vec<(usize, usize, usize, usize, Vec<Vec<i32>>)>) -> Vec<i32> {
    let mut results = Vec::new();

    for (w, h, x, y, grid) in test_cases {
        let mut visited = vec![vec![false; w]; h];
        let mut pq = BinaryHeap::new();
        
        // Convert 1-based coordinates to 0-based
        let start_x = x - 1;
        let start_y = y - 1;

        // Push the starting cell into the priority queue
        pq.push(Cell { level: grid[start_y][start_x], x: start_x, y: start_y });
        visited[start_y][start_x] = true;

        let mut min_sum = 0;

        // Directions for moving in the grid (right, down, left, up)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Cell { level, x, y }) = pq.pop() {
            min_sum += level;

            // Explore neighbors
            for &(dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x >= 0 && new_x < w as isize && new_y >= 0 && new_y < h as isize {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    if !visited[new_y][new_x] {
                        visited[new_y][new_x] = true;
                        pq.push(Cell { level: grid[new_y][new_x], x: new_x, y: new_y });
                    }
                }
            }
        }

        results.push(min_sum);
    }

    results
}

fn main() {
    let test_cases = vec![
        (3, 3, 1, 1, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]),
    ];
    let results = min_combined_authentication_level(test_cases);
    for result in results {
        println!("{}", result);
    }
}
