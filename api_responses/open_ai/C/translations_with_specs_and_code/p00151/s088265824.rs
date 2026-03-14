use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: usize = match line {
            Ok(line) => line.trim().parse().unwrap(),
            Err(_) => break,
        };

        if n == 0 {
            break;
        }

        let mut grid = vec![String::new(); n];
        for i in 0..n {
            grid[i] = reader.lines().next().unwrap().unwrap();
        }

        let mut max_ones = 0;

        // Vertical
        for i in 0..n {
            let mut ones = 0;
            for j in 0..n {
                if grid[j].chars().nth(i).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        // Horizontal
        for j in 0..n {
            let mut ones = 0;
            for i in 0..n {
                if grid[i].chars().nth(j).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        // Diagonal (top-left to bottom-right)
        for j in 0..n {
            let mut ones = 0;
            for i in 0..=j {
                if i < n && (j - i) < n && grid[i].chars().nth(j - i).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        for i in 1..n {
            let mut ones = 0;
            for k in i..n {
                if k < n && (n - 1 - (k - i)) < n && grid[k].chars().nth(n - 1 - (k - i)).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        // Diagonal (top-right to bottom-left)
        for j in 0..n {
            let mut ones = 0;
            for i in 0..=j {
                if i < n && (j + i) < n && grid[i].chars().nth(j + i).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        for i in 1..n {
            let mut ones = 0;
            for k in i..n {
                if k < n && (k + i) < n && grid[k].chars().nth(i).unwrap() == '1' {
                    ones += 1;
                } else {
                    max_ones = max_ones.max(ones);
                    ones = 0;
                }
            }
            max_ones = max_ones.max(ones);
        }

        println!("{}", max_ones);
    }
}
