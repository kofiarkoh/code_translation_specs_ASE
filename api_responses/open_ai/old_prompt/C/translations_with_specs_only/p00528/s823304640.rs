use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u64,
    position: Point,
}

// Implementing Ord and PartialOrd for State to use in the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(m: usize, n: usize, obstacles: &Vec<Point>, start: Point, goal: Point) -> i64 {
    // Create a grid to mark obstacles
    let mut grid = vec![vec![true; n]; m];
    for &obstacle in obstacles {
        grid[obstacle.x - 1][obstacle.y - 1] = false; // Mark obstacle positions
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = vec![
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: -1 },
    ];

    // Priority queue for Dijkstra's algorithm
    let mut heap = BinaryHeap::new();
    let mut distances = vec![vec![u64::MAX; n]; m];
    distances[start.x - 1][start.y - 1] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost as i64; // Return the cost if we reached the goal
        }

        // If the cost is greater than the recorded distance, skip
        if cost > distances[position.x - 1][position.y - 1] {
            continue;
        }

        // Explore neighbors
        for &dir in &directions {
            let new_x = position.x as isize + dir.x as isize;
            let new_y = position.y as isize + dir.y as isize;

            if new_x >= 1 && new_x <= m as isize && new_y >= 1 && new_y <= n as isize {
                let new_pos = Point {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if grid[new_pos.x - 1][new_pos.y - 1] { // Check if not an obstacle
                    let new_cost = cost + 1; // Assuming cost to move to adjacent cell is 1
                    if new_cost < distances[new_pos.x - 1][new_pos.y - 1] {
                        distances[new_pos.x - 1][new_pos.y - 1] = new_cost;
                        heap.push(State { cost: new_cost, position: new_pos });
                    }
                }
            }
        }
    }

    -1 // Return -1 if no path exists
}

fn main() {
    let m = 5; // Example grid rows
    let n = 5; // Example grid columns
    let k = 2; // Number of obstacles
    let obstacles = vec![
        Point { x: 2, y: 2 },
        Point { x: 3, y: 3 },
    ];
    let start = Point { x: 1, y: 1 }; // Starting point
    let goal = Point { x: 5, y: 5 }; // Goal point

    let result = dijkstra(m, n, &obstacles, start, goal);
    println!("{}", result); // Output the result
}
