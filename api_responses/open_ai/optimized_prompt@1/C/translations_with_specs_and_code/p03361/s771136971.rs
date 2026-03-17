use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read height and width
    let mut dimensions = String::new();
    reader.read_line(&mut dimensions).unwrap();
    let mut iter = dimensions.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![String::new(); h];
    for i in 0..h {
        reader.read_line(&mut grid[i]).unwrap();
    }

    // Check adjacency of '#' characters
    for i in 0..h {
        for j in 0..w {
            if grid[i].chars().nth(j) == Some('#') {
                let up = if i > 0 { grid[i - 1].chars().nth(j) } else { None };
                let down = if i < h - 1 { grid[i + 1].chars().nth(j) } else { None };
                let left = if j > 0 { grid[i].chars().nth(j - 1) } else { None };
                let right = if j < w - 1 { grid[i].chars().nth(j + 1) } else { None };

                if up != Some('#') && down != Some('#') && left != Some('#') && right != Some('#') {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
