// puzzle_solver.rs

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Filled,
}

struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

struct Piece {
    height: usize,
    width: usize,
    shape: Vec<Vec<bool>>, // true for filled, false for empty
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        let cells = vec![vec![Cell::Empty; width]; height];
        Grid { height, width, cells }
    }

    fn can_fill(&mut self, pieces: &[Piece]) -> bool {
        if self.height == 0 || self.width == 0 {
            return false; // Early exit for zero dimensions
        }
        self.solve(pieces, 0)
    }

    fn solve(&mut self, pieces: &[Piece], piece_index: usize) -> bool {
        if piece_index == pieces.len() {
            return self.is_filled(); // Check if grid is completely filled
        }

        let piece = &pieces[piece_index];
        for rotation in 0..4 {
            let rotated_shape = self.rotate_piece(piece, rotation);
            for row in 0..self.height {
                for col in 0..self.width {
                    if self.can_place(&rotated_shape, row, col) {
                        self.place(&rotated_shape, row, col);
                        if self.solve(pieces, piece_index + 1) {
                            return true;
                        }
                        self.remove(&rotated_shape, row, col); // Backtrack
                    }
                }
            }
        }
        false // No valid placement found
    }

    fn is_filled(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&cell| cell == Cell::Filled))
    }

    fn can_place(&self, shape: &Vec<Vec<bool>>, start_row: usize, start_col: usize) -> bool {
        for (r, row) in shape.iter().enumerate() {
            for (c, &filled) in row.iter().enumerate() {
                if filled {
                    let grid_row = start_row + r;
                    let grid_col = start_col + c;
                    if grid_row >= self.height || grid_col >= self.width || self.cells[grid_row][grid_col] == Cell::Filled {
                        return false; // Out of bounds or overlap
                    }
                }
            }
        }
        true
    }

    fn place(&mut self, shape: &Vec<Vec<bool>>, start_row: usize, start_col: usize) {
        for (r, row) in shape.iter().enumerate() {
            for (c, &filled) in row.iter().enumerate() {
                if filled {
                    self.cells[start_row + r][start_col + c] = Cell::Filled;
                }
            }
        }
    }

    fn remove(&mut self, shape: &Vec<Vec<bool>>, start_row: usize, start_col: usize) {
        for (r, row) in shape.iter().enumerate() {
            for (c, &filled) in row.iter().enumerate() {
                if filled {
                    self.cells[start_row + r][start_col + c] = Cell::Empty;
                }
            }
        }
    }

    fn rotate_piece(&self, piece: &Piece, rotation: usize) -> Vec<Vec<bool>> {
        match rotation {
            0 => piece.shape.clone(),
            1 => (0..piece.width).map(|c| (0..piece.height).map(|r| piece.shape[piece.height - 1 - r][c]).collect()).collect(),
            2 => (0..piece.height).rev().map(|r| piece.shape[r].iter().cloned().rev().collect()).collect(),
            3 => (0..piece.width).rev().map(|c| (0..piece.height).map(|r| piece.shape[r][piece.width - 1 - c]).collect()).collect(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    // Example usage
    let mut grid = Grid::new(4, 4);
    let pieces = vec![
        Piece {
            height: 2,
            width: 2,
            shape: vec![vec![true, true], vec![true, true]],
        },
        Piece {
            height: 1,
            width: 4,
            shape: vec![vec![true, true, true, true]],
        },
    ];

    let can_fill = grid.can_fill(&pieces);
    let result = if can_fill { "YES" } else { "NO" };
    println!("{}", result);
}
