use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Start,
    Target,
    Bomb,
    Obstacle,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    terrain: Terrain,
    cost: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Position,
    cost: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(height: usize, width: usize, costs: (i64, i64), map: Vec<Vec<char>>) -> Self {
        let mut cells = vec![vec![Cell { terrain: Terrain::Empty, cost: 0 }; width]; height];

        for i in 0..height {
            for j in 0..width {
                cells[i][j] = match map[i][j] {
                    'S' => Cell { terrain: Terrain::Start, cost: 0 },
                    'T' => Cell { terrain: Terrain::Target, cost: 0 },
                    'B' => Cell { terrain: Terrain::Bomb, cost: costs.0 },
                    '#' => Cell { terrain: Terrain::Obstacle, cost: i64::MAX }, // Impassable
                    '.' => Cell { terrain: Terrain::Empty, cost: costs.1 },
                    _ => Cell { terrain: Terrain::Empty, cost: i64::MAX }, // Invalid character
                };
            }
        }

        Grid { cells, height, width }
    }

    fn dijkstra(&self) -> Result<i64, &'static str> {
        let mut min_costs = HashMap::new();
        let mut heap = BinaryHeap::new();
        let start_pos = self.find_position(Terrain::Start)?;
        let target_pos = self.find_position(Terrain::Target)?;

        min_costs.insert(start_pos, 0);
        heap.push(State { position: start_pos, cost: 0 });

        while let Some(State { position, cost }) = heap.pop() {
            if position == target_pos {
                return Ok(cost);
            }

            if cost > *min_costs.get(&position).unwrap_or(&i64::MAX) {
                continue;
            }

            for &delta in &[(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
                let new_x = position.x.wrapping_add(delta.0);
                let new_y = position.y.wrapping_add(delta.1);

                if new_x < self.height && new_y < self.width {
                    let next_cell = &self.cells[new_x][new_y];
                    let new_cost = cost + next_cell.cost;

                    if new_cost < *min_costs.get(&Position { x: new_x, y: new_y }).unwrap_or(&i64::MAX) {
                        min_costs.insert(Position { x: new_x, y: new_y }, new_cost);
                        heap.push(State { position: Position { x: new_x, y: new_y }, cost: new_cost });
                    }
                }
            }
        }

        Err("Target is unreachable")
    }

    fn find_position(&self, terrain: Terrain) -> Result<Position, &'static str> {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.cells[i][j].terrain == terrain {
                    return Ok(Position { x: i, y: j });
                }
            }
        }
        Err("Start or target position not found")
    }
}

fn main() {
    let height = 5;
    let width = 5;
    let costs = (1, 1);
    let map = vec![
        vec!['S', '.', '.', '#', 'T'],
        vec!['.', '#', '.', '.', '.'],
        vec!['.', '.', '#', 'B', '.'],
        vec!['#', '.', '.', '.', '.'],
        vec!['.', '.', '.', '#', '.'],
    ];

    let grid = Grid::new(height, width, costs, map);
    match grid.dijkstra() {
        Ok(cost) => println!("Minimum cost: {}", cost),
        Err(message) => println!("{}", message),
    }
}
