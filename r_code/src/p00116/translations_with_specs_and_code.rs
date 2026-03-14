use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if dimensions.len() != 2 {
            break; // Exit if we don't have exactly two dimensions
        }

        let (h, w) = (dimensions[0], dimensions[1]);
        if h == 0 || w == 0 {
            break; // Exit if height or width is zero
        }

        let mut mat = vec![vec![0; w]; h];
        let mut ans = 0;

        for i in 0..h {
            let mut k = 0;
            let mut line = String::new();
            reader.read_line(&mut line).unwrap(); // Read the grid line

            for j in 0..w {
                let c = line.chars().nth(j).unwrap();
                if c == '.' {
                    k += 1;
                } else {
                    k = 0;
                }
                mat[i][j] = k;
            }
        }

        for j in (0..w).rev() {
            for k in (0..h).rev() {
                let mut min = mat[k][j];
                for i in 1..=h - 1 - k {
                    if min > mat[i + k][j] {
                        min = mat[i + k][j];
                    }
                    if min == 0 {
                        break;
                    }
                    if min * (i + 1) > ans {
                        ans = min * (i + 1);
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
