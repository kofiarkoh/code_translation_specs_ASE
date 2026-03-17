use std::io::{self, BufRead};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let (w, h) = (dimensions[0], dimensions[1]);

    // Validate dimensions
    assert!(w > 0 && h > 0 && w <= 12 && h <= 60, "Invalid grid dimensions");

    let mut grid = Vec::new();
    for line in lines.take(h) {
        let row: Vec<char> = line.unwrap().chars().collect();
        grid.push(row);
    }

    let mut uf = UnionFind::new(w * h);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                for &(di, dj) in &directions {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                        if grid[ni as usize][nj as usize] == '#' {
                            uf.union(i * w + j, ni as usize * w + nj as usize);
                        }
                    }
                }
            }
        }
    }

    // Check stability: if all occupied cells are connected
    let root = uf.find(0);
    let stable = grid.iter().flat_map(|row| row.iter())
        .filter(|&&cell| cell == '#')
        .all(|&cell| uf.find((cell as usize) * w + (cell as usize)) == root);

    if stable {
        println!("STABLE");
    } else {
        println!("UNSTABLE");
    }
}
