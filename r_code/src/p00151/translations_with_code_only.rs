use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut grid = [[0u8; 256]; 255]; // Using u8 to represent characters
    let mut max_ones: usize;

    for line in reader.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n {
            let input_line = reader.lines().next().unwrap().unwrap();
            grid[i][..input_line.len()].copy_from_slice(input_line.as_bytes());
        }

        max_ones = 0;

        // Vertical
        for i in 0..n {
            let mut ones = 0;
            for j in 0..n {
                if grid[i][j] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Horizontal
        for j in 0..n {
            let mut ones = 0;
            for i in 0..n {
                if grid[i][j] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Diagonal (top-left to bottom-right)
        for j in 0..n {
            let mut ones = 0;
            for i in 0..=j {
                if i < n && j - i < n && grid[i][j - i] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Diagonal (bottom-left to top-right)
        for i in 1..n {
            let mut ones = 0;
            for k in i..n {
                let j = n - 1 - (k - i);
                if k < n && j >= 0 && grid[k][i] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Diagonal (top-right to bottom-left)
        for j in 0..n {
            let mut ones = 0;
            for i in 0..n {
                if i < n && j + i < n && grid[i][j + i] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        // Diagonal (bottom-right to top-left)
        for i in 1..n {
            let mut ones = 0;
            for k in i..n {
                let j = k - i;
                if k < n && j < n && grid[k][i] == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
        }

        println!("{}", max_ones);
    }
}
