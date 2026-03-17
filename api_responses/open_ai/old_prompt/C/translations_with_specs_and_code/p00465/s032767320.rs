use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    t: i32,
    r: usize,
    c: usize,
}

// Implementing Ord for Cell to use it in a BinaryHeap as a min-heap
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t) // Reverse order for min-heap
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut input = |lines: &mut std::iter::FilterMap<std::io::Lines<io::StdinLock>, fn(Result<String, std::io::Error>) -> Option<String>>| {
        lines.next().unwrap().parse::<usize>().unwrap()
    };

    let mut results = Vec::new();

    while let Ok(r) = input(&mut lines) {
        for i in 0..2 {
            let w = input(&mut lines);
            let h = input(&mut lines);
            let x = input(&mut lines) - 1; // Convert to 0-based index
            let y = input(&mut lines) - 1; // Convert to 0-based index

            let mut l = vec![vec![0; 501]; 501];
            for r in 0..h {
                for c in 0..w {
                    l[r][c] = input(&mut lines);
                }
            }

            let mut mk = vec![vec![false; 501]; 501];
            let mut level = vec![0; 100001];
            let mut lsz = 1;
            let mut max = 0;

            let mut heap = BinaryHeap::new();
            heap.push(Cell { t: l[y][x], r: y, c: x });
            mk[y][x] = true;

            let mv = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // Directions: up, right, down, left

            while let Some(Cell { t, r, c }) = heap.pop() {
                if t > max {
                    max = t;
                }
                level[lsz] = max;
                lsz += 1;

                for &(dr, dc) in &mv {
                    let r2 = r as isize + dr;
                    let c2 = c as isize + dc;

                    if r2 < 0 || r2 >= h as isize || c2 < 0 || c2 >= w as isize {
                        continue;
                    }
                    let r2 = r2 as usize;
                    let c2 = c2 as usize;

                    if mk[r2][c2] {
                        continue;
                    }
                    mk[r2][c2] = true;
                    heap.push(Cell { t: l[r2][c2], r: r2, c: c2 });
                }
            }
        }

        let mut ans = i32::MAX;
        for i in 0..=r {
            let j = r - i;
            if i < lsz && j < lsz {
                let t = level[i] + level[j];
                if t < ans {
                    ans = t;
                }
            }
        }
        results.push(ans);
    }

    for result in results {
        println!("{}", result);
    }
}
