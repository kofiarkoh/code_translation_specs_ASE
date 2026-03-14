use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    while input.read_line(&mut buffer).unwrap() > 0 {
        let mut dimensions = buffer.split_whitespace();
        let h: usize = dimensions.next().unwrap().parse().unwrap();
        let w: usize = dimensions.next().unwrap().parse().unwrap();

        if h * w == 0 {
            break;
        }

        let mut mat = vec![vec![0; 700]; 700];

        for i in 0..h {
            buffer.clear();
            input.read_line(&mut buffer).unwrap(); // Read the newline
            let chars: Vec<char> = buffer.chars().collect();
            let mut k = 0;

            for j in 0..w {
                if chars[j] == '.' {
                    k += 1;
                } else {
                    k = 0;
                }
                mat[i][j] = k;
            }
        }

        let mut ans = 0;

        for j in (0..w).rev() {
            for k in (0..h).rev() {
                let mut min = mat[k][j];
                for i in 1..(h - k) {
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
        buffer.clear(); // Clear buffer for the next iteration
    }
}
