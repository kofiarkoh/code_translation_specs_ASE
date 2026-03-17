use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct State {
    cost: u64,
    position: Position,
}

// Implementing Ord and PartialOrd for State to use in the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap, so we reverse the order
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read grid dimensions
    let mut dimensions = String::new();
    reader.read_line(&mut dimensions).unwrap();
    let mut dims = dimensions.split_whitespace();
    let h: usize = dims.next().unwrap().parse().unwrap();
    let w: usize = dims.next().unwrap().parse().unwrap();

    // Read movement costs
    let mut costs = String::new();
    reader.read_line(&mut costs).unwrap();
    let mut cost_values = costs.split_whitespace();
    let a: u64 = cost_values.next().unwrap().parse().unwrap();
    let b: u64 = cost_values.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = Vec::new();
    let mut start = None;
    let mut goal = None;

    for i in 0..h {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let row: Vec<char> = line.trim().chars().collect();
        grid.push(row);

        // Identify start and goal positions
        for j in 0..w {
            if row[j] == 's' {
                start = Some(Position { x: i, y: j });
            } else if row[j] == 'g' {
                goal = Some(Position { x: i, y: j });
            }
        }
    }

    // Ensure start and goal are found
    let start = start.expect("Start position 's' not found");
    let goal = goal.expect("Goal position 'g' not found");

    // Dijkstra's algorithm
    let mut vis = vec![vec![false; w]; h];
    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0, position: start });

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    while let Some(State { cost, position }) = pq.pop() {
        if vis[position.x][position.y] {
            continue;
        }
        vis[position.x][position.y] = true;

        // If we reached the goal, print the cost
        if position == goal {
            println!("{}", cost);
            return;
        }

        // Explore neighbors
        for &(dx, dy) in &directions {
            let new_x = position.x as isize + dx;
            let new_y = position.y as isize + dy;

            if new_x >= 0 && new_x < h as isize && new_y >= 0 && new_y < w as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if !vis[new_x][new_y] {
                    let new_cost = cost + if grid[new_x][new_y] == '*' { b } else { a };
                    pq.push(State { cost: new_cost, position: Position { x: new_x, y: new_y } });
                }
            }
        }
    }

    // If we exit the loop without finding the goal, print "INF"
    println!("INF");
}
