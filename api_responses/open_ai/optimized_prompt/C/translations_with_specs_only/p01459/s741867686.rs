use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Start,
    Goal,
    Obstacle,
    Empty,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let height = cells.len();
        let width = cells[0].len();
        Grid { cells, height, width }
    }

    fn find_positions(&self) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let mut start = None;
        let mut goal = None;

        for i in 0..self.height {
            for j in 0..self.width {
                match self.cells[i][j] {
                    Cell::Start => start = Some((i, j)),
                    Cell::Goal => goal = Some((i, j)),
                    _ => {}
                }
            }
        }
        (start, goal)
    }

    fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < self.height && y < self.width && self.cells[x][y] != Cell::Obstacle
    }
}

fn bfs(grid: &Grid, start: (usize, usize), goal: (usize, usize), max_moves: (usize, usize)) -> Result<usize, &'static str> {
    let (max_up, max_down) = max_moves;
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid.width]; grid.height];
    let mut moves = 0;

    queue.push_back((start, moves));
    visited[start.0][start.1] = true;

    while let Some(((x, y), current_moves)) = queue.pop_front() {
        if (x, y) == goal {
            return Ok(current_moves);
        }

        // Explore possible moves
        for dx in (1..=max_down).chain((1..=max_up).map(|d| -d)) {
            for dy in (1..=max_down).chain((1..=max_up).map(|d| -d)) {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if grid.is_valid_position(new_x, new_y) && !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    queue.push_back(((new_x, new_y), current_moves + 1));
                }
            }
        }
    }

    Err("Goal not reachable within constraints")
}

fn main() {
    // Example grid initialization
    let grid_data = vec![
        vec![Cell::Start, Cell::Empty, Cell::Empty],
        vec![Cell::Obstacle, Cell::Empty, Cell::Goal],
        vec![Cell::Empty, Cell::Empty, Cell::Empty],
    ];
    
    let grid = Grid::new(grid_data);
    let (start, goal) = grid.find_positions();

    if let (Some(start_pos), Some(goal_pos)) = (start, goal) {
        // Define maximum moves allowed in two directions
        let max_moves = (2, 2); // Example values
        match bfs(&grid, start_pos, goal_pos, max_moves) {
            Ok(moves) => println!("Minimum moves required: {}", moves),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Invalid grid: Start or Goal position not found.");
    }
}
