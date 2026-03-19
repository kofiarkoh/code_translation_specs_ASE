use std::fmt;

struct BipartiteMatcher {
    n: usize,
    grid: Vec<Vec<u8>>,
    match_arr: Vec<i32>,
    seen: Vec<bool>,
}

impl BipartiteMatcher {
    fn new(n: usize, grid: Vec<Vec<u8>>) -> Self {
        BipartiteMatcher {
            n,
            grid,
            match_arr: vec![-1; n],
            seen: vec![false; n],
        }
    }

    fn can_match(&mut self, u: usize) -> bool {
        for v in 0..self.n {
            if self.grid[u][v] == 1 && !self.seen[v] {
                self.seen[v] = true; // Mark this node as seen
                if self.match_arr[v] == -1 || self.can_match(self.match_arr[v] as usize) {
                    self.match_arr[v] = u as i32; // Match u with v
                    return true;
                }
            }
        }
        false
    }

    fn find_perfect_matching(&mut self) -> bool {
        for u in 0..self.n {
            self.seen.fill(false); // Reset seen for each node
            if !self.can_match(u) {
                return false; // If we can't match this node, no perfect matching exists
            }
        }
        true
    }

    fn get_matching_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for v in 0..self.n {
            if self.match_arr[v] != -1 {
                pairs.push((self.match_arr[v] as usize, v));
            }
        }
        pairs
    }
}

fn main() {
    let n: usize = 3; // Example input, should be read from input in a real scenario
    let grid: Vec<Vec<u8>> = vec![
        vec![1, 0, 1],
        vec![1, 1, 0],
        vec![0, 1, 1],
    ]; // Example grid, should be read from input in a real scenario

    let mut matcher = BipartiteMatcher::new(n, grid);
    if matcher.find_perfect_matching() {
        println!("yes");
        let pairs = matcher.get_matching_pairs();
        for (u, v) in pairs {
            println!("{} {}", u, v);
        }
    } else {
        println!("no");
    }
}
