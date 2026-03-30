struct Grid {
    data: Vec<Vec<char>>,
    size: usize,
}

impl Grid {
    fn new(n: usize) -> Self {
        Self {
            data: vec![vec!['0'; n]; n],
            size: n,
        }
    }

    fn toggle(&mut self, x: usize, y: usize) {
        // Toggle the cell at (x, y)
        self.data[x][y] = if self.data[x][y] == '0' { '1' } else { '0' };
    }

    fn is_symmetric(&self) -> bool {
        for x in 0..self.size {
            for y in 0..self.size {
                if self.data[x][y] != self.data[self.size - 1 - x][y] || 
                   self.data[x][y] != self.data[x][self.size - 1 - y] {
                    return false;
                }
            }
        }
        true
    }

    fn count_symmetric_configurations(&self) -> usize {
        // Check if the current grid configuration is symmetric
        if self.is_symmetric() {
            1 // Count this configuration as symmetric
        } else {
            0 // Not symmetric
        }
    }
}

fn process_grid(c: usize, n: usize, initial_grid: Vec<Vec<char>>, updates: Vec<(usize, usize)>) -> usize {
    let mut grid = Grid::new(n);
    grid.data = initial_grid;

    // Apply all toggle operations
    for (x, y) in updates {
        grid.toggle(x, y);
    }

    // Count the number of symmetric configurations after all updates
    grid.count_symmetric_configurations()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let c: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut initial_grid = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        initial_grid.push(line.chars().collect());
    }

    let mut updates = Vec::new();
    for _ in 0..c {
        let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse::<usize>().unwrap()).collect();
        // Ensure the coordinates are valid
        if coords.len() == 2 {
            updates.push((coords[0], coords[1]));
        }
    }

    let result = process_grid(c, n, initial_grid, updates);
    println!("{}", result);
}
