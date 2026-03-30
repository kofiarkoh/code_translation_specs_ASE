use std::io::{self, BufRead};

const NUM: usize = 1000;

fn check(grid: &Vec<Vec<char>>, n: usize, x: usize, y: usize) -> i32 {
    if grid[x][y] == grid[n - 1 - x][y] && grid[x][y] == grid[x][n - 1 - y] {
        1 // symmetric
    } else {
        0 // not symmetric
    }
}

fn update(grid: &mut Vec<Vec<char>>, t: &mut Vec<Vec<i32>>, total: &mut i32, n: usize, x: usize, y: usize) {
    *total -= t[x][y]; // decrement total
    t[x][y] = check(grid, n, x, y); // update t
    *total += t[x][y]; // increment total
}

fn change(grid: &mut Vec<Vec<char>>, t: &mut Vec<Vec<i32>>, total: &mut i32, n: usize, x: usize, y: usize) {
    if grid[x][y] == '0' {
        grid[x][y] = '1';
    } else {
        grid[x][y] = '0';
    }
    update(grid, t, total, n, x, y); // update for (x, y)
    update(grid, t, total, n, n - 1 - x, y); // update for (N-1-x, y)
    update(grid, t, total, n, x, n - 1 - y); // update for (x, N-1-y)
}

fn solve(total: i32, n: usize) -> i32 {
    if total == (n * n) as i32 {
        1 // all cells are symmetric
    } else {
        0 // not all cells are symmetric
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
    let mut total: i32 = 0;

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for j in 0..n {
            grid[i][j] = line.chars().nth(j).unwrap();
        }
    }

    for i in 0..n {
        for j in 0..n {
            t[i][j] = check(&grid, n, i, j);
            total += t[i][j];
        }
    }

    let mut ans = solve(total, n); // initial symmetry check

    for _ in 0..c - 1 {
        let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

        for _ in 0..k {
            let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse::<usize>().unwrap() - 1) // convert to 0-based index
                .collect();
            let (x, y) = (coords[0], coords[1]);
            change(&mut grid, &mut t, &mut total, n, x, y);
        }

        ans += solve(total, n); // check symmetry after updates
    }

    println!("{}", ans);
}
