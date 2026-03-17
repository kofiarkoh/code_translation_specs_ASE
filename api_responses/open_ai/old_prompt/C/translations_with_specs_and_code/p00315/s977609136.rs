use std::io::{self, BufRead};

const NUM: usize = 1000;

fn check(x: usize, y: usize, grid: &Vec<Vec<char>>, n: usize) -> i32 {
    if grid[x][y] == grid[n - 1 - x][y] && grid[x][y] == grid[x][n - 1 - y] {
        1 // Symmetry condition satisfied
    } else {
        0 // Symmetry condition not satisfied
    }
}

fn update(x: usize, y: usize, grid: &mut Vec<Vec<char>>, t: &mut Vec<Vec<i32>>, total: &mut i32, n: usize) {
    *total -= t[x][y]; // Decrease total count
    t[x][y] = check(x, y, grid, n); // Update symmetry check
    *total += t[x][y]; // Increase total count
}

fn change(x: usize, y: usize, grid: &mut Vec<Vec<char>>, t: &mut Vec<Vec<i32>>, total: &mut i32, n: usize) {
    if grid[x][y] == '0' {
        grid[x][y] = '1';
    } else {
        grid[x][y] = '0';
    }
    update(x, y, grid, t, total, n); // Update symmetry for (x, y)
    update(n - 1 - x, y, grid, t, total, n); // Update symmetry for (N-1-x, y)
    update(x, n - 1 - y, grid, t, total, n); // Update symmetry for (x, N-1-y)
}

fn solve(total: i32, n: usize) -> i32 {
    if total == (n * n) as i32 {
        1 // All symmetry conditions satisfied
    } else {
        0 // Not all symmetry conditions satisfied
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let c: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['0'; NUM]; NUM];
    let mut t: Vec<Vec<i32>> = vec![vec![0; NUM]; NUM];
    let mut total = 0;

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for j in 0..n {
            grid[i][j] = line.chars().nth(j).unwrap();
        }
    }

    for i in 0..n {
        for j in 0..n {
            t[i][j] = check(i, j, &grid, n);
            total += t[i][j];
        }
    }

    let mut ans = solve(total, n); // Initial symmetry check

    for _ in 0..c - 1 {
        let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

        for _ in 0..k {
            let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse::<usize>().unwrap() - 1) // Convert to 0-based index
                .collect();
            let (x, y) = (coords[0], coords[1]);
            change(x, y, &mut grid, &mut t, &mut total, n); // Toggle and update
        }

        ans += solve(total, n); // Check symmetry after updates
    }

    println!("{}", ans); // Output the final count of symmetry conditions
}
