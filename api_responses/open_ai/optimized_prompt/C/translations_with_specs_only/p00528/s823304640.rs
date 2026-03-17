use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    point: Point,
    cost: i64,
}

// Implementing Ord and PartialOrd for Edge to use in the priority queue
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

fn dijkstra(m: usize, n: usize, special_points: Vec<Point>) -> i64 {
    // Directions for moving in the grid (up, down, left, right)
    let directions = vec![
        Point { x: 0, y: 1 },  // right
        Point { x: 1, y: 0 },  // down
        Point { x: 0, y: isize::MAX as usize }, // left (not used)
        Point { x: isize::MAX as usize, y: 0 }, // up (not used)
    ];

    // Priority queue for Dijkstra's algorithm
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<Point, i64> = HashMap::new();

    // Initialize distances to a large number
    for point in &special_points {
        distances.insert(*point, i64::MAX);
    }

    // Starting point (first special point) and goal point (last special point)
    let start = special_points.get(0).unwrap();
    let goal = special_points.last().unwrap();

    // Set the distance to the starting point to 0
    distances.insert(*start, 0);
    heap.push(Edge { point: *start, cost: 0 });

    while let Some(Edge { point, cost }) = heap.pop() {
        // If we reach the goal, return the cost
        if point == *goal {
            return cost;
        }

        // If the cost is greater than the recorded distance, skip
        if cost > distances[&point] {
            continue;
        }

        // Explore neighbors
        for direction in &directions {
            let new_x = point.x.wrapping_add(direction.x);
            let new_y = point.y.wrapping_add(direction.y);

            // Check bounds
            if new_x < m && new_y < n {
                let neighbor = Point { x: new_x, y: new_y };
                let new_cost = cost + 1; // Assuming uniform cost for edges

                // If the new cost is less, update and push to the heap
                if new_cost < distances.get(&neighbor).unwrap_or(&i64::MAX) {
                    distances.insert(neighbor, new_cost);
                    heap.push(Edge { point: neighbor, cost: new_cost });
                }
            }
        }
    }

    // If we exit the loop without finding the goal, return -1
    -1
}

fn main() {
    let m = 5; // Example grid width
    let n = 5; // Example grid height
    let k = 3; // Number of special points
    let special_points = vec![
        Point { x: 0, y: 0 }, // Starting point
        Point { x: 1, y: 1 }, // Intermediate point
        Point { x: 4, y: 4 }, // Goal point
    ];

    let result = dijkstra(m, n, special_points);
    println!("{}", result); // Output the minimum distance
}
