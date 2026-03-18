use std::io::{self, Read};

const N: usize = 9;

fn in_bounds(r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && (r as usize) < N && (c as usize) < N
}

fn explode(grid: &mut [[char; N]; N], sr: i32, sc: i32) {
    // If out of bounds, do nothing.
    if !in_bounds(sr, sc) {
        return;
    }

    // Flood-fill from the start cell, turning all 4-connected '1' cells into '0'.
    // Coordinates are interpreted as 1-based in the input (common in such problems).
    let r0 = sr - 1;
    let c0 = sc - 1;
    if !in_bounds(r0, c0) {
        return;
    }
    if grid[r0 as usize][c0 as usize] != '1' {
        return;
    }

    let mut stack = vec![(r0, c0)];
    while let Some((r, c)) = stack.pop() {
        if !in_bounds(r, c) {
            continue;
        }
        let (ru, cu) = (r as usize, c as usize);
        if grid[ru][cu] != '1' {
            continue;
        }
        grid[ru][cu] = '0';
        stack.push((r - 1, c));
        stack.push((r + 1, c));
        stack.push((r, c - 1));
        stack.push((r, c + 1));
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap_or(0),
        None => return,
    };

    for case_idx in 0..t {
        let mut grid = [['0'; N]; N];

        for r in 0..N {
            let row = it.next().unwrap_or("");
            let mut chars = row.chars();
            for c in 0..N {
                grid[r][c] = chars.next().unwrap_or('0');
            }
        }

        let sr: i32 = it.next().and_then(|x| x.parse().ok()).unwrap_or(-1);
        let sc: i32 = it.next().and_then(|x| x.parse().ok()).unwrap_or(-1);

        explode(&mut grid, sr, sc);

        println!("Data {}:", case_idx + 1);
        for r in 0..N {
            let line: String = grid[r].iter().collect();
            println!("{line}");
        }
    }
}
