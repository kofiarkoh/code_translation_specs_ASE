use std::io::{self, BufRead};

const MAX_SIZE: usize = 128;

fn dfs(i: usize, j: usize, h: usize, w: usize, str: &Vec<Vec<char>>, area_id: &mut Vec<Vec<usize>>) {
    // Check down
    if i + 1 < h {
        if str[i][j] == str[i + 1][j] && area_id[i + 1][j] == (i + 1) * w + j {
            area_id[i + 1][j] = area_id[i][j];
            dfs(i + 1, j, h, w, str, area_id);
        }
    }

    // Check up
    if i > 0 {
        if str[i][j] == str[i - 1][j] && area_id[i - 1][j] == (i - 1) * w + j {
            area_id[i - 1][j] = area_id[i][j];
            dfs(i - 1, j, h, w, str, area_id);
        }
    }

    // Check right
    if j + 1 < w {
        if str[i][j] == str[i][j + 1] && area_id[i][j + 1] == i * w + (j + 1) {
            area_id[i][j + 1] = area_id[i][j];
            dfs(i, j + 1, h, w, str, area_id);
        }
    }

    // Check left
    if j > 0 {
        if str[i][j] == str[i][j - 1] && area_id[i][j - 1] == i * w + (j - 1) {
            area_id[i][j - 1] = area_id[i][j];
            dfs(i, j - 1, h, w, str, area_id);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut area_num = vec![0; MAX_SIZE * MAX_SIZE];

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut dimensions = input.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        if dimensions.len() != 2 {
            continue; // Skip invalid input
        }

        let h = dimensions[0];
        let w = dimensions[1];

        if h == 0 && w == 0 {
            break;
        }

        let mut str = vec![vec![' '; MAX_SIZE]; MAX_SIZE];
        let mut area_id = vec![vec![0; MAX_SIZE]; MAX_SIZE];

        for i in 0..h {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            str[i] = line.trim().chars().collect();
            for j in 0..w {
                area_id[i][j] = i * w + j;
            }
        }

        area_num.fill(0); // Reset area_num

        for i in 0..h {
            for j in 0..w {
                dfs(i, j, h, w, &str, &mut area_id);
            }
        }

        let mut cnt = 0;

        for i in 0..h {
            for j in 0..w {
                area_num[area_id[i][j]] = 1;
            }
        }

        for i in 0..h {
            for j in 0..w {
                if area_num[i * w + j] == 1 {
                    cnt += 1;
                }
            }
        }

        println!("{}", cnt);
    }
}
