use std::io::{self, BufRead};

const NUM: usize = 1000;

fn check(x: usize, y: usize, a: &[[char; NUM]; NUM], n: usize) -> i32 {
    if a[x][y] == a[n - 1 - x][y] && a[x][y] == a[x][n - 1 - y] {
        1 // true
    } else {
        0 // false
    }
}

fn update(x: usize, y: usize, a: &mut [[char; NUM]; NUM], t: &mut [[i32; NUM]; NUM], total: &mut i32, n: usize) {
    *total -= t[x][y];
    t[x][y] = check(x, y, a, n);
    *total += t[x][y];
}

fn change(x: usize, y: usize, a: &mut [[char; NUM]; NUM], t: &mut [[i32; NUM]; NUM], total: &mut i32, n: usize) {
    if a[x][y] == '0' {
        a[x][y] = '1';
    } else {
        a[x][y] = '0';
    }
    update(x, y, a, t, total, n);
    update(n - 1 - x, y, a, t, total, n);
    update(x, n - 1 - y, a, t, total, n);
}

fn solve(total: i32, n: usize) -> i32 {
    if total == (n * n) as i32 {
        1 // true
    } else {
        0 // false
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let c: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse count");
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse size");

    let mut a: [[char; NUM]; NUM] = [['0'; NUM]; NUM];
    let mut t: [[i32; NUM]; NUM] = [[0; NUM]; NUM];
    let mut total: i32 = 0;

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for j in 0..n {
            a[i][j] = line.chars().nth(j).unwrap();
        }
    }

    for i in 0..n {
        for j in 0..n {
            t[i][j] = check(i, j, &a, n);
            total += t[i][j];
        }
    }

    let mut ans = solve(total, n);

    for _ in 0..c - 1 {
        let k: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse number of changes");

        for _ in 0..k {
            let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse::<usize>().expect("Failed to parse coordinates"))
                .collect();
            if coords.len() < 2 {
                continue; // Skip if not enough coordinates
            }
            let (x, y) = (coords[0] - 1, coords[1] - 1); // Decrement for 0-based index
            if x < n && y < n { // Ensure coordinates are within bounds
                change(x, y, &mut a, &mut t, &mut total, n);
            }
        }

        ans += solve(total, n);
    }

    println!("{}", ans);
}
