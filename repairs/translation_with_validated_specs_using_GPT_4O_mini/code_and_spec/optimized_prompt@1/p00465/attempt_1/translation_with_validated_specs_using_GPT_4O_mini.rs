use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    t: i32,
    r: usize,
    c: usize,
}

// Implementing Ord for Cell to use it in a BinaryHeap
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
    
    let r: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..r {
        let (w, h, x, y): (usize, usize, usize, usize) = {
            let line = lines.next().unwrap();
            let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap() - 1, iter.next().unwrap() - 1)
        };

        let mut grid = vec![vec![0; w]; h];
        for i in 0..h {
            let line = lines.next().unwrap();
            let values: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            grid[i].copy_from_slice(&values);
        }

        let mut visited = vec![vec![false; w]; h];
        let mut levels = vec![0; 100001];
        let mut lsz = vec![1, 1];
        let mut max = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Cell { t: grid[y][x], r: y, c: x });
        visited[y][x] = true;

        while let Some(Cell { t, r, c }) = heap.pop() {
            if t > max {
                max = t;
            }
            levels[lsz[0]] = max;
            lsz[0] += 1;

            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (r2, c2) = (r as isize + dr, c as isize + dc);
                if r2 < 0 || r2 >= h as isize || c2 < 0 || c2 >= w as isize {
                    continue;
                }
                let (r2, c2) = (r2 as usize, c2 as usize);
                if visited[r2][c2] {
                    continue;
                }
                visited[r2][c2] = true;
                heap.push(Cell { t: grid[r2][c2], r: r2, c: c2 });
            }
        }

        visited = vec![vec![false; w]; h];
        max = 0;
        heap.clear();
        heap.push(Cell { t: grid[y][x], r: y, c: x });
        visited[y][x] = true;

        while let Some(Cell { t, r, c }) = heap.pop() {
            if t > max {
                max = t;
            }
            levels[lsz[1]] = max;
            lsz[1] += 1;

            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (r2, c2) = (r as isize + dr, c as isize + dc);
                if r2 < 0 || r2 >= h as isize || c2 < 0 || c2 >= w as isize {
                    continue;
                }
                let (r2, c2) = (r2 as usize, c2 as usize);
                if visited[r2][c2] {
                    continue;
                }
                visited[r2][c2] = true;
                heap.push(Cell { t: grid[r2][c2], r: r2, c: c2 });
            }
        }

        let mut ans = i32::MAX;
        for i in 0..=r {
            let j = r - i;
            if i < lsz[0] && j < lsz[1] {
                let t = levels[i] + levels[j];
                if t < ans {
                    ans = t;
                }
            }
        }
        println!("{}", ans);
    }
}
